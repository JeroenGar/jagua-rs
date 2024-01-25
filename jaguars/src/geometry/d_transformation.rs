use ordered_float::NotNan;

use crate::geometry::transformation::Transformation;

//A decomposed Transformation represents a rotation followed by a translation
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DTransformation {
    rotation: NotNan<f64>,
    translation: (NotNan<f64>, NotNan<f64>),
}

impl DTransformation {
    pub fn new(rotation: f64, translation: (f64, f64)) -> Self {
        Self {
            rotation: NotNan::new(rotation).expect("rotation is NaN"),
            translation: (
                NotNan::new(translation.0).expect("translation.0 is NaN"),
                NotNan::new(translation.1).expect("translation.1 is NaN"),
            ),
        }
    }

    pub fn empty() -> Self {
        const _0: NotNan<f64> = unsafe { NotNan::new_unchecked(0.0) };
        Self {
            rotation: _0,
            translation: (_0, _0),
        }
    }

    pub fn rotation(&self) -> f64 {
        self.rotation.into()
    }

    pub fn translation(&self) -> (f64, f64) {
        (self.translation.0.into(), self.translation.1.into())
    }

    pub fn compose(&self) -> Transformation {
        Transformation::from_rotation(self.rotation()).translate(self.translation())
    }
}

impl From<Transformation> for DTransformation {
    fn from(t: Transformation) -> Self {
        t.decompose()
    }
}

impl From<&Transformation> for DTransformation {
    fn from(t: &Transformation) -> Self {
        t.decompose()
    }
}