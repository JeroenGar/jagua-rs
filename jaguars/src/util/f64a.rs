use std::cmp::Ordering;

use ordered_float::NotNan;

//Wrapper around the almost crate for easy comparison with tolerance of floats
pub struct F64A(pub f64);

impl F64A {
    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn is_zero(&self) -> bool {
        almost::zero::<f64>(self.0.into())
    }
}


impl From<NotNan<f64>> for F64A {
    fn from(n: NotNan<f64>) -> Self {
        F64A(n.into())
    }
}

impl From<f64> for F64A {
    fn from(n: f64) -> Self {
        F64A(n)
    }
}

impl Eq for F64A {}

impl PartialEq<Self> for F64A {
    fn eq(&self, other: &Self) -> bool {
        almost::equal(self.0, other.0)
    }
}

impl PartialOrd<Self> for F64A {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.eq(other) {
            true => Some(Ordering::Equal),
            false => self.0.partial_cmp(&other.0)
        }
    }
}