use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Uniform;

use jagua_rs::entities::item::Item;
use jagua_rs::fsize;
use jagua_rs::geometry::d_transformation::DTransformation;
use jagua_rs::geometry::primitives::aa_rectangle::AARectangle;

use crate::samplers::rotation_distr::UniformRotDistr;

/// Samples a `DTransformation` from a uniform distribution over a given `AARectangle` and a `UniformRotDistr`.
pub struct UniformAARectSampler {
    pub bbox: AARectangle,
    pub uniform_x: Uniform<fsize>,
    pub uniform_y: Uniform<fsize>,
    pub uniform_r: UniformRotDistr,
}

impl UniformAARectSampler {
    pub fn new(bbox: AARectangle, item: &Item) -> Self {
        let uniform_x = Uniform::new(bbox.x_min, bbox.x_max).unwrap();
        let uniform_y = Uniform::new(bbox.y_min, bbox.y_max).unwrap();
        let uniform_r = UniformRotDistr::from_item(item);
        Self {
            bbox,
            uniform_x,
            uniform_y,
            uniform_r,
        }
    }

    pub fn sample(&self, rng: &mut impl Rng) -> DTransformation {
        let r_sample = self.uniform_r.sample(rng);
        let x_sample = self.uniform_x.sample(rng);
        let y_sample = self.uniform_y.sample(rng);

        DTransformation::new(r_sample, (x_sample, y_sample))
    }
}
