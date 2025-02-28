use std::cmp::Ordering;
use std::fmt::{Debug, Display};

use crate::fsize;

///Wrapper around the [`float_cmp::approx_eq!()`] macro for easy comparison of floats with a certain tolerance.
///Two FPAs are considered equal if they are within a certain tolerance of each other.
#[derive(Debug, Clone, Copy)]
pub struct FPA(pub fsize);

impl<T> From<T> for FPA
where
    T: Into<fsize>,
{
    fn from(n: T) -> Self {
        FPA(n.into())
    }
}

impl PartialEq<Self> for FPA {
    fn eq(&self, other: &Self) -> bool {
        float_cmp::approx_eq!(fsize, self.0, other.0)
    }
}

impl PartialOrd<Self> for FPA {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.eq(other) {
            true => Some(Ordering::Equal),
            false => self.0.partial_cmp(&other.0),
        }
    }
}

impl Display for FPA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
