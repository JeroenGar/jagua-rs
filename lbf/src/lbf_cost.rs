use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use ordered_float::NotNan;
use rand_distr::num_traits::Zero;
use jaguars::geometry::geo_traits::Shape;
use jaguars::geometry::primitives::simple_polygon::SimplePolygon;


#[derive(PartialEq)]
pub struct LBFCost {
    pub x_max: NotNan<f64>,
    pub y_max: NotNan<f64>,
}

impl LBFCost {
    pub fn new(shape: &SimplePolygon) -> Self {
        let x_max = shape.bbox().x_max();
        let y_max = shape.bbox().y_max();
        Self {
            x_max: NotNan::new(x_max).unwrap(),
            y_max: NotNan::new(y_max).unwrap(),
        }
    }
    pub fn cmp(&self, other: &LBFCost) -> Ordering {
        let dx = self.x_max - other.x_max;
        let dy = self.y_max - other.y_max;

        (NotNan::new(2.0).unwrap() * dx + dy).cmp(&NotNan::zero())
    }
}

impl Display for LBFCost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.5}, {:.5})", self.x_max, self.y_max)
    }
}