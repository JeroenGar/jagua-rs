use std::borrow::Borrow;
use std::f32::consts::PI;
use std::fmt::Display;

use crate::geometry::Transformation;
use ordered_float::NotNan;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
/// A rigid transformation: optional reflection across the x-axis, then rotation, then translation.
/// The reflection axis is in the transformation's input coordinate system.
pub struct DTransformation {
    /// Reflect across the x-axis before rotating (negate the local y coordinate).
    pub reflected: bool,
    /// The rotation in radians
    pub rotation: NotNan<f32>,
    /// The translation in the x and y-axis
    pub translation: (NotNan<f32>, NotNan<f32>),
}

impl DTransformation {
    #[must_use]
    pub fn new(rotation: f32, translation: (f32, f32)) -> Self {
        Self {
            reflected: false,
            rotation: NotNan::new(rotation).expect("rotation is NaN"),
            translation: (
                NotNan::new(translation.0).expect("translation.0 is NaN"),
                NotNan::new(translation.1).expect("translation.1 is NaN"),
            ),
        }
    }

    #[must_use]
    pub const fn empty() -> Self {
        const _0: NotNan<f32> = unsafe { NotNan::new_unchecked(0.0) };
        Self {
            reflected: false,
            rotation: _0,
            translation: (_0, _0),
        }
    }

    /// Sets the canonical x-axis reflection applied before rotation and translation.
    #[must_use]
    pub fn with_reflection(mut self, reflected: bool) -> Self {
        self.reflected = reflected;
        self
    }

    #[must_use]
    pub fn rotation(&self) -> f32 {
        self.rotation.into()
    }

    #[must_use]
    pub fn translation(&self) -> (f32, f32) {
        (self.translation.0.into(), self.translation.1.into())
    }

    #[must_use]
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
            "reflected: {}, r: {:.3}°, t: ({:.3}, {:.3})",
            self.reflected,
            self.rotation.to_degrees(),
            self.translation.0.into_inner(),
            self.translation.1.into_inner()
        )
    }
}

/// Normalizes a rotation angle to the range [0, 2π).
#[must_use]
pub fn normalize_rotation(r: f32) -> f32 {
    // rem_euclid can round a tiny negative remainder up to the modulus.
    r.rem_euclid(2.0 * PI) % (2.0 * PI)
}
