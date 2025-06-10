use crate::geometry::Transformation;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_traits::{
    CollidesWith, DistanceTo, SeparationDistance, Transformable, TransformableFrom,
};
use crate::geometry::primitives::{Point, Rect, SPolygon};
use anyhow::{Result, bail, ensure};
use crate::geometry::fail_fast::SPSurrogate;

#[derive(Clone, Debug)]
/// A non-simple polygon, which consists of an outer simple polygon and a set of inner simple polygons (holes).
pub struct NSPolygon {
    pub outer: SPolygon,
    pub inner: Vec<SPolygon>,
    pub surrogate: Option<SPSurrogate>
}

//TODO: Surrogate is different?

impl NSPolygon {
    pub fn new(outer: SPolygon, inner: Vec<SPolygon>) -> Result<Self> {
        todo!("make sure there are inner polygons");
        todo!("make sure the inner polygons are inside the outer polygon");
    }
    
    pub fn centroid(&self) -> Point {
        todo!("find out how to calculate centroid for polygons with holes");
    }
    
    pub fn bbox(&self) -> Rect {
        self.outer.bbox
    }
    
    pub fn diameter(&self) -> f32 {
        self.outer.diameter
    }
    
    pub fn area(&self) -> f32 {
        let mut area = self.outer.area;
        for inner in &self.inner {
            area -= inner.area;
        }
        area
    }
}

impl Transformable for NSPolygon {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        todo!()
    }
}

impl TransformableFrom for NSPolygon {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        todo!()
    }
}

impl CollidesWith<Point> for NSPolygon {
    fn collides_with(&self, point: &Point) -> bool {
        todo!()
    }
}

impl DistanceTo<Point> for NSPolygon {
    fn distance_to(&self, point: &Point) -> f32 {
        todo!()
    }

    fn sq_distance_to(&self, point: &Point) -> f32 {
        todo!()
    }
}

impl SeparationDistance<Point> for NSPolygon {
    fn separation_distance(&self, other: &Point) -> (GeoPosition, f32) {
        todo!()
    }

    fn sq_separation_distance(&self, other: &Point) -> (GeoPosition, f32) {
        todo!()
    }
}
