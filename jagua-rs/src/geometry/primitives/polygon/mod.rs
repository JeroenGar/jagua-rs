use crate::geometry::fail_fast::SPSurrogateConfig;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_traits::{
    CollidesWith, DistanceTo, SeparationDistance, Transformable, TransformableFrom,
};
use crate::geometry::primitives::{NSPolygon, Point, Rect, SPolygon};
use crate::geometry::Transformation;

use anyhow::{Result};

pub mod nonsimple_polygon;

pub mod simple_polygon;

#[derive(Debug, Clone)]
pub enum Polygon {
    Simple(SPolygon),
    NonSimple(NSPolygon),
}

impl Polygon {
    
    pub fn centroid(&self) -> Point {
        match self {
            Polygon::Simple(sp) => sp.centroid(),
            Polygon::NonSimple(nsp) => nsp.centroid(),
        }
    }
    
    pub fn bbox(&self) -> Rect{
        match self {
            Polygon::Simple(sp) => sp.bbox(),
            Polygon::NonSimple(nsp) => nsp.bbox(),
        }
    }
    
    pub fn diameter(&self) -> f32 {
        match self {
            Polygon::Simple(sp) => sp.diameter(),
            Polygon::NonSimple(nsp) => nsp.diameter(),
        }
    }
    
    pub fn area(&self) -> f32 {
        match self {
            Polygon::Simple(sp) => sp.area(),
            Polygon::NonSimple(nsp) => nsp.area(),
        }
    }
    
    pub fn generate_surrogate(&mut self, surr_config: SPSurrogateConfig) -> Result<()> {
        match self {
            Polygon::Simple(sp) => {
                sp.generate_surrogate(surr_config)
            }
            Polygon::NonSimple(nsp) => {
                nsp.generate_surrogate(surr_config)
            }
        }
    }
}

impl Transformable for Polygon {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        match self {
            Polygon::Simple(sp) => {
                sp.transform(t);
            }
            Polygon::NonSimple(nsp) => {
                nsp.transform(t);
            }
        };
        self
    }
}

impl TransformableFrom for Polygon {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        match (self, reference) {
            (Polygon::Simple(sp), Polygon::Simple(ref_sp)) => {
                sp.transform_from(ref_sp, t);
            }
            (Polygon::NonSimple(nsp), Polygon::NonSimple(ref_nsp)) => {
                nsp.transform_from(ref_nsp, t);
            }
            _ => {
                unreachable!("Cannot transform from different polygon types");
            }
        };
        self
    }
}

impl CollidesWith<Point> for Polygon {
    fn collides_with(&self, point: &Point) -> bool {
        match self {
            Polygon::Simple(sp) => sp.collides_with(point),
            Polygon::NonSimple(nsp) => nsp.collides_with(point),
        }
    }
}

impl DistanceTo<Point> for Polygon {
    fn distance_to(&self, point: &Point) -> f32 {
        match self {
            Polygon::Simple(sp) => sp.distance_to(point),
            Polygon::NonSimple(nsp) => nsp.distance_to(point),
        }
    }

    fn sq_distance_to(&self, point: &Point) -> f32 {
        match self {
            Polygon::Simple(sp) => sp.sq_distance_to(point),
            Polygon::NonSimple(nsp) => nsp.sq_distance_to(point),
        }
    }
}

impl SeparationDistance<Point> for Polygon {
    fn separation_distance(&self, other: &Point) -> (GeoPosition, f32) {
        match self {
            Polygon::Simple(sp) => sp.separation_distance(other),
            Polygon::NonSimple(nsp) => nsp.separation_distance(other),
        }
    }

    fn sq_separation_distance(&self, other: &Point) -> (GeoPosition, f32) {
        match self {
            Polygon::Simple(sp) => sp.sq_separation_distance(other),
            Polygon::NonSimple(nsp) => nsp.sq_separation_distance(other),
        }
    }
}
