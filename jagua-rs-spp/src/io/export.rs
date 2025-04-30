use std::time::Instant;
use jagua_rs_base::io::export::export_layout_snapshot;
use crate::entities::{SPInstance, SPSolution};
use crate::io::ext_repr::{ExtSPSolution};

pub fn export(
    instance: &SPInstance,
    solution: &SPSolution,
    epoch: Instant,
) -> ExtSPSolution {
    ExtSPSolution {
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
        layout: export_layout_snapshot(&solution.layout_snapshot, instance),
        strip_width: solution.strip.width
    }
}