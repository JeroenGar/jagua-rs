use ordered_float::NotNan;

use jagua_rs::geometry::geo_traits::Shape;
use jagua_rs::geometry::primitives::SPolygon;

const X_MULTIPLIER: f32 = 10.0;

/// The loss LBF assigned to a placing option.
/// Weighted sum of the x_max and y_max of the shape, with the horizontal dimension being more important.
/// <br>
/// A pure lexicographic comparison (always prioritizing x-axis) would lead to undesirable results due to the continuous nature of the values.
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Eq, Ord)]
pub struct LBFLoss(NotNan<f32>);

impl LBFLoss {
    pub fn new(x_max: f32, y_max: f32) -> Self {
        let cost = x_max * X_MULTIPLIER + y_max;
        LBFLoss(NotNan::new(cost).expect("cost is NaN"))
    }

    pub fn from_shape(shape: &SPolygon) -> Self {
        LBFLoss::new(shape.bbox().x_max, shape.bbox().y_max)
    }
}
