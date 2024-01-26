use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;

pub fn convert_to_simple_polygon(_outer: &SimplePolygon, _inners: &Vec<SimplePolygon>) -> SimplePolygon {
    todo!("Implement this")
}

pub fn center_around_centroid(shape: &SimplePolygon) -> (SimplePolygon, Transformation) {
    let Point(c_x, c_y) = shape.centroid();
    let transformation = Transformation::from_translation((-c_x, -c_y));

    let centered_shape = shape.transform_clone(&transformation);

    (centered_shape, transformation)
}