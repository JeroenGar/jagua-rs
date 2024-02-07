use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::Normal;

use jaguars::entities::item::Item;
use jaguars::geometry::d_transformation::DTransformation;
use jaguars::geometry::transformation::Transformation;
use crate::samplers::rotation_distr::NormalRotDistr;

pub struct LSSampler {
    normal_x: Normal<f64>,
    normal_y: Normal<f64>,
    normal_r: NormalRotDistr,
    stddev_transl: f64,
    stddev_rot: f64,
}


impl LSSampler {
    pub fn new(item: &Item, ref_transform: &DTransformation, stddev_transl: f64, stddev_rot: f64) -> Self {
        let normal_x = Normal::new(ref_transform.translation().0, stddev_transl).unwrap();
        let normal_y = Normal::new(ref_transform.translation().1, stddev_transl).unwrap();
        let normal_r = NormalRotDistr::from_item(item, ref_transform.rotation(), stddev_rot);

        Self { normal_x, normal_y, normal_r, stddev_transl, stddev_rot }
    }

    pub fn set_mean(&mut self, ref_transform: &DTransformation) {
        self.normal_x = Normal::new(ref_transform.translation().0, self.stddev_transl).unwrap();
        self.normal_y = Normal::new(ref_transform.translation().1, self.stddev_transl).unwrap();
        self.normal_r.set_mean(ref_transform.rotation());
    }

    pub fn set_stddev(&mut self, stddev_transl: f64, stddev_rot: f64) {
        assert!(stddev_transl >= 0.0 && stddev_rot >= 0.0);

        self.stddev_transl = stddev_transl;
        self.stddev_rot = stddev_rot;
        self.normal_x = Normal::new(self.normal_x.mean(), self.stddev_transl).unwrap();
        self.normal_y = Normal::new(self.normal_y.mean(), self.stddev_transl).unwrap();
        self.normal_r.set_stddev(self.stddev_rot);
    }

    pub fn stddev_transl(&self) -> f64 {
        self.stddev_transl
    }
    pub fn stddev_rot(&self) -> f64 {
        self.stddev_rot
    }

    pub fn sample(&self, rng: &mut impl Rng) -> Transformation {
        Transformation::from_rotation(self.normal_r.sample(rng))
            .translate((self.normal_x.sample(rng), self.normal_y.sample(rng)))
    }
}