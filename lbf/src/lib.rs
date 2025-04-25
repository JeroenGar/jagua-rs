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

//limits the number of items to be placed, for debugging purposes
pub const ITEM_LIMIT: usize = usize::MAX;

#[allow(clippy::large_enum_variant)]
pub enum LBFSolution {
    BP(BPSolution),
    SP(SPSolution),
}

#[allow(clippy::large_enum_variant)]
pub enum LBFInstance {
    BP(BPInstance),
    SP(SPInstance),
}
