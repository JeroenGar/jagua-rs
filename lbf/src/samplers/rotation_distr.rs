use std::f64::consts::PI;
use rand::distributions::Uniform;
use rand::prelude::{Distribution, SmallRng};
use rand::seq::SliceRandom;
use rand_distr::Normal;

use jaguars::entities::item::Item;
use jaguars::geometry::rotation::Rotation;

pub trait RotationSampler {
    fn sample(&self, rng: &mut SmallRng) -> f64;
}

pub enum UniformRotDistr {
    Range(Uniform<f64>),
    Discrete(Vec<f64>),
    None,
}

pub enum NormalRotDistr {
    Range(Normal<f64>),
    Discrete(f64), //normal distribution of an item with discrete orientations will always result in the same orientation being returned
    None,
}

impl UniformRotDistr {
    pub fn from_item(item: &Item) -> Self {
        match item.allowed_orientations() {
            Rotation::None  => UniformRotDistr::None,
            Rotation::Continuous => UniformRotDistr::Range(Uniform::new(0.0, 2.0 * PI)),
            Rotation::Discrete(a_o) => UniformRotDistr::Discrete(a_o.clone())

        }
    }

    pub fn sample(&self, rng: &mut SmallRng) -> f64 {
        match self {
            UniformRotDistr::None => 0.0,
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
            Rotation::None  => NormalRotDistr::None,
            Rotation::Continuous => NormalRotDistr::Range(Normal::new(r_ref, stddev).unwrap()),
            Rotation::Discrete(_) => NormalRotDistr::Discrete(r_ref)

        }
    }

    pub fn set_mean(&mut self, mean: f64) {
        match self {
            NormalRotDistr::Range(n) => {
                *n = Normal::new(mean, n.std_dev()).unwrap();
            }
            NormalRotDistr::Discrete(_) | NormalRotDistr::None => {}
        }
    }

    pub fn set_stddev(&mut self, stddev: f64) {
        match self {
            NormalRotDistr::Range(n) => {
                *n = Normal::new(n.mean(), stddev).unwrap();
            }
            NormalRotDistr::Discrete(_) | NormalRotDistr::None => {}
        }
    }

    pub fn sample(&self, rng: &mut SmallRng) -> f64 {
        match self {
            NormalRotDistr::None => 0.0,
            NormalRotDistr::Range(n) => n.sample(rng),
            NormalRotDistr::Discrete(r) => *r
        }
    }
}

