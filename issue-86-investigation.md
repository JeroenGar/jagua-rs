# Issue 86 investigation

The direct-quadrant-check candidate was tested in a remote scratch checkout.
See [M1 Pro benchmark results](issue-86-benchmarks.md) for the correctness
checks and `ci_bench` comparison. The local classifier remains unchanged.

## Confirmed: bisector classifier can produce a false negative

The f64 assertion introduced in PR #87 still fails for this edge:

```text
node: [0, 6.1875] to [100, 106.1875]
edge: (-1.9965185, 54.71038) to (6.0424886, 60.658016)
classifier: [false, true, false, false]
f64 oracle: [false, true, true, false]
```

The exact f32 endpoint values define a segment that crosses x = 0 at
y = 56.187498867985276, below the y = 56.1875 bisector by about
0.000001132014726. The exact rational difference is
20011715 / 17677963493376. One f32 ULP at the bisector is
0.000003814697265625.

The omitted lower-left quadrant is a true intersection. Unlike the earlier
regression test, this is not a conservative f32 false positive. The edge is
approximately 10 units long.

A hardcoded test now constructs two complete squares from this edge. The
registered square is [0, 46.1875] to [10, 56.1875]. The query square is built
by rotating and translating an ordinary 10 by 10 square through the library's
transformation API. Its first edge matches the coordinates above. Both shapes
are inside the root rectangle
[-100, 6.1875] to [100, 206.1875].

With quadtree depth 0 the collision is detected. With depth 3:

- The optimized debug test fails in the f64 assertion in `qt_traits.rs`.
- The release test fails because the quadtree returns no collision.

Both hazards are created and registered through the ordinary constructors.
No partial-hazard state is fabricated. An additional standalone check through
`CDEngine::detect_poly_collision` reaches the debug assertion in current code.
The same standalone polygon check against the published 0.7.2 crate fails at
the original `qt_traits.rs:87` assertion in debug and returns a missed
collision in release.

Two rounded calculations explain the omission. Against the node's left side,
the classifier computes u = 0.5000000596046448, placing the intersection in
the upper half. Against the horizontal bisector, it computes
u = 1.0000001192092896 and rejects the intersection as outside the segment.
The real edge crosses the lower-left quadrant in the narrow space between
those two intersections.

This establishes a collision-detection failure, not a demonstrated invalid
output from a complete solver run. The overlap is smaller than one f32 ULP.

## Remaining investigation: empty hazard constriction

The reported assertion is in `QTHazard::constrict`:

```rust
debug_assert!(constricted_hazards.iter().filter(|h| h.is_some()).count() > 0);
```

Registration calls `Rect::collides_with` directly. It does not call the fast
bisector classifier. A reproduction must therefore establish how a partial
hazard reaches a node without any edge intersecting its children. Manually
constructing an inconsistent partial hazard does not establish this.

Checks so far, without a reproduction of this assertion:

- 10 million edges targeted at node corners and sub-ULP offsets, including
  y = 56.1875.
- 940,895 registrations of rotated 10 by 10 squares, in batches of up to four,
  entirely inside a 100 by 50 bin. The quadtree used the bin's inflated square
  bounds, depth 5, and a registered exterior hazard. Placements targeted
  quadtree grid corners.

The published 0.7.2 crate uses the same rectangle-edge predicate and hazard
constriction logic at these sites. The reporter has not supplied the requested
fixture as of this investigation.

## Confirmed separate trigger: registering a square outside the root

Reasoning backward from `qt_hazard` exposed a root precondition:
`QTHazard::from_root` always creates `Partial`, even if none of the shape's
edges reach the root. There is no incoming parent intersection test at the
root. A completely external square then leaves all four constricted hazards
empty and trips exactly the reported assertion.

Reproduced through `CDEngine::register_hazard` in a 100 by 50 bin, with its
normal inflated root [0, -25] to [100, 75] and a registered exterior hazard.
Register 10 by 10 squares at lower-left corners (10, 20), (30, 20), (50, 20),
then (100.00001, 20). The fourth registration fails at `qt_hazard.rs:113`.

This is a different trigger from the reporter's proposed sub-ULP bisector
mechanism. It requires a square entirely outside the root and does not prove
that a valid in-bin placement can trigger this assertion. Whether downstream
code registers such intermediate placements remains unknown. It is recorded
as an assertion precondition to investigate, rather than treated as proof of
the original numerical diagnosis.

The outside-root trigger also reproduces at the original `qt_hazard.rs:111`
site using the published 0.7.2 crate.

## Assertion hardening

The empty-constriction assertion now permits no retained edges only when an
independent f64 check finds no original polygon boundary intersecting any of
the four quadrants. It checks the original polygon, not the cached partial
edge list. The existing containment logic then resolves `Entire` or `None`.
The extra check is only reached for empty constrictions in debug builds.
Release behavior and the query classifier are unchanged.

Tests cover absent boundaries for outside and surrounding shapes. During the
investigation, a deliberately corrupted edge cache verified that missing real
intersections still trigger the assertion. That artificial-state test was
subsequently removed to keep the suite focused on three regression cases.

The separate query false-negative test is retained as an explicitly ignored
known failure so assertion hardening can be tested independently. To reproduce
it in either build mode:

```sh
cargo test -p jagua-rs rotated_square_collision_survives_quadtree_subdivision -- --ignored --nocapture
cargo test -p jagua-rs --release rotated_square_collision_survives_quadtree_subdivision -- --ignored --nocapture
```

Both commands are expected to fail until the classifier is fixed.

Validation of the hardening:

- `cargo test --workspace --all-features`: 22 passed, one known failure ignored.
- `cargo test -p jagua-rs --release --all-features`: passed, with the same
  known failure ignored.
- The standalone four-square outside-root registration now passes.
- Formatting and `git diff --check` pass.
