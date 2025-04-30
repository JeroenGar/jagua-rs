use crate::entities::{BPInstance, BPSolution};
use jagua_rs_base::io::export::export_json_placed_items;
use jagua_rs_base::io::json_solution::{JsonContainer, JsonLayout, JsonLayoutStats, JsonSolution};
use std::time::Instant;

/// Exports [`BPSolution`] by composing a [`JsonSolution`] from it.
pub fn export_bpp_solution(
    solution: &BPSolution,
    instance: &BPInstance,
    epoch: Instant,
) -> JsonSolution {
    let layouts = solution
        .layout_snapshots
        .iter()
        .map(|(_, sl)| JsonLayout {
            container: JsonContainer::Bin { index: sl.bin.id },
            placed_items: export_json_placed_items(sl.placed_items.values(), instance),
            statistics: JsonLayoutStats {
                density: sl.density(instance),
            },
        })
        .collect_vec();

    JsonSolution {
        layouts,
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}
