use std::borrow::Borrow;
use std::f32::consts::PI;
use std::fmt::Display;

use crate::geometry::Transformation;
use ordered_float::NotNan;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
/// [Proper rigid transformation](https://en.wikipedia.org/wiki/Rigid_transformation),
/// decomposed into a rotation followed by a translation.
pub struct DTransformation {
    /// The rotation in radians
    pub rotation: NotNan<f32>,
    /// The translation in the x and y-axis
    pub translation: (NotNan<f32>, NotNan<f32>),
}

impl DTransformation {
    pub fn new(rotation: f32, translation: (f32, f32)) -> Self {
        Self {
            rotation: NotNan::new(rotation).expect("rotation is NaN"),
            translation: (
                NotNan::new(translation.0).expect("translation.0 is NaN"),
                NotNan::new(translation.1).expect("translation.1 is NaN"),
            ),
        }
    }

    pub const fn empty() -> Self {
        const _0: NotNan<f32> = unsafe { NotNan::new_unchecked(0.0) };
        Self {
            rotation: _0,
            translation: (_0, _0),
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rotation.into()
    }

    pub fn translation(&self) -> (f32, f32) {
        (self.translation.0.into(), self.translation.1.into())
    }

    pub fn compose(&self) -> Transformation {
        self.into()
    }
}

impl<T> From<T> for DTransformation
where
    T: Borrow<Transformation>,
{
    fn from(t: T) -> Self {
        t.borrow().decompose()
    }
}

impl Display for DTransformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "r: {:.3}°, t: ({:.3}, {:.3})",
            self.rotation.to_degrees(),
            self.translation.0.into_inner(),
            self.translation.1.into_inner()
        )
    }
}

/// Normalizes a rotation angle to the range [0, 2π).
pub fn normalize_rotation(r: f32) -> f32 {
    let normalized = r % (2.0 * PI);
    if normalized < 0.0 {
        normalized + 2.0 * PI
    } else {
        normalized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::F32Margin;
    use float_cmp::{FloatMargin, approx_eq};
    use std::f32::consts::PI;
    #[test]
    fn test_decomp() {
        for t1 in data() {
            assert_match(t1, t1.compose().decompose());
        }
    }

    #[test]
    fn test_double_inverse() {
        for t1 in data() {
            assert_match(t1, t1.compose().inverse().inverse().decompose());
        }
    }

    #[test]
    fn test_inverse_transform() {
        for dt1 in data() {
            let t1 = dt1.compose();
            let t1_inv = t1.clone().inverse();
            //dbg!(dt1, t1_inv.decompose());
            for dt2 in data() {
                let dt2_transf = dt2.compose().transform(&t1);
                let dt2_reverse = dt2_transf.clone().transform(&t1_inv);
                assert_match(dt2, dt2_reverse.decompose());
            }
        }
    }

    fn assert_match(dt1: DTransformation, dt2: DTransformation) {
        // Normalize angles modulo 2π for proper comparison
        let diff = (dt1.rotation() - dt2.rotation()) % (2.0 * PI);
        let angle_matches =
            diff.abs() <= (2.0 * PI * 1e-4) || (2.0 * PI - diff.abs()) <= (2.0 * PI * 1e-4);
        let x1 = dt1.translation().0;
        let x2 = dt2.translation().0;
        let y1 = dt1.translation().1;
        let y2 = dt2.translation().1;
        let x_matches = approx_eq!(f32, x1, x2, F32Margin::default().epsilon(1e-4).ulps(4));
        let y_matches = approx_eq!(f32, y1, y2, F32Margin::default().epsilon(1e-4).ulps(4));

        assert!(
            angle_matches,
            "Angles do not match: {} != {}",
            dt1.rotation(),
            dt2.rotation()
        );
        assert!(x_matches, "X translations do not match: {} != {}", x1, x2);
        assert!(y_matches, "Y translations do not match: {} != {}", y1, y2);
    }

    fn data() -> [DTransformation; 10] {
        [
            DTransformation::new(0.0, (0.0, 0.0)),
            DTransformation::new(1.0, (2.0, 3.0)),
            DTransformation::new(-1.0, (-2.0, -3.0)),
            DTransformation::new(3.14, (1.5, -1.5)),
            DTransformation::new(-3.14, (-1.5, 1.5)),
            DTransformation::new(0.0, (100.0, -100.0)),
            DTransformation::new(0.0, (-50.0, 50.0)),
            DTransformation::new(2.0, (1.0, 1.0)),
            DTransformation::new(-2.0, (-1.0, -1.0)),
            DTransformation::new(100.0, (0.0, 0.0)),
        ]
    }
}