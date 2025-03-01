use std::borrow::Borrow;

use itertools::Itertools;
use num_integer::Integer;
use ordered_float::{NotNan, OrderedFloat};

use crate::fsize;
use crate::geometry::convex_hull::convex_hull_from_points;
use crate::geometry::fail_fast::poi;
use crate::geometry::fail_fast::sp_surrogate::SPSurrogate;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_traits::{
    CollidesWith, Distance, SeparationDistance, Shape, Transformable, TransformableFrom,
};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::point::Point;
use crate::geometry::transformation::Transformation;
use crate::util::config::SPSurrogateConfig;
use crate::util::fpa::FPA;

/// Geometric primitive representing a simple polygon: <https://en.wikipedia.org/wiki/Simple_polygon>
#[derive(Clone, Debug)]
pub struct SimplePolygon {
    /// Set of bounds describing the polygon
    pub points: Vec<Point>,
    /// Bounding box
    pub bbox: AARectangle,
    pub area: fsize,
    /// Maximum distance between any two points in the polygon
    pub diameter: fsize,
    /// Pole of inaccessibility
    pub poi: Circle,
    /// Surrogate representation (subset of the simple polygon)
    pub surrogate: Option<SPSurrogate>,
}

impl SimplePolygon {
    /// Create a new simple polygon from a set of points, expensive operations are performed here! Use [Self::clone()] or [Self::transform()] to avoid recomputation.
    pub fn new(mut points: Vec<Point>) -> Self {
        assert!(
            points.len() >= 3,
            "simple polygon must have at least 3 points"
        );
        assert_eq!(
            points.iter().unique().count(),
            points.len(),
            "simple polygon should not contain duplicate points: {:?}",
            points
        );

        let area = match SimplePolygon::calculate_area(&points) {
            area if area == 0.0 => panic!("simple polygon has no area: {:?}", points),
            area if area < 0.0 => {
                //edges should always be ordered counterclockwise (positive area)
                points.reverse();
                -area
            }
            area => area,
        };

        let diameter = SimplePolygon::calculate_diameter(points.clone());
        let bbox = SimplePolygon::generate_bounding_box(&points);
        let poi = SimplePolygon::calculate_poi(&points, diameter);

        SimplePolygon {
            points,
            bbox,
            area,
            diameter,
            poi,
            surrogate: None,
        }
    }

    pub fn generate_surrogate(&mut self, config: SPSurrogateConfig) {
        match &self.surrogate {
            Some(surrogate) if surrogate.config == config => {}
            _ => self.surrogate = Some(SPSurrogate::new(self, config)),
        }
    }

    pub fn get_point(&self, i: usize) -> Point {
        self.points[i]
    }

    pub fn get_edge(&self, i: usize) -> Edge {
        let j = (i + 1) % self.number_of_points();
        Edge::new(self.points[i], self.points[j])
    }

    pub fn edge_iter(&self) -> impl Iterator<Item = Edge> + '_ {
        (0..self.number_of_points()).map(move |i| self.get_edge(i))
    }

    pub fn number_of_points(&self) -> usize {
        self.points.len()
    }

    pub fn surrogate(&self) -> &SPSurrogate {
        self.surrogate.as_ref().expect("surrogate not generated")
    }

    pub fn calculate_diameter(points: Vec<Point>) -> fsize {
        //The two points furthest apart must be part of the convex hull
        let ch = convex_hull_from_points(points);

        //go through all pairs of points and find the pair with the largest distance
        let sq_diam = ch
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| p1.sq_distance(*p2))
            .max_by_key(|sq_d| NotNan::new(*sq_d).unwrap())
            .expect("convex hull is empty");

        sq_diam.sqrt()
    }

    pub fn generate_bounding_box(points: &[Point]) -> AARectangle {
        let (mut x_min, mut y_min) = (fsize::MAX, fsize::MAX);
        let (mut x_max, mut y_max) = (fsize::MIN, fsize::MIN);

        for point in points.iter() {
            x_min = x_min.min(point.0);
            y_min = y_min.min(point.1);
            x_max = x_max.max(point.0);
            y_max = y_max.max(point.1);
        }
        AARectangle::new(x_min, y_min, x_max, y_max)
    }

    //https://en.wikipedia.org/wiki/Shoelace_formula
    //counterclockwise = positive area, clockwise = negative area
    pub fn calculate_area(points: &[Point]) -> fsize {
        let mut sigma: fsize = 0.0;
        for i in 0..points.len() {
            //next point
            let j = (i + 1) % points.len();

            let (x_i, y_i) = points[i].into();
            let (x_j, y_j) = points[j].into();

            sigma += (y_i + y_j) * (x_i - x_j)
        }

        0.5 * sigma
    }

    pub fn calculate_poi(points: &[Point], diameter: fsize) -> Circle {
        //need to make a dummy simple polygon, because the pole generation algorithm
        //relies on many of the methods provided by the simple polygon struct
        let dummy_sp = {
            let bbox = SimplePolygon::generate_bounding_box(points);
            let area = SimplePolygon::calculate_area(points);
            let dummy_poi = Circle::new(Point(fsize::MAX, fsize::MAX), fsize::MAX);

            SimplePolygon {
                points: points.to_vec(),
                bbox,
                area,
                diameter,
                poi: dummy_poi,
                surrogate: None,
            }
        };

        poi::generate_next_pole(&dummy_sp, &[])
    }

    pub fn center_around_centroid(mut self) -> (SimplePolygon, Transformation) {
        let Point(c_x, c_y) = self.centroid();
        let transformation = Transformation::from_translation((-c_x, -c_y));

        self.transform(&transformation);

        (self, transformation)
    }
}

impl Shape for SimplePolygon {
    fn centroid(&self) -> Point {
        //based on: https://en.wikipedia.org/wiki/Centroid#Of_a_polygon

        let area = self.area();
        let mut c_x = 0.0;
        let mut c_y = 0.0;

        for i in 0..self.number_of_points() {
            let j = if i == self.number_of_points() - 1 {
                0
            } else {
                i + 1
            };
            let Point(x_i, y_i) = self.get_point(i);
            let Point(x_j, y_j) = self.get_point(j);
            c_x += (x_i + x_j) * (x_i * y_j - x_j * y_i);
            c_y += (y_i + y_j) * (x_i * y_j - x_j * y_i);
        }

        c_x /= 6.0 * area;
        c_y /= 6.0 * area;

        (c_x, c_y).into()
    }

    fn area(&self) -> fsize {
        self.area
    }

    fn bbox(&self) -> AARectangle {
        self.bbox.clone()
    }

    fn diameter(&self) -> fsize {
        self.diameter
    }
}

impl Transformable for SimplePolygon {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        //destructuring pattern to ensure that the code is updated when the struct changes
        let SimplePolygon {
            points,
            bbox,
            area: _,
            diameter: _,
            poi,
            surrogate,
        } = self;

        //transform all points of the simple poly
        points.iter_mut().for_each(|p| {
            p.transform(t);
        });

        poi.transform(t);

        //transform the surrogate
        if let Some(surrogate) = surrogate.as_mut() {
            surrogate.transform(t);
        }

        //regenerate bounding box
        *bbox = SimplePolygon::generate_bounding_box(points);

        self
    }
}

impl TransformableFrom for SimplePolygon {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        //destructuring pattern to ensure that the code is updated when the struct changes
        let SimplePolygon {
            points,
            bbox,
            area: _,
            diameter: _,
            poi,
            surrogate,
        } = self;

        for (p, ref_p) in points.iter_mut().zip(&reference.points) {
            p.transform_from(ref_p, t);
        }

        poi.transform_from(&reference.poi, t);

        //transform the surrogate
        if let Some(surrogate) = surrogate.as_mut() {
            surrogate.transform_from(reference.surrogate(), t);
        }
        //regenerate bounding box
        *bbox = SimplePolygon::generate_bounding_box(points);

        self
    }
}

impl CollidesWith<Point> for SimplePolygon {
    fn collides_with(&self, point: &Point) -> bool {
        //based on the ray casting algorithm: https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
        match self.bbox().collides_with(point) {
            false => false,
            true => {
                //horizontal ray shot to the right.
                //Starting from the point to another point that is certainly outside the shape
                let point_outside = Point(self.bbox.x_max + self.bbox.width(), point.1);
                let ray = Edge::new(*point, point_outside);

                let mut n_intersections = 0;
                for edge in self.edge_iter() {
                    //Check if the ray does not go through (or almost through) a vertex
                    //This can result in funky behaviour, which could incorrect results
                    //Therefore we handle this case
                    let (s_x, s_y) = (FPA(edge.start.0), FPA(edge.start.1));
                    let (e_x, e_y) = (FPA(edge.end.0), FPA(edge.end.1));
                    let (p_x, p_y) = (FPA(point.0), FPA(point.1));

                    if (s_y == p_y && s_x > p_x) || (e_y == p_y && e_x > p_x) {
                        //in this case, the ray passes through (or dangerously close to) a vertex
                        //We handle this case by only counting an intersection if the edge is below the ray
                        if s_y < p_y || e_y < p_y {
                            n_intersections += 1;
                        }
                    } else if ray.collides_with(&edge) {
                        n_intersections += 1;
                    }
                }

                n_intersections.is_odd()
            }
        }
    }
}

impl Distance<Point> for SimplePolygon {
    fn sq_distance(&self, point: &Point) -> fsize {
        match self.collides_with(point) {
            true => 0.0,
            false => self
                .edge_iter()
                .map(|edge| edge.sq_distance(point))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
        }
    }
    fn distance(&self, point: &Point) -> fsize {
        self.sq_distance(point).sqrt()
    }
}

impl SeparationDistance<Point> for SimplePolygon {
    fn separation_distance(&self, point: &Point) -> (GeoPosition, fsize) {
        let (position, sq_distance) = self.sq_separation_distance(point);
        (position, sq_distance.sqrt())
    }

    fn sq_separation_distance(&self, point: &Point) -> (GeoPosition, fsize) {
        let distance_to_closest_edge = self
            .edge_iter()
            .map(|edge| edge.sq_distance(point))
            .min_by_key(|sq_d| OrderedFloat(*sq_d))
            .unwrap();

        match self.collides_with(point) {
            true => (GeoPosition::Interior, distance_to_closest_edge),
            false => (GeoPosition::Exterior, distance_to_closest_edge),
        }
    }
}

impl<T> From<T> for SimplePolygon
where
    T: Borrow<AARectangle>,
{
    fn from(r: T) -> Self {
        let r = r.borrow();
        SimplePolygon::new(vec![
            (r.x_min, r.y_min).into(),
            (r.x_max, r.y_min).into(),
            (r.x_max, r.y_max).into(),
            (r.x_min, r.y_max).into(),
        ])
    }
}
