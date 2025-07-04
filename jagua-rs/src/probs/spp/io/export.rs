use crate::io::export::export_layout_snapshot;
use crate::probs::spp::entities::{SPInstance, SPSolution};
use crate::probs::spp::io::ext_repr::ExtSPSolution;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(not(target_arch = "wasm32"))]
/// Exports a solution out of the library
pub fn export(instance: &SPInstance, solution: &SPSolution, epoch: Instant) -> ExtSPSolution {
    ExtSPSolution {
        strip_width: solution.strip.width,
        layout: export_layout_snapshot(&solution.layout_snapshot, instance),
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}

#[cfg(target_arch = "wasm32")]
/// Exports a solution out of the library for Wasm
pub fn export(instance: &SPInstance, solution: &SPSolution, epoch_ms: f64) -> ExtSPSolution {
    ExtSPSolution {
        strip_width: solution.strip.width,
        layout: export_layout_snapshot(&solution.layout_snapshot, instance),
        density: solution.density(instance),
        run_time_sec: ((solution.time_stamp - epoch_ms) / 1000.0).max(0.0) as u64,
    }
}
