use std::time::Instant;
use jagua_rs_base::io::export::export_json_placed_items;
use jagua_rs_base::io::json_solution::{JsonContainer, JsonLayout, JsonLayoutStats, JsonSolution};
use crate::entities::{SPInstance, SPSolution};

/// Exports [`SPSolution`] by composing a [`JsonSolution`] from it.
pub fn export(
    solution: &SPSolution,
    instance: &SPInstance,
    epoch: Instant,
) -> JsonSolution {
    let json_layout = JsonLayout {
        container: JsonContainer::Strip {
            width: solution.strip_width,
            height: instance.strip_height,
        },
        placed_items: export_json_placed_items(
            solution.layout_snapshot.placed_items.values(),
            instance,
        ),
        statistics: JsonLayoutStats {
            density: solution.density(instance),
        },
    };

    JsonSolution {
        layouts: vec![json_layout],
        density: solution.density(instance),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
    }
}