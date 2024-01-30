use mimalloc::MiMalloc;

pub mod collision_detection;
pub mod entities;
pub mod geometry;
pub mod util;
pub mod simplification;

pub mod parse;

pub const N_QUALITIES: usize = 10;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;