# Library polish backlog

Review of the 1.0.0 preparation branch, 2026-09-28. These are deferred proposals,
not approved implementation work. Preserve useful consumer APIs and keep changes
separate from the orientation/reflection PR.

## Accepted direction

1. Protect coupled state in Layout, Container and the CDE. Callers must not mutate
   placements, geometry or collision state independently. Keep controlled mutation
   methods and convenient read access.
2. Protect cached geometry and placed-item state with ordinary read-only accessors.
   The selected design uses private-to-the-crate fields, borrowed access for owned
   data, and copied scalar values. No data-view wrapper or Deref implementation.

The stacked implementation covers Layout, Container, InferiorQualityZone, Item,
PlacedItem, CDEngine and SPolygon. LBF consumers are adapted in the same slice.
Items 3 and 4 are now implemented in the same stacked PR, as detailed below.

## Deferred findings

### 3. Snapshot integrity

Implemented: LayoutSnapshot fields and CDESnapshot hazards are crate-private.
LayoutSnapshot exposes read-only container and placement accessors. No public
constructor or mutable accessor allows editing captured components independently.

LayoutSnapshot exposes independently mutable placements, container and CDE
snapshot. Restrict mutation while preserving inspection so restore can trust the
saved components to agree. See entities/layout.rs and collision_detection/cd_engine.rs.

### 4. Accidental public APIs

Implemented: hide the quadtree module and remove its public CDE accessor; make
virtual-root and containment helpers private; hide polygon diameter/pole
construction helpers and degenerate-vertex cleanup; remove public surrogate
generation re-exports. Remove unused quadtree wrappers and a diagnostic helper
made dead by the visibility changes. Keep layout/CDE diagnostic functions used
by consumers public.

Consumer audit of sparrow's jg-jagua-0.9-compat branch:

- src/eval/sep_evaluator.rs indexes hazards_map after resolving a placed-item key.
  Approved: use the inline CDEngine::hazard lookup instead. See
  [vanilla sparrow migration](vanilla-sparrow-migration.md) for the pending change.
- The same evaluator seeds BasicHazardCollector with the moving item's hazard
  to exclude self-collision, then subtracts that entry from the count. This is
  supported by the collector/filter contract, but couples exclusion and results.
  Decision: keep this behavior unchanged for now.
- quantify/overlap_proxy.rs, quantify/simd/overlap_proxy_simd.rs and
  eval/collision_loss.rs read surrogate poles directly; optimizer/explore.rs and
  optimizer/lbf.rs use convex_hull_area. These are deliberate geometry inputs to
  sparrow's loss/ordering algorithms. Preserve read access rather than hide them.
- util/assertions.rs calls layout_qt_matches_fresh_qt. Keep this diagnostic entry
  point without exposing the quadtree representation.

No sparrow source usages of the newly hidden helpers or quadtree module were found.
Downstream accessor migration is still separate; this was a source-usage audit,
not a downstream compilation check.

Review public quadtree internals, get_virtual_root, polygon-construction helpers
and utility functions. Check consumers before reducing visibility: sparrow and
Pro read hazards and use jagua-rs assertions. Preserve useful diagnostics and
read access; do not hide entire modules indiscriminately.

### 5. Import errors versus panics

Implemented: invalid quality levels and unsupported quality-zone shapes return
errors from container import and InferiorQualityZone::new. The existing geometry
import integration test covers these cases and a valid quality-zone control.

Importer::import_container returns Result but asserts on out-of-range quality
levels and reaches unimplemented! for unsupported quality-zone shapes. Return
errors for invalid external input. Keep assertions for internal invariants.
See io/import.rs and entities/container.rs.

### 6. Collision and feasibility contracts

Implemented: rename Layout::is_feasible to is_collision_free, document unchecked
placement, and state that a positive surrogate collision proves a collision while
a negative result requires the full polygon query. Behavior is unchanged.

Document that Layout::place_item registers a placement without validating
collisions or orientation permissions. Layout::is_feasible checks collisions,
not demand or permitted orientations. A negative surrogate screening result
does not prove the full polygon collision-free. Keep these contracts concise.

### 7. CDE restore assumptions

Implemented: CDEngine::save, CDEngine::restore and CDESnapshot are crate-private.
Layout is their only caller; consumers restore through Layout::restore, which
checks container identity. The CDE snapshot re-export is also crate-private.

CDEngine::restore restores dynamic hazards and matches existing hazards by
entity; it does not replace the geometry of an existing entity. Decide whether
to document the required identity/static-state assumptions or restrict this
lower-level operation. Layout::restore already checks container identity.

### 8. Quality-filter discrepancy

HazKeyFilter::from_irrelevant_qzones documents ignoring zones above or at the
required quality, while its predicate selects quality < required_quality.
Verify the intended semantics and consumers before changing code or wording.
See collision_detection/hazards/filter.rs.

### 9. Library entry-point documentation

Add one runnable crate-level example showing construction/import, a collision
query, placement and snapshot/restore. Explain geometry, CDE, layouts and optional
problem definitions. Document TransformableFrom buffer compatibility: matching
vertex counts alone does not make arbitrary reference polygons interchangeable.
Avoid filler documentation for trivial accessors.

### 10. Release and CI checks

Several workflows only run for PRs targeting main, missing stacked PRs. Markdown
and SVG changes do not trigger documentation builds. Align/document toolchain
policy and declare a tested minimum Rust version. Add crate-only feature checks
to avoid workspace feature unification and package verification before release.
The package file list already includes the embedded Markdown and SVG.

### 11. Naming and release notes

Rename Importer::import_item's internal_id parameter to idx. Before final release,
replace the temporary migration document with appropriate release notes, per the
earlier review decision. Keep API documentation about current behavior.

## Outside this polishing slice

Removing SPP/BPP, splitting crates and introducing a broad typed-error hierarchy
need separate design decisions. Do not turn this audit into an algorithm rewrite
or add getters to plain input/configuration structs without an invariant to protect.
