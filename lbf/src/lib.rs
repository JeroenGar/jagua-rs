use std::time::Instant;
use mimalloc::MiMalloc;
use once_cell::sync::Lazy;

pub mod lbf_optimizer;
pub mod samplers;
pub mod io;
pub mod lbf_cost;
pub mod config;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc; //more efficient allocator

pub static EPOCH: Lazy<Instant> = Lazy::new(Instant::now);