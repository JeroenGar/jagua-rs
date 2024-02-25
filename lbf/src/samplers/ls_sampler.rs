use std::f64::consts::PI;

use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::Normal;

use jagua_rs::entities::item::Item;
use jagua_rs::geometry::d_transformation::DTransformation;
use jagua_rs::geometry::primitives::aa_rectangle::AARectangle;
use jagua_rs::geometry::transformation::Transformation;

use crate::samplers::rotation_distr::NormalRotDistr;

const TRANSL_START_FRAC: f64 = 0.01;
const TRANSL_END_FRAC: f64 = 0.001;
const ROT_START_FRAC: f64 = 2.0 * (PI / 180.0);
const ROT_END_FRAC: f64 = 0.5 * (PI / 180.0);

pub struct LSSampler {
    normal_x: Normal<f64>,
    normal_y: Normal<f64>,
    normal_r: NormalRotDistr,
    stddev_transl: f64,
    stddev_rot: f64,
    stddev_transl_range: (f64, f64),
    stddev_rot_range: (f64, f64),
}

impl LSSampler {
    pub fn new(
        item: &Item,
        ref_transform: &DTransformation,
        stddev_transl_range: (f64, f64),
        stddev_rot_range: (f64, f64),
    ) -> Self {
        let stddev_transl = stddev_transl_range.0;
        let stddev_rot = stddev_rot_range.0;

        let normal_x = Normal::new(ref_transform.translation().0, stddev_transl).unwrap();
        let normal_y = Normal::new(ref_transform.translation().1, stddev_transl).unwrap();
        let normal_r = NormalRotDistr::from_item(item, ref_transform.rotation(), stddev_rot);

        Self {
            normal_x,
            normal_y,
            normal_r,
            stddev_transl,
            stddev_rot,
            stddev_transl_range,
            stddev_rot_range,
        }
    }

    pub fn from_default_stddevs(
        item: &Item,
        ref_transform: &DTransformation,
        bbox: &AARectangle,
    ) -> Self {
        let max_dim = f64::max(bbox.width(), bbox.height());
        let stddev_transl_range = (max_dim * TRANSL_START_FRAC, max_dim * TRANSL_END_FRAC);
        let stddev_rot_range = (ROT_START_FRAC, ROT_END_FRAC);
        Self::new(item, ref_transform, stddev_transl_range, stddev_rot_range)
    }

    /// Shifts the mean of the normal distributions to the given reference transformation.
    pub fn shift_mean(&mut self, ref_transform: &DTransformation) {
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

    /// Adjusts the standard deviation according to the fraction of samples that have passed,
    /// following an exponential decay curve.
    /// `progress_pct` is a value in [0, 1].
    ///
    /// f(0) = init;
    /// f(1) = end;
    /// f(x) = init * (end/init)^x;
    pub fn adjust_stddev(&mut self, progress_pct: f64) {
        let calc_stddev = |(init, end): (f64, f64), pct: f64| init * (end / init).powf(pct);
        self.set_stddev(
            calc_stddev(self.stddev_transl_range, progress_pct),
            calc_stddev(self.stddev_rot_range, progress_pct),
        );
    }

    pub fn sample(&self, rng: &mut impl Rng) -> Transformation {
        Transformation::from_rotation(self.normal_r.sample(rng))
            .translate((self.normal_x.sample(rng), self.normal_y.sample(rng)))
    }
}
