use std::cmp::Ordering;

use almost::AlmostEqual;

///Wrapper around the almost crate for easy comparison of floats with a certain tolerance
///the almost crate considers two floats equal if they are within a certain tolerance of each other.
///see crate docs for more details
pub struct F64A(pub f64);

impl F64A {
    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn is_zero(&self) -> bool {
        almost::zero::<f64>(self.0.into())
    }
}

impl<T> From<T> for F64A
where
    T: Into<f64>,
{
    fn from(n: T) -> Self {
        F64A(n.into())
    }
}

impl PartialEq<Self> for F64A {
    fn eq(&self, other: &Self) -> bool {
        self.0.almost_equals(other.0)
    }
}

impl PartialOrd<Self> for F64A {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.eq(other) {
            true => Some(Ordering::Equal),
            false => self.0.partial_cmp(&other.0),
        }
    }
}
