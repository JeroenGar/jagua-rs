## Rotations and reflections

Items specify allowed rotations and optional reflection axes. The same item fields
are used for strip, bin, and multi-strip packing. Angles in JSON are **degrees**;
angles in the native geometry API are **radians**.

Every item requires an `orientation` object containing a `rotation` object:

```json
{
  "orientation": {
    "rotation": { "mode": "stepped", "step": 180 },
    "reflection_axes": [0]
  }
}
```

| Rotation mode | Example | Meaning |
| --- | --- | --- |
| `discrete` | `{"mode":"discrete","angles":[0]}` | Fixed at zero |
| `discrete` | `{"mode":"discrete","angles":[15,195]}` | Exactly those alternatives; zero is not implicit |
| `stepped` | `{"mode":"stepped","step":90}` | 0°, 90°, 180°, 270° |
| `continuous` | `{"mode":"continuous"}` | Any angle |

Discrete lists must be nonempty and finite. A stepped rotation starts at zero;
its positive, finite step must divide 360°. A step of 360° permits only zero.
Use discrete angles for offset patterns. Angular intervals are not supported.

**Step validation:** the input step is stored as `f32`. In `f64` arithmetic,
compute `count = round(360 / step)`. Require `0 < step <= 360` and
`abs(count * step - 360) <= 360 * f32::EPSILON` (about 0.00004292°).
This accepts decimal steps such as 0.1° despite their binary rounding, but rejects
7° and 90.001°. Generate angles as `i * 360 / count` for integer `i` from zero
through `count - 1`, then convert to native radians. No repeated addition or
360° endpoint is used.

Explicit angle lists, reflection-axis lists, and stepped expansions are limited
to 65,536 entries each. Lists are bounded before deduplication. The Cartesian
product of normalized, unique rotations and axes is also limited to 65,536
before reflected-angle deduplication, to bound allocation. Continuous rotation
needs no Cartesian expansion. These limits are shared with native
`AllowedOrientations::MAX_ANGLES`.

`reflection_axes` may be omitted or `[]` to disable reflection; `null` is invalid.
Missing/null `orientation` or `rotation`, unknown modes, unknown orientation fields,
and mode-inappropriate fields are rejected. The former `allowed_rotations`,
`allowed_orientations`, and `allowed_reflection_axes` fields are rejected,
including when mixed with the new object. There are no compatibility aliases.
Serialization preserves the selected mode and emits only the new representation;
an empty reflection-axis list is omitted.

### Migrating inputs and saved solutions

This is a breaking format change. Replace the old item fields in both input files
and saved outputs before using them as warm starts:

| Previous rotation value | New `orientation.rotation` |
| --- | --- |
| `null` (or historically omitted) | `{"mode":"continuous"}` |
| `[]` or `[0]` | `{"mode":"discrete","angles":[0]}` |
| `[0,180]` | `{"mode":"stepped","step":180}` |
| Other nonempty list | `{"mode":"discrete","angles":[...]}` |

Move the previous reflection-axis list into `orientation.reflection_axes` and
remove all old permission keys. Concrete placement transformations are unchanged.

Every orientation chooses **no reflection or one permitted reflection**, then
applies an allowed rotation, then a translation. The unreflected shape remains
permitted under the same rotation restriction. Multiple listed axes are
alternatives; they are not successive reflections. In particular, allowing both
X and Y reflection with fixed rotation does not also allow an unreflected 180° turn.

Reflection axes pass through the **original item's local origin**, measured
counterclockwise from its positive X-axis, before rotation:

| Axis angle | Effect before rotation and translation |
| --- | --- |
| 0° (horizontal X-axis) | `(x, y)` becomes `(x, -y)` |
| 90° (vertical Y-axis) | `(x, y)` becomes `(-x, y)` |
| 45° | `(x, y)` becomes `(y, x)` |

Axes are equivalent modulo 180° and rotations modulo 360°. Import rejects
non-finite angles, normalizes in degrees before converting to radians, and removes
duplicate canonical angles by exact equality. Nearby distinct angles are retained.
Continuous rotation with any nonempty axis list permits every mirrored orientation;
adding more axes does not create more possibilities in that case.

### Example: fixed rotation, optional Y-axis reflection

This is a complete strip-packing input, also available as `assets/reflection.json`
in the repository:

```json
{
  "name": "fixed-rotation-y-reflection",
  "strip_height": 12,
  "items": [
    {
      "id": 0,
      "demand": 4,
      "orientation": {
        "rotation": {"mode": "discrete", "angles": [0]},
        "reflection_axes": [90]
      },
      "shape": {
        "type": "simple_polygon",
        "data": [[2, 3], [8, 3], [8, 5], [5, 5], [5, 9], [2, 9]]
      }
    }
  ]
}
```

This permits the original shape and its left-right mirror, each translated freely.
Both preserve the direction of a local upward arrow, which can model directional
grain aligned with Y. X-axis reflection would reverse that arrow. The library does
not infer a material's grain; the input author specifies the permitted operations.

Run the bundled LBF example from the repository root:

```sh
cargo run --release --package lbf -- -p spp -i assets/reflection.json -s /tmp/jagua-reflection
```

### Transformation representation

`DTransformation` stores `reflected`, `rotation`, and `translation`.
Its Boolean always means **reflection across the X-axis before rotation**.
An input-axis angle `a` followed by allowed rotation `r` becomes the canonical pose
`reflected = true, rotation = r + 2*a` (modulo a full turn). Unreflected poses use `r`.

Consequently, the Y-axis mirror in the example is exported as:

```json
{
  "reflected": true,
  "rotation": 180,
  "translation": [20, 5]
}
```

This maps original coordinates to `(20-x, 5+y)`. The 180° is part of the canonical
representation of Y reflection; it does not mean that 180° was added to the item's
allowed rotations. Matrix decomposition can return equivalent negative angles,
such as -180°. A missing `reflected` defaults to false, and false is omitted when
serializing, preserving the old format for unreflected placements.

`Transformation` retains its matrix representation and derives reflection from
the determinant. Composition and inversion preserve reflections. For native use:

```rust
use std::f32::consts::{FRAC_PI_2, PI};
use jagua_rs::geometry::{AllowedOrientations, DTransformation};
use jagua_rs::geometry::geo_enums::RotationRange;
use jagua_rs::geometry::geo_traits::Transformable;
use jagua_rs::geometry::primitives::Point;

let allowed = AllowedOrientations::new(RotationRange::None, vec![FRAC_PI_2])?;
let pose = DTransformation::new(PI, (20.0, 5.0)).with_reflection(true);
assert!(allowed.allows(&pose));
let point = Point(2.0, 3.0).transform_clone(&pose.compose());
assert!((point.0 - 18.0).abs() < 0.00001);
assert!((point.1 - 8.0).abs() < 0.00001);
# Ok::<(), anyhow::Error>(())
```

`AllowedOrientations` compiles the input into a canonical rotation range for each
reflection state. `rotations(false)` returns the unreflected range;
`rotations(true)` returns the reflected range, or `None` if reflection is disabled.
A returned `RotationRange::None` means **fixed zero**, not a forbidden state.
The original axis list is not needed in the sampling loop. `allows` checks the full
orientation, using a circular angular tolerance of `4 * f32::EPSILON * TAU` radians
for matrix and wrapping roundoff. That tolerance is not used for deduplication.

The bundled LBF sampler chooses reflected or unreflected with equal probability
when reflection is enabled, then samples the corresponding canonical rotation
range. Local refinement preserves the chosen reflection and refines translation
and, when permitted, continuous rotation. Custom optimizers must likewise sample
both states and preserve `reflected` when reconstructing a pose. Collision queries
and layout placement apply supplied transforms; they do not enforce item orientation
restrictions. Use `AllowedOrientations::allows` for that check.

### Coordinates and polygon storage

The importer centers collision shapes internally. Use `ext_to_int_transformation`
and `int_to_ext_transformation` with the original shape's `pre_transform` to convert
placements. These compose the full transform, including reflection, so exported
translations apply to the original input coordinates. SVG output applies the same
order: `translate(...) rotate(...) scale(1 -1)` for a reflected placement.

`SPolygon` vertices remain physically counterclockwise with positive area after
every transform. Reflection reverses vertex storage and remaps and reverses the
cached convex-hull indices. `transform_from` writes reversed source vertices into
the existing buffer and restores the reference hull indices for unreflected poses.
It allocates no new vertex or hull buffers and does not regenerate surrogates.
Vertex indices therefore identify storage positions, not stable identities across
reflections. As before, a transform buffer must come from the same reference
geometry and surrogate configuration.

### Native API migration

Replace the external item's rotation and axis fields with `ExtItem.orientation`:

```rust
use jagua_rs::io::ext_repr::{ExtOrientation, ExtRotation};

let orientation = ExtOrientation {
    rotation: ExtRotation::Stepped { step: 180.0 },
    reflection_axes: vec![0.0],
};
```

Use `ExtRotation::Discrete { angles: vec![0.0] }` for fixed zero or
`ExtRotation::Continuous {}` for unrestricted rotation. The importer validates
these values and compiles them to the existing `AllowedOrientations` ranges.
Native sampling and concrete transformation APIs are unchanged by the tagged
input contract. In particular, samplers do not need a stepped-rotation case.
