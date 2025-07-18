use std::sync::LazyLock;
use jagua_rs::Instant;

pub mod config;
pub mod io;
pub mod opt;
pub mod samplers;
pub mod util;
pub mod wasm;

pub static EPOCH: LazyLock<Instant> = LazyLock::new(Instant::now);

//limits the number of items to be placed, for debugging purposes
pub const ITEM_LIMIT: usize = usize::MAX;

pub fn init_logger() {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
}
