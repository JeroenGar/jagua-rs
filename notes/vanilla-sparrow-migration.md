# Vanilla sparrow migration to jagua-rs 1.0

Scope: vanilla sparrow only, starting from its jagua-rs compatibility PR #164.
This records pending consumer changes; it does not enable reflection solving.

## Read-only state access

Replace field reads on Layout, LayoutSnapshot, Container, InferiorQualityZone,
Item, PlacedItem, CDEngine and SPolygon with the corresponding accessors.
Owned data is borrowed; small values are copied. Remove redundant `&` where an
accessor already returns a reference. To clone collision geometry into a mutable
buffer, use `item.shape_cd().as_ref().clone()`.

Use `polygon.remove_surrogate()` instead of assigning `polygon.surrogate = None`.
Use layout placement/removal/container-swap methods for mutation. Snapshots are
read-only and must be produced through save operations.

## Hazard lookup

In `src/eval/sep_evaluator.rs`, retain the existing placed-item-to-hazard-key
lookup and replace direct hazard-map indexing:

```rust
let entity = layout.cde()
    .hazard(current_haz_key)
    .expect("placed item should be registered in the CDE")
    .entity;
```

`CDEngine::hazard` is an inline wrapper around `SlotMap::get`. It performs one
key lookup, without allocation, scanning, cloning or reference-count changes.
The caller's `expect` preserves the old invalid-key panic behavior. No end-to-end
performance delta has been measured for the consumer migration.

## Preserve existing behavior

Rename `Layout::is_feasible()` calls to `Layout::is_collision_free()`.
The behavior is unchanged: this checks collisions, not demand or orientation
permissions. There is no compatibility alias for the old name.

- Keep inserting the moving item's hazard into the collision collector to
  exclude self-collision. Keep the corresponding ignored-entry count.
- Keep read access to surrogate poles and convex-hull area for overlap losses,
  SIMD buffers and search ordering.
- Keep the public `layout_qt_matches_fresh_qt` diagnostic. Do not access the
  hidden quadtree representation.
- Keep vanilla sparrow rotation-only; read unreflected orientation permissions.

## Verification when applying

Compile all consumer targets and features, run the existing integration tests,
and check release eval/s if changing the evaluator's hot path beyond this lookup.
