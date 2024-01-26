use std::cmp::Ordering;
use std::usize;

use itertools::Itertools;
use num_integer::Integer;
use ordered_float::NotNan;

use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::geo_traits::{CollidesWith, DistanceFrom, Shape, Transformable, TransformableFrom};
use crate::geometry::edge_iterator::EdgeIterator;
use crate::geometry::primitives::point::Point;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::sp_surrogate::{SPSurrogate, SPSurrogateConfig};
use crate::geometry::transformation::Transformation;
use crate::util::f64a::F64A;

//https://en.wikipedia.org/wiki/Simple_polygon

#[derive(Clone, Debug)]
pub struct SimplePolygon {
    points: Vec<Point>,
    bbox: AARectangle,
    area: f64,
    diameter: f64,
    surrogate: Option<SPSurrogate>,
}

impl SimplePolygon {
    pub fn new(mut points: Vec<Point>) -> Self {
        //Check if no two pair of consecutive points are identical
        if points.len() < 3 {
            panic!("simple poly needs at least 3 points, but was only {} points", points.len());
        }
        //assert that there are no duplicate points
        assert!(points.iter().unique().count() == points.len(), "there are duplicate points in the poly: {:?}", points);

        let mut area = SimplePolygon::calculate_area(&points);

        //edges should always be ordered counter clockwise (positive area)
        match area.partial_cmp(&0.0).unwrap() {
            Ordering::Equal => panic!("simple poly has no area {}", area),
            Ordering::Less => {
                points.reverse();
                area *= -1.0;
            }
            Ordering::Greater => (),
        }

        let diameter = SimplePolygon::calculate_diameter(&points);
        let bbox = SimplePolygon::generate_bounding_box(&points);

        let mut simple_poly = SimplePolygon {
            points,
            bbox,
            area,
            diameter,
            surrogate: None,
        };

        simple_poly.generate_surrogate(SPSurrogateConfig::default());

        simple_poly
    }

    pub fn generate_surrogate(&mut self, config: SPSurrogateConfig) {
        self.surrogate = Some(SPSurrogate::new(self, config));
    }

    pub fn get_point(&self, i: usize) -> Point {
        self.points[i]
    }

    pub fn get_edge(&self, i: usize, j: usize) -> Edge {
        debug_assert!(j == (i + 1) % self.number_of_points() || i == (j + 1) % self.number_of_points(), "i:{i} and j:{j} are not adjacent. (n:{})", self.number_of_points());
        Edge::new(self.get_point(i), self.get_point(j))
    }

    pub fn edge_iter(&self) -> EdgeIterator {
        EdgeIterator::new(self)
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn number_of_points(&self) -> usize {
        self.points.len()
    }

    pub fn surrogate(&self) -> &SPSurrogate {
        self.surrogate.as_ref().expect("surrogate should initialized during construction")
    }

    pub fn calculate_diameter(points: &[Point]) -> f64 {
        let diameter = points.iter().tuple_combinations()
            .map(|(&p1, &p2)| NotNan::new(
                Edge::new(p1.into(), p2.into()).diameter()
            ).expect("line length is NaN"))
            .max().expect("could not determine shape diameter").into();
        diameter
    }

    pub fn generate_bounding_box(points: &[Point]) -> AARectangle {
        let mut x_min = f64::MAX;
        let mut y_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_max = f64::MIN;

        for point in points.iter() {
            x_min = x_min.min(point.0);
            x_max = x_max.max(point.0);
            y_min = y_min.min(point.1);
            y_max = y_max.max(point.1);
        }
        AARectangle::new(x_min, y_min, x_max, y_max)
    }

    //https://en.wikipedia.org/wiki/Shoelace_formula
    //counter clockwise = positive area, clockwise = negative area
    pub fn calculate_area(points: &[Point]) -> f64 {
        let mut sigma: f64 = 0.0;
        for i in 0..points.len() {
            //next point
            let j = (i + 1) % points.len();

            let (x_i, y_i) = points[i].into();
            let (x_j, y_j) = points[j].into();

            sigma += (y_i + y_j) * (x_i - x_j)
        }

        0.5 * sigma
    }
}

impl Shape for SimplePolygon {
    //https://en.wikipedia.org/wiki/Centroid#Of_a_polygon
    fn centroid(&self) -> Point {
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

    fn area(&self) -> f64 {
        self.area
    }

    fn bbox(&self) -> AARectangle {
        self.bbox.clone()
    }

    fn diameter(&self) -> f64 {
        self.diameter
    }
}


impl Transformable for SimplePolygon {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        let SimplePolygon { points, bbox, area: _, diameter: _, surrogate } = self;

        //transform all points of the simple poly
        points.iter_mut().for_each(|p| { p.transform(t); });

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
        let SimplePolygon { points, bbox, area: _, diameter: _, surrogate } = self;

        for (p, ref_p) in points.iter_mut().zip(&reference.points) {
            p.transform_from(ref_p, t);
        }

        //transform the surrogate
        if let Some(surrogate) = surrogate.as_mut() {
            surrogate.transform_from(reference.surrogate(), t);
        }
        //regenerate bounding box
        *bbox = SimplePolygon::generate_bounding_box(&points);

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
                let point_outside = Point(self.bbox.x_max() + self.bbox.width(), point.1);
                let ray = Edge::new(*point, point_outside);


                let mut n_intersections = 0;
                for edge in EdgeIterator::new(self) {
                    //Check if the ray does not go through (or almost through) a vertex
                    //This can result in funky behaviour, which could incorrect results
                    //Therefore we handle this case
                    let (s_x, s_y) = (F64A(edge.start().0), F64A(edge.start().1));
                    let (e_x, e_y) = (F64A(edge.end().0), F64A(edge.end().1));
                    let (p_x, p_y) = (F64A(point.0), F64A(point.1));

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

impl DistanceFrom<Point> for SimplePolygon {
    fn sq_distance(&self, point: &Point) -> f64 {
        match self.collides_with(point) {
            true => 0.0,
            false => self.edge_iter()
                .map(|edge| edge.sq_distance(point))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        }
    }
    fn distance(&self, point: &Point) -> f64 {
        self.sq_distance(point).sqrt()
    }

    fn distance_from_border(&self, point: &Point) -> (GeoPosition, f64) {
        let (position, sq_distance) = self.sq_distance_from_border(point);
        (position, sq_distance.sqrt())
    }

    fn sq_distance_from_border(&self, point: &Point) -> (GeoPosition, f64) {
        let distance_to_border = self.edge_iter()
            .map(|edge| edge.sq_distance(point))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        match self.collides_with(point) {
            true => (GeoPosition::Interior, distance_to_border),
            false => (GeoPosition::Exterior, distance_to_border)
        }
    }
}

impl From<AARectangle> for SimplePolygon {
    fn from(r: AARectangle) -> Self {
        SimplePolygon::new(
            vec![
                (r.x_min(), r.y_min()).into(),
                (r.x_max(), r.y_min()).into(),
                (r.x_max(), r.y_max()).into(),
                (r.x_min(), r.y_max()).into(),
            ]
        )
    }
}