## Rotations and reflections

Every item requires an `orientation` object. Angles in JSON are in **degrees**.

```json
"orientation": {
  "rotation": {"mode": "stepped", "step": 180},
  "reflection_axes": [0]
}
```

### Rotations

| Rotation | Meaning |
| --- | --- |
| `{"mode":"discrete","angles":[0]}` | Fixed at zero |
| `{"mode":"discrete","angles":[15,195]}` | Only those angles; zero is not implicit |
| `{"mode":"stepped","step":90}` | 0°, 90°, 180°, 270° |
| `{"mode":"continuous"}` | Any angle |

Discrete lists must be nonempty and contain finite angles. Equivalent angles
modulo 360° are treated alike. A stepped rotation starts at zero; its positive,
finite step must divide 360°, allowing for floating-point rounding.
See [`ExtRotation`](crate::io::ext_repr::ExtRotation) for exact validation rules.

### Reflections

Omit `reflection_axes` or use `[]` to disable reflection. Otherwise, each placement
may use **no reflection or one listed axis**, followed by an allowed rotation
and then translation. Listed axes are alternatives, not successive reflections.

Axes pass through the **original item's local origin**, measured counterclockwise
from its positive X-axis:

| Axis angle | Effect before rotation and translation |
| --- | --- |
| 0° (horizontal X-axis) | `(x, y)` becomes `(x, -y)` |
| 90° (vertical Y-axis) | `(x, y)` becomes `(-x, y)` |
| 45° | `(x, y)` becomes `(y, x)` |

Axis angles must be finite; axes differing by 180° are equivalent. With continuous
rotation, any nonempty axis list permits every mirrored orientation.

### Placement output

A placement's `reflected` flag means reflection across the original local
**X-axis**, followed by `rotation` and `translation`. Missing `reflected` means
false. An input reflection about axis `a`, followed by rotation `r`, is expressed
as `reflected: true` with rotation `r + 2*a`, modulo 360°.

For example, reflection about the Y-axis followed by translation by `(20, 5)` is:

```json
{"reflected": true, "rotation": 180, "translation": [20, 5]}
```

This maps `(x, y)` to `(20-x, 5+y)`. The 180° expresses the Y-axis reflection;
it does not grant an additional rotation permission.

### Example: fixed rotation with optional Y-axis reflection

```json
"orientation": {
  "rotation": {"mode": "discrete", "angles": [0]},
  "reflection_axes": [90]
}
```

This permits the original shape and its left-right mirror. Both preserve the
direction of an upward arrow. The complete input is in `assets/reflection.json`.
The illustration shows both shapes before translation.
