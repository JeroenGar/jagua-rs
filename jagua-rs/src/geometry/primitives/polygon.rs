use crate::geometry::Transformation;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_traits::{
    CollidesWith, DistanceTo, SeparationDistance, Transformable, TransformableFrom,
};
use crate::geometry::primitives::{Point, SPolygon};
use anyhow::{Result, bail, ensure};

/// Generalization of a [`SPolygon`] to a polygon that can contain one or multiple holes.
///  Defined by an outer contour and a vector of inner contours (holes).
#[derive(Clone, Debug)]
pub struct Polygon {
    pub outer: SPolygon,
    pub inner: Option<Vec<SPolygon>>,
}

//TODO: Surrogate is different?

impl Polygon {
    pub fn new(outer: SPolygon, inner: Vec<SPolygon>) -> Result<Self> {
        bail!("make sure the inner polygons are inside the outer polygon");
        let inner = Some(inner);
        Ok(Self { outer, inner })
    }
}

impl From<SPolygon> for Polygon {
    fn from(outer: SPolygon) -> Self {
        Self { outer, inner: None }
    }
}

impl Transformable for Polygon {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        todo!()
    }
}

impl TransformableFrom for Polygon {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        todo!()
    }
}

impl CollidesWith<Point> for Polygon {
    fn collides_with(&self, point: &Point) -> bool {
        todo!()
    }
}

impl DistanceTo<Point> for Polygon {
    fn distance_to(&self, point: &Point) -> f32 {
        todo!()
    }

    fn sq_distance_to(&self, point: &Point) -> f32 {
        todo!()
    }
}

impl SeparationDistance<Point> for Polygon {
    fn separation_distance(&self, other: &Point) -> (GeoPosition, f32) {
        todo!()
    }

    fn sq_separation_distance(&self, other: &Point) -> (GeoPosition, f32) {
        todo!()
    }
}
