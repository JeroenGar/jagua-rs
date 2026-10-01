use crate::Instant;
use crate::io::export::export_layout_snapshot;
use crate::probs::spp::entities::SPSolution;
use crate::probs::spp::io::ext_repr::ExtSPSolution;

/// Exports a solution out of the library
#[must_use]
pub fn export(solution: &SPSolution, epoch: Instant) -> ExtSPSolution {
    ExtSPSolution {
        strip_width: solution.strip.width,
        layout: export_layout_snapshot(&solution.layout_snapshot, 0),
        density: solution.density(),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}
