use std::time::Instant;

use once_cell::sync::Lazy;
use jagua_rs_bpp::entities::{BPInstance, BPSolution};
use jagua_rs_spp::entities::{SPInstance, SPSolution};

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
