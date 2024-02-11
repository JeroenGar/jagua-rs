use std::cmp::Ordering;

use ordered_float::NotNan;

use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_enums::GeoPosition::{Exterior, Interior};

/// Represents the proximity to another entity.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Proximity {
    /// Whether we are inside or outside the other entity
    pub position: GeoPosition,
    /// Distance from the border of the other entity
    pub distance_from_border: NotNan<f64>,
}

impl Proximity {
    pub fn new(position: GeoPosition, distance: f64) -> Self {
        Self {
            position,
            distance_from_border: NotNan::new(distance).expect("distance was NaN"),
        }
    }
}

impl Into<f64> for &Proximity {
    fn into(self) -> f64 {
        //interior proximity is negative, exterior is positive
        match self.position {
            Interior => -self.distance_from_border.into_inner(),
            Exterior => self.distance_from_border.into_inner(),
        }
    }
}

impl Into<f64> for Proximity {
    fn into(self) -> f64 {
        (&self).into()
    }
}

impl Default for Proximity {
    fn default() -> Self {
        Self {
            position: Exterior,
            distance_from_border: NotNan::new(f64::MAX).expect("distance was NaN"),
        }
    }
}

impl PartialOrd for Proximity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_val: f64 = self.into();
        let other_val: f64 = other.into();
        self_val.partial_cmp(&other_val)
    }
}

impl Ord for Proximity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}