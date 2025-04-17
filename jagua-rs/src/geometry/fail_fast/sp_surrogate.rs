use crate::geometry::Transformation;
use crate::geometry::convex_hull;
use crate::geometry::fail_fast::{piers, pole};
use crate::geometry::geo_traits::{DistanceTo, Shape, Transformable, TransformableFrom};
use crate::geometry::primitives::Circle;
use crate::geometry::primitives::Edge;
use crate::geometry::primitives::SPolygon;
use crate::util::SPSurrogateConfig;

#[derive(Clone, Debug)]
/// Surrogate representation of a [`SPolygon`] - a 'light-weight' representation that
/// is fully contained in the original [`SPolygon`].
/// Used for *fail-fast* collision detection.
pub struct SPSurrogate {
    /// Set of [poles](pole::generate_surrogate_poles)
    pub poles: Vec<Circle>,
    /// Set of [piers](piers::generate_piers)
    pub piers: Vec<Edge>,
    /// Indices of the vertices in the [`SPolygon`] that form the convex hull
    pub convex_hull_indices: Vec<usize>,
    /// The area of the convex hull of the [`SPolygon`].
    pub convex_hull_area: f32,
    /// The configuration used to generate the surrogate
    pub config: SPSurrogateConfig,
}

impl SPSurrogate {
    /// Creates a new [`SPSurrogate`] from a [`SPolygon`] and a configuration.
    /// Expensive operations are performed here!
    pub fn new(simple_poly: &SPolygon, config: SPSurrogateConfig) -> Self {
        let convex_hull_indices = convex_hull::convex_hull_indices(simple_poly);
        let convex_hull_area = SPolygon::new(
            convex_hull_indices
                .iter()
                .map(|&i| simple_poly.vertices[i])
                .collect(),
        )
        .area();
        let mut poles = pole::generate_surrogate_poles(simple_poly, &config.n_pole_limits);
        let pole_bounding_circle = Circle::bounding_circle(&poles);
        let n_ff_poles = usize::min(config.n_ff_poles, poles.len());
        let relevant_poles_for_piers = &poles[0..n_ff_poles]; //poi + all poles that will be checked during fail fast are relevant for piers
        let piers = piers::generate_piers(simple_poly, config.n_ff_piers, relevant_poles_for_piers);

        Self {
            convex_hull_indices,
            poles,
            piers,
            convex_hull_area,
            config,
        }
    }

    pub fn ff_poles(&self) -> &[Circle] {
        &self.poles[0..self.config.n_ff_poles]
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
            piers,
            convex_hull_area: _,
            config: _,
        } = self;

        //transform poles
        poles.iter_mut().for_each(|c| {
            c.transform(t);
        });

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
            piers,
            convex_hull_area: _,
            config: _,
        } = self;

        for (pole, ref_pole) in poles.iter_mut().zip(reference.poles.iter()) {
            pole.transform_from(ref_pole, t);
        }

        for (pier, ref_pier) in piers.iter_mut().zip(reference.piers.iter()) {
            pier.transform_from(ref_pier, t);
        }

        self
    }
}
