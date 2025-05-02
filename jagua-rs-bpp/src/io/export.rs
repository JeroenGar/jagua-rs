use crate::entities::{BPInstance, BPSolution};
use crate::io::ext_repr::ExtBPSolution;
use jagua_rs_base::io::export::export_layout_snapshot;
use std::time::Instant;

/// Exports a solution out of the library
pub fn export(solution: &BPSolution, instance: &BPInstance, epoch: Instant) -> ExtBPSolution {
    ExtBPSolution {
        cost: solution.cost(instance),
        layouts: solution
            .layout_snapshots
            .values()
            .map(|sl| export_layout_snapshot(sl, instance))
            .collect(),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
        density: solution.density(instance),
    }
}
