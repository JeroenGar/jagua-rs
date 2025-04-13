use crate::fsize;
use crate::geometry::Transformation;
use crate::geometry::convex_hull;
use crate::geometry::fail_fast::{piers, poi};
use crate::geometry::geo_traits::{DistanceTo, Shape, Transformable, TransformableFrom};
use crate::geometry::primitives::Circle;
use crate::geometry::primitives::Edge;
use crate::geometry::primitives::SimplePolygon;
use crate::util::SPSurrogateConfig;

#[derive(Clone, Debug)]
/// Surrogate representation of a [`SimplePolygon`] for fail-fast purposes
pub struct SPSurrogate {
    /// Indices of the points in the [`SimplePolygon`] that form the convex hull
    pub convex_hull_indices: Vec<usize>,
    /// Set of poles
    pub poles: Vec<Circle>,
    /// Circle in which all poles are contained
    pub poles_bounding_circle: Circle,
    /// The maximum distance from any point in the SP to a pole
    pub max_distance_point_to_pole: fsize,
    /// Set of piers
    pub piers: Vec<Edge>,
    /// Number of poles that will be checked during fail-fast
    pub n_ff_poles: usize,
    /// The area of the convex hull of the [`SimplePolygon`].
    pub convex_hull_area: fsize,
    /// The configuration used to generate the surrogate
    pub config: SPSurrogateConfig,
}

impl SPSurrogate {
    /// Creates a new [`SPSurrogate`] from a [`SimplePolygon`] and a configuration.
    /// Expensive operations are performed here!
    pub fn new(simple_poly: &SimplePolygon, config: SPSurrogateConfig) -> Self {
        let convex_hull_indices = convex_hull::convex_hull_indices(simple_poly);
        let convex_hull_area = SimplePolygon::new(
            convex_hull_indices
                .iter()
                .map(|&i| simple_poly.points[i])
                .collect(),
        )
        .area();
        let mut poles = vec![simple_poly.poi.clone()];
        poles.extend(poi::generate_poles(simple_poly, &config.n_pole_limits));
        let poles_bounding_circle = Circle::bounding_circle(&poles);
        let max_distance_point_to_pole = simple_poly
            .points
            .iter()
            .map(|p| {
                poles
                    .iter()
                    .map(|c| c.distance_to(p))
                    .fold(fsize::MAX, |acc, d| fsize::min(acc, d))
            })
            .fold(fsize::MIN, |acc, d| fsize::max(acc, d));

        let n_ff_poles = usize::min(config.n_ff_poles, poles.len());
        let relevant_poles_for_piers = &poles[0..n_ff_poles]; //poi + all poles that will be checked during fail fast are relevant for piers
        let piers = piers::generate_piers(simple_poly, config.n_ff_piers, relevant_poles_for_piers);

        Self {
            convex_hull_indices,
            poles,
            piers,
            poles_bounding_circle,
            max_distance_point_to_pole,
            n_ff_poles,
            convex_hull_area,
            config,
        }
    }

    pub fn ff_poles(&self) -> &[Circle] {
        &self.poles[0..self.n_ff_poles]
    }

    pub fn ff_piers(&self) -> &[Edge] {
        &self.piers
    }
}

impl Transformable for SPSurrogate {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        //destructuring pattern used to ensure that the code is updated accordingly when the struct changes
        let Self {
            convex_hull_indices: _,
            poles,
            poles_bounding_circle,
            max_distance_point_to_pole: _,
            piers,
            n_ff_poles: _,
            convex_hull_area: _,
            config: _,
        } = self;

        //transform poles
        poles.iter_mut().for_each(|c| {
            c.transform(t);
        });

        poles_bounding_circle.transform(t);

        //transform piers
        piers.iter_mut().for_each(|p| {
            p.transform(t);
        });

        self
    }
}

impl TransformableFrom for SPSurrogate {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        debug_assert!(self.poles.len() == reference.poles.len());
        debug_assert!(self.piers.len() == reference.piers.len());

        //destructuring pattern used to ensure that the code is updated accordingly when the struct changes
        let Self {
            convex_hull_indices: _,
            poles,
            poles_bounding_circle,
            max_distance_point_to_pole: _,
            piers,
            n_ff_poles: _,
            convex_hull_area: _,
            config: _,
        } = self;

        for (pole, ref_pole) in poles.iter_mut().zip(reference.poles.iter()) {
            pole.transform_from(ref_pole, t);
        }

        poles_bounding_circle.transform_from(&reference.poles_bounding_circle, t);

        for (pier, ref_pier) in piers.iter_mut().zip(reference.piers.iter()) {
            pier.transform_from(ref_pier, t);
        }

        self
    }
}
