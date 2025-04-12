use std::time::Instant;

use once_cell::sync::Lazy;

pub mod io;
pub mod lbf_config;
pub mod lbf_cost;
pub mod lbf_optimizer;
pub mod samplers;
pub static EPOCH: Lazy<Instant> = Lazy::new(Instant::now);
