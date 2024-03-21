use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::Normal;

use jagua_rs::entities::item::Item;
use jagua_rs::geometry::geo_enums::AllowedRotation;
use jagua_rs::{fsize, PI};

/// Samples a rotation (radians).
pub trait RotationSampler {
    fn sample(&self, rng: &mut impl Rng) -> fsize;
}

/// Samples a rotation from a uniform distribution over a given range or a discrete set of rotations.
pub enum UniformRotDistr {
    Range(Uniform<fsize>),
    Discrete(Vec<fsize>),
    None,
}

/// Samples a rotation from a normal distribution over a given range or a discrete set of rotations.
/// In case of discrete rotations the mean is always returned.
pub enum NormalRotDistr {
    Range(Normal<fsize>),
    Discrete(fsize),
    None,
}

impl UniformRotDistr {
    pub fn from_item(item: &Item) -> Self {
        match &item.allowed_rotation {
            AllowedRotation::None => UniformRotDistr::None,
            AllowedRotation::Continuous => UniformRotDistr::Range(Uniform::new(0.0, 2.0 * PI)),
            AllowedRotation::Discrete(a_o) => UniformRotDistr::Discrete(a_o.clone()),
        }
    }

    pub fn sample(&self, rng: &mut impl Rng) -> fsize {
        match self {
            UniformRotDistr::None => 0.0,
            UniformRotDistr::Range(u) => u.sample(rng),
            UniformRotDistr::Discrete(a_o) => *a_o.choose(rng).unwrap(),
        }
    }
}

impl NormalRotDistr {
    pub fn from_item(item: &Item, r_ref: fsize, stddev: fsize) -> Self {
        match &item.allowed_rotation {
            AllowedRotation::None => NormalRotDistr::None,
            AllowedRotation::Continuous => {
                NormalRotDistr::Range(Normal::new(r_ref, stddev).unwrap())
            }
            AllowedRotation::Discrete(_) => NormalRotDistr::Discrete(r_ref),
        }
    }

    pub fn set_mean(&mut self, mean: fsize) {
        match self {
            NormalRotDistr::Range(n) => {
                *n = Normal::new(mean, n.std_dev()).unwrap();
            }
            NormalRotDistr::Discrete(_) | NormalRotDistr::None => {}
        }
    }

    pub fn set_stddev(&mut self, stddev: fsize) {
        match self {
            NormalRotDistr::Range(n) => {
                *n = Normal::new(n.mean(), stddev).unwrap();
            }
            NormalRotDistr::Discrete(_) | NormalRotDistr::None => {}
        }
    }

    pub fn sample(&self, rng: &mut impl Rng) -> fsize {
        match self {
            NormalRotDistr::None => 0.0,
            NormalRotDistr::Range(n) => n.sample(rng),
            NormalRotDistr::Discrete(r) => *r,
        }
    }
}
