use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;

use jaguars::entities::item::Item;
use jaguars::geometry::d_transformation::DTransformation;
use jaguars::geometry::primitives::aa_rectangle::AARectangle;
use crate::samplers::rotation_distr::UniformRotDistr;

pub struct UniformAARectSampler {
    pub bbox: AARectangle,
    pub uniform_r: UniformRotDistr,
}

impl UniformAARectSampler {
    pub fn new(bbox: AARectangle, item: &Item) -> Self {
        let uniform_r = UniformRotDistr::from_item(item);
        Self {
            bbox,
            uniform_r
        }
    }

    pub fn sample(&self, rng: &mut SmallRng) -> DTransformation {
        let r_sample = self.uniform_r.sample(rng);
        let x_sample = Uniform::new(self.bbox.x_min(), self.bbox.x_max()).sample(rng);
        let y_sample = Uniform::new(self.bbox.y_min(), self.bbox.y_max()).sample(rng);
        
        DTransformation::new(r_sample,(x_sample, y_sample))
    }

    pub fn sample_x_bounded(&self, rng: &mut SmallRng, x_bound: f64) -> DTransformation {
        let x_max = f64::min(self.bbox.x_max(), x_bound);
        
        let r_sample = self.uniform_r.sample(rng);
        let x_sample = Uniform::new(self.bbox.x_min(), x_max).sample(rng);
        let y_sample = Uniform::new(self.bbox.y_min(), self.bbox.y_max()).sample(rng);
        
        DTransformation::new(r_sample,(x_sample, y_sample))
    }
}