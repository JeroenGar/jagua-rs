use ordered_float::NotNan;

use jagua_rs::fsize;
use jagua_rs::geometry::geo_traits::Shape;
use jagua_rs::geometry::primitives::SimplePolygon;

const X_MULTIPLIER: fsize = 10.0;

/// The loss LBF assigned to a placing option.
/// Weighted sum of the x_max and y_max of the shape, with the horizontal dimension being more important.
/// <br>
/// A pure lexicographic comparison (always prioritizing x-axis) would lead to undesirable results due to the continuous nature of the values.
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Eq, Ord)]
pub struct LBFLoss(NotNan<fsize>);

impl LBFLoss {
    pub fn new(x_max: fsize, y_max: fsize) -> Self {
        let cost = x_max * X_MULTIPLIER + y_max;
        LBFLoss(NotNan::new(cost).expect("cost is NaN"))
    }

    pub fn from_shape(shape: &SimplePolygon) -> Self {
        LBFLoss::new(shape.bbox().x_max, shape.bbox().y_max)
    }
}
