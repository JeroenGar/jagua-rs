use crate::geometry::convex_hull;
use crate::geometry::fail_fast::{piers, poi};
use crate::geometry::geo_traits::{Transformable, TransformableFrom};
use crate::geometry::primitives::circle::Circle;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::util::config::SPSurrogateConfig;

#[derive(Clone, Debug)]
pub struct SPSurrogate {
    pub convex_hull_indices: Vec<usize>,
    pub poles: Vec<Circle>,
    pub poles_bounding_circle: Circle,
    pub piers: Vec<Edge>,
    pub n_ff_poles: usize,
}

impl SPSurrogate {
    pub fn new(simple_poly: &SimplePolygon, config: SPSurrogateConfig) -> Self {
        let convex_hull_indices = convex_hull::convex_hull_indices(simple_poly);
        let mut poles = vec![simple_poly.poi.clone()];
        poles.extend(poi::generate_additional_surrogate_poles(
            simple_poly,
            config.max_poles.saturating_sub(1),
            config.pole_coverage_goal,
        ));
        let poles_bounding_circle = Circle::bounding_circle(&poles);

        let n_ff_poles = usize::min(config.n_ff_poles, poles.len());
        let relevant_poles_for_piers = &poles[0..n_ff_poles]; //poi + all poles that will be checked during fail fast are relevant for piers
        let piers = piers::generate(simple_poly, config.n_ff_piers, relevant_poles_for_piers);

        Self {
            convex_hull_indices,
            poles,
            piers,
            poles_bounding_circle,
            n_ff_poles,
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
            piers,
            n_ff_poles: _,
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
            piers,
            n_ff_poles: _,
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
