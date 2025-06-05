use crate::io::export::export_layout_snapshot;
use crate::probs::bpp::entities::{BPInstance, BPSolution};
use crate::probs::bpp::io::ext_repr::ExtBPSolution;

/// Exports a solution out of the library
pub fn export(instance: &BPInstance, solution: &BPSolution, epoch_ms: f64) -> ExtBPSolution {
    ExtBPSolution {
        cost: solution.cost(instance),
        layouts: solution
            .layout_snapshots
            .values()
            .map(|sl| export_layout_snapshot(sl, instance))
            .collect(),
        // compute runtime in seconds as float
        run_time_sec: ((solution.time_stamp - epoch_ms) / 1000.0).max(0.0) as u64,
        density: solution.density(instance),
    }
}
