use crate::entities::{SPInstance, SPSolution};
use crate::io::ext_repr::ExtSPSolution;
use jagua_rs_base::io::export::export_layout_snapshot;
use std::time::Instant;

/// Exports a solution out of the library
pub fn export(instance: &SPInstance, solution: &SPSolution, epoch: Instant) -> ExtSPSolution {
    ExtSPSolution {
        strip_width: solution.strip.width,
        layout: export_layout_snapshot(&solution.layout_snapshot, instance),
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}
