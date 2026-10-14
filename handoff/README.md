# CDE redesign: handoff from the hazard-first prototype

Written on 14 October 2026 at the end of a cloud prototyping session. Read this first. `ledger.md` next to it is the raw record of every experiment, with IDs (p1, q1–q18, Q1–Q3, p4/p5/p6) that this document refers to.

## Goal

Turn the hazard-first prototype on this branch into a proper redesign of jagua-rs's collision detection engine (CDE): the same public query API, a clean data structure, and no quadtree maintenance for movable hazards. The prototype proves the approach and its gains; its code is not production quality.

## Where things are

| What | Where |
|---|---|
| Prototype | jagua-rs `hazard-first-proto` @ 0616a23: 11 commits on main 3becd0c, touching `cd_engine.rs` and a new `hazard_index.rs` (+635/−98 lines) |
| Sparrow Pro on the prototype | sparrow-pro `quantify-proto` @ 9ce0478; `m3/exact-k8` @ 6c9cc8b (the same plus a pin to jagua-rs 0616a23); `m3/q3` @ 9ab29cd (previous stack, used as baseline) |
| Raw experiment log | `handoff/ledger.md` |
| Equivalence harness, profiling driver, bench scripts | `handoff/tools/` (see Measuring) |
| Explainers | "Hazard-First Collisions" https://claude.ai/artifact/GgXGbVLSU355dAdAtcD6qh and "Pole Overlap Signal" https://claude.ai/artifact/FG25PzjBcHkRWeBFhfAT8Q |

Sparrow Pro's own changes (the 8-lane quantification kernel, no surrogate screening against the container) are independent of the CDE redesign and stay on sparrow-pro.

## The prototype in one page

Every query, collect or detect, runs the same pipeline:

1. **Broad phase.** Hazard bounding boxes live in flat arrays (`x_min`, `y_min`, `x_max`, `y_max`, `keys`) sorted by `x_min`. A query binary-searches its x-window, the hazards with `x_min` in `[q.x_min − max_width, q.x_max]` (skipped at 64 hazards or fewer), and tests 64 boxes per step with a branch-free mask.
2. **Surrogate pass (optional).** The query's fail-fast poles (and piers) against the edges of candidate hazards. All poles share one scan with their union bounding box (`any_overlapping_each`); each pole then tests its candidates.
3. **Narrow phase.** Edge–edge tests. Query edges are culled with Cohen–Sutherland outcodes against the overlap of both bounding boxes; hazard edges by cached per-hazard edge bounding boxes (`EdgeBoxes`).
4. **Containment.** Point-in-polygon both ways when no edges cross.

The container exterior is not in the index:

- Rectangular container: a query strictly inside skips the exterior test.
- Irregular outline: its edges are grouped in y-monotone chains of at most 16 edges with a bounding box each (`EdgeChains`), used for edge culling and for point-in-exterior by ray casting over whole chains.
- A shape whose bounding box misses the container's bounding box collides with the exterior (fixed in q15).

Detection runs on the same index. The quadtree is no longer registered or updated; its code still compiles, unused.

## Results

| Measure | Result |
|---|---|
| CI bench `cde_collect_1k/4` | 3.0× on the cloud VM (3836 → 1296 µs); 2.5–2.9× on the M3 Max |
| CI bench `cde_detect_1k/4` | 1.33× on the M3 Max (108 → 82 µs) |
| Collection per evaluation inside Sparrow Pro's search (M3, samply) | swim 1.6×, gardeyn7 1.55×, baldacci1 1.19× |
| Sparrow Pro evals/s, jagua-rs prototype only | +16–35% (VM), +22% (M1, swim) |
| Sparrow Pro evals/s, everything (`m3/exact-k8`) | +54–75% (VM) |
| Solution quality | Equal so far: M1 swim race p = 0.91; baldacci1 10 seeds, no significant difference; the M3 race on `m3/exact-k8` looks similar |
| Correctness | f64-oracle harness, strip (swim, ~100k queries): no missed collisions, no false positives, plus 331 real collisions that main's quadtree collection misses (main's own detection reports them). Bins (baldacci1, 60k queries): collected sets identical to main |

## What we learned

Treat these as design constraints.

1. **Early exit dominates in Sparrow Pro.** An evaluation stops once its loss passes the best sample so far, so the search cares about time to the first hits and cheap rejection, not full collection. `cde_collect_1k` collects everything at random positions: it is directionally right but overstates gains. Add an early-exit collect bench and judge by Sparrow Pro evals/s.
2. **The quadtree's cost was repetition, not memory.** Every query edge walked the tree and rescanned the same leaves: about 146 hazard checks per query where about 15 suffice. Flat arrays and masks removed that. The data stays cache-resident: a 30 s profile had 27 last-level cache misses in total.
3. **The surrogate pass is now the larger half of collection in the search** (swim: 163 of 274 ns per evaluation), mostly `any_overlapping_each` → `EdgeBoxes::any_overlapping` and `circle_touches_edges_cached`. It pays on strips (skipping it: swim −7 to −19%) and costs on bins (baldacci1 +12–18% without it). Widening it (more fail-fast poles, piers) loses everywhere; a fail-fast ratio of 0.3 instead of 0.5 only helps baldacci1. Keep it optional per query and make it cheaper.
4. **Detection is setup-only in Sparrow Pro** (LBF construction and the placeability check); exploration never detects. Time detection on LBF construction, not on evals/s.
5. **The x-window is enough.** Scans stay at about 22 hazards per query for layouts of 160 to 1280 items, so a second axis (grid, range tree) can save at most one 64-wide mask. Query time still grows with layout size (1.4 → 2.8 µs from 160 to 1280 items), from the binary search and memory rather than the scan.
6. **Irregular exteriors need cached geometry.** Scanning all exterior vertices per query and per pole cost −35% on baldacci1; cached edge boxes plus chains turned that into +46% over main.
7. **Exploration profile of `m3/exact-k8`** (swim, instruction counts): collision collection about 55% (edge tests 15%, pole–edge tests 11%, point-in-polygon 6%, index scans and collector about 20%), quantification 21%, transforms 8%, allocation 5%.
8. **Allocations.** Each hazard's `EdgeBoxes` (four Vecs) is rebuilt by pushing on every insert: placing an item, every strip-width change (which re-registers everything), and every restore. It is deep-copied on every CDE clone (about once per 18 evaluations in a profiled swim run) and freed on every remove. Reserving capacity and sharing them behind `Arc` (`tools/alloc_edgeboxes.patch`) showed no measurable end-to-end gain on the noisy VM, but costs nothing. Do it in the redesign, or better, compute edge boxes once per shape.

## Tried and dropped

| Idea | ID | Result |
|---|---|---|
| Quadtree tweaks (branchless prefilter, layout and traversal changes) | t1–t5 | neutral or slower |
| u16 edge buffers instead of `Edge` buffers | q3 | neutral |
| Query edge boxes on the stack, pair culling by edge-box overlap | q9, q12 | −2.4%, −3.4%: under the 4% bar on the VM; worth a retry on real hardware |
| Mask building by multiply-packing flag bytes | q10 | slower |
| Pole–pole quick positive test before edge scans | q13 | slower |
| Entity and shape `Arc` stored in the index | q14 | neutral |
| Item edge chains for point-in-polygon | p5 step 5 | +2.5%, under the bar |
| Second bounding-box axis | p5 step 4 | no headroom (see 5) |
| Smaller pole limits, fail-fast pier as defaults | 44ef9d6 | cost solution quality |
| Approximate pole cull in quantification | Q3 | no end-to-end gain, changes the loss signal |

## Prototype shortcuts to replace

- The quadtree still compiles but is dead; `CDEConfig::quadtree_depth` and `cd_threshold` now mean nothing.
- `HazardIndex::remove` finds the key by linear scan, and insert/remove shift five arrays. Fine at 50–1000 hazards, but O(n) per move.
- `EdgeBoxes` as four separate Vecs, rebuilt on every registration (see 8).
- `edges_intersect_cached` zero-fills a 64-edge stack buffer per call and falls back to an all-pairs test above 64 candidate edges.
- Exterior special cases are flags on `CDEngine` (`exterior_is_rect`, `exterior_chains`); the exterior is the only user of chains.
- Static hazards (holes, exclusions, locked items) and movable items share one sorted index; nothing exploits that static hazards never move.
- `save`/`restore` diff hazards by entity with a linear search.
- `hazard_index.rs` has no tests of its own; correctness rests on the external oracle harness.

## Open design questions

These are decisions for you.

1. **Static and dynamic hazards.** One index for everything, or a structure built once per container (exterior, holes, defects, locked items) plus the flat index for movable items? Earlier you suggested a simpler static quadtree for bins.
2. **Public API.** Keep `CDEngine`'s queries exactly (`collect_poly_collisions(_until)`, `collect_surrogate_collisions(_until)`, `detect_poly_collision`, `detect_surrogate_collision`, `register_hazard`, `deregister_hazard_*`, `save`/`restore`)? What happens to the quadtree settings in `CDEConfig`?
3. **Where edge boxes and chains live.** Per hazard in the index (prototype), or computed once per `SPolygon` with its surrogate and shared by `Arc` across clones and registrations?
4. **Surrogate pass shape.** A separate pass (prototype), or pole tests interleaved with the polygon pass per candidate hazard, sharing one broad-phase scan?
5. **Benchmarks.** An early-exit collect bench in `ci_bench`.

## Suggested slices

1. Bring the equivalence harness into the repo, as an integration test or a tool next to the benches. It is the safety net for everything after.
2. Add an early-exit collect bench to `ci_bench`.
3. The hazard index (broad phase and edge boxes) as a clean module behind the collect path.
4. The detect path on the index; stop maintaining the quadtree.
5. The exterior: rectangular shortcut and chains.
6. Remove the quadtree and settle the `CDEConfig` API.
7. Sparrow Pro: bump jagua-rs, then check evals/s and solution quality.

## Measuring

Results are labelled IDENTICAL (bit-identical output), EQUIVALENT (same collisions or the same loss up to summation order, explained) or APPROXIMATE (changed values, needs quality races).

- **Correctness.** `tools/eqh` prints, for seeded queries on an LBF layout with moves, save/restore and strip-width changes, every query's collected hazards, the surrogate and polygon detect results, and f64-oracle verdicts. Run it on main and on the redesign and compare with `tools/scripts/cmp_oracle.py`: no MISSING, and every extra must be an oracle hit. `cargo run --release --bin bpp` covers bin packing with irregular containers. The paths in `tools/eqh/Cargo.toml` are relative to this directory.
- **CI bench A/B.** `tools/scripts/ab.sh` alternates base and variant runs of `ci_bench` and summarizes means and spreads. Its paths are from the cloud session; adjust them.
- **Sparrow Pro throughput.** Release build with `--features simd,only_final_svg` and `RUSTFLAGS='-C target-cpu=native'`; at least 3 seeds single-threaded in parallel for 60 s (`tools/scripts/e2e.sh` shows the runner invocation). Alternate variants seed by seed: on the VM, the same binary moved ±10% between one-minute blocks.
- **Profiling.** `tools/eqh/src/bin/prof.rs` replays `ci_bench`-style collect, detect and feasibility loops for samply or cachegrind. For valgrind, build with `-C target-cpu=x86-64-v3` (no AVX-512).
- **Quality.** Multi-seed races compared as unpaired groups (means, standard errors, permutation tests).
