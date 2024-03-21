use ordered_float::NotNan;

use jagua_rs::fsize;
use jagua_rs::geometry::geo_traits::Shape;
use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;

const X_MULTIPLIER: fsize = 10.0;

/// The cost LBF assigned to a placing option.
/// Weighted sum of the x_max and y_max of the shape, with the horizontal dimension being more important.
/// <br>
/// A pure lexicographic comparison (always prioritizing x-axis) would lead to undesirable results due to the continuous nature of the values.
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Eq, Ord)]
pub struct LBFPlacingCost(NotNan<fsize>);

impl LBFPlacingCost {
    pub fn new(x_max: fsize, y_max: fsize) -> Self {
        let cost = x_max * X_MULTIPLIER + y_max;
        LBFPlacingCost(NotNan::new(cost).expect("cost is NaN"))
    }

    pub fn from_shape(shape: &SimplePolygon) -> Self {
        LBFPlacingCost::new(shape.bbox().x_max, shape.bbox().y_max)
    }
}
