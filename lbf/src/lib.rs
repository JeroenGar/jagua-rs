use std::time::Instant;

use once_cell::sync::Lazy;

pub mod config;
pub mod io;
pub mod opt;
pub mod samplers;

pub static EPOCH: Lazy<Instant> = Lazy::new(Instant::now);

//limits the number of items to be placed, for debugging purposes
pub const ITEM_LIMIT: usize = usize::MAX;
