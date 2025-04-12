use std::time::Instant;

use once_cell::sync::Lazy;
use jagua_rs::entities::instances::bin_packing::BPInstance;
use jagua_rs::entities::instances::strip_packing::SPInstance;
use jagua_rs::entities::solution::{BPSolution, SPSolution};

pub mod io;
pub mod lbf_config;
pub mod lbf_cost;
pub mod lbf_optimizer;
pub mod samplers;
pub static EPOCH: Lazy<Instant> = Lazy::new(Instant::now);

pub enum LBFSolution{
    BP(BPSolution),
    SP(SPSolution),
}

pub enum LBFInstance{
    BP(BPInstance),
    SP(SPInstance),
}
