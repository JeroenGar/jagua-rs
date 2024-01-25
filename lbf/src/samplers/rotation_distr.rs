use rand::distributions::Uniform;
use rand::prelude::{Distribution, SmallRng};
use rand::seq::SliceRandom;
use rand_distr::Normal;

use jaguars::entities::item::{AllowedOrients, Item};

pub trait RotationSampler {
    fn sample(&self, rng: &mut SmallRng) -> f64;
}

pub enum UniformRotDistr {
    Range(Uniform<f64>),
    Discrete(Vec<f64>),
}

pub enum NormalRotDistr {
    Range(Normal<f64>),
    Discrete(f64), //normal distribution of an item with discrete orientations will always result in the same orientation being returned
}

impl UniformRotDistr {
    pub fn from_item(item: &Item) -> Self {
        match item.allowed_orientations() {
            AllowedOrients::Range(start, end) =>
                UniformRotDistr::Range(Uniform::new(start, end)),
            AllowedOrients::Set(a_o) =>
                UniformRotDistr::Discrete(a_o.clone())
        }
    }

    pub fn sample(&self, rng: &mut SmallRng) -> f64 {
        match self {
            UniformRotDistr::Range(u) => u.sample(rng),
            UniformRotDistr::Discrete(a_o) => {
                *a_o.choose(rng).unwrap()
            }
        }
    }
}

impl NormalRotDistr {
    pub fn from_item(item: &Item, r_ref: f64, stddev: f64) -> Self {
        match item.allowed_orientations() {
            AllowedOrients::Range(_, _) => {
                assert_eq!(item.allowed_orientations(), &AllowedOrients::full_range(), "Limited range not yet implemented!");
                NormalRotDistr::Range(Normal::new(r_ref, stddev).unwrap())
            }
            AllowedOrients::Set(_) =>
                NormalRotDistr::Discrete(r_ref)
        }
    }

    pub fn set_mean(&mut self, mean: f64) {
        match self {
            NormalRotDistr::Range(n) => {
                *n = Normal::new(mean, n.std_dev()).unwrap();
            }
            NormalRotDistr::Discrete(_) => {}
        }
    }

    pub fn set_stddev(&mut self, stddev: f64) {
        match self {
            NormalRotDistr::Range(n) => {
                *n = Normal::new(n.mean(), stddev).unwrap();
            }
            NormalRotDistr::Discrete(_) => {}
        }
    }

    pub fn sample(&self, rng: &mut SmallRng) -> f64 {
        match self {
            NormalRotDistr::Range(n) => n.sample(rng),
            NormalRotDistr::Discrete(r) => *r
        }
    }
}

