use crate::Instant;
use crate::io::export::export_layout_snapshot;
use crate::probs::mspp::entities::MSPSolution;
use crate::probs::mspp::io::ext_repr::ExtMSPSolution;
use crate::probs::mspp::entities::MSPInstance;

/// Exports a solution out of the library
pub fn export(instance: &MSPInstance, solution: &MSPSolution, epoch: Instant) -> ExtMSPSolution {
    ExtMSPSolution {
        layouts: solution.layout_snapshots
            .values()
            .map(|sl| export_layout_snapshot(sl, instance))
            .collect(),
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}
