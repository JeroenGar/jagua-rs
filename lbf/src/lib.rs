use std::time::Instant;

use jagua_rs::entities::bin_packing::BPInstance;
use jagua_rs::entities::bin_packing::BPSolution;
use jagua_rs::entities::strip_packing::{SPInstance, SPSolution};
use once_cell::sync::Lazy;

pub mod config;
pub mod io;
pub mod opt;
pub mod samplers;

pub static EPOCH: Lazy<Instant> = Lazy::new(Instant::now);

pub enum LBFSolution {
    BP(BPSolution),
    SP(SPSolution),
}

pub enum LBFInstance {
    BP(BPInstance),
    SP(SPInstance),
}
