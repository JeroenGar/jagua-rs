use rand::distributions::{Distribution, Uniform};
use rand::Rng;

use jagua_rs::entities::item::Item;
use jagua_rs::geometry::d_transformation::DTransformation;
use jagua_rs::geometry::primitives::aa_rectangle::AARectangle;

use crate::samplers::rotation_distr::UniformRotDistr;

/// Samples a `DTransformation` from a uniform distribution over a given `AARectangle` and a `UniformRotDistr`.
pub struct UniformAARectSampler {
    pub bbox: AARectangle,
    pub uniform_r: UniformRotDistr,
}

impl UniformAARectSampler {
    pub fn new(bbox: AARectangle, item: &Item) -> Self {
        let uniform_r = UniformRotDistr::from_item(item);
        Self { bbox, uniform_r }
    }

    pub fn sample(&self, rng: &mut impl Rng) -> DTransformation {
        let r_sample = self.uniform_r.sample(rng);
        let x_sample = Uniform::new(self.bbox.x_min, self.bbox.x_max).sample(rng);
        let y_sample = Uniform::new(self.bbox.y_min, self.bbox.y_max).sample(rng);

        DTransformation::new(r_sample, (x_sample, y_sample))
    }
}
