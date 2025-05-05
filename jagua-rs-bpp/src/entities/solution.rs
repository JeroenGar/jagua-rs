use crate::entities::{BPInstance, LayKey};
use jagua_rs_base::entities::LayoutSnapshot;
use slotmap::SecondaryMap;
use std::time::Instant;

/// Snapshot of [`BPProblem`](crate::entities::BPProblem) at a specific moment.
/// Can be used to restore to a previous state.
#[derive(Debug, Clone)]
pub struct BPSolution {
    pub layout_snapshots: SecondaryMap<LayKey, LayoutSnapshot>,
    /// Instant the solution was created
    pub time_stamp: Instant,
}

impl BPSolution {
    pub fn density(&self, instance: &BPInstance) -> f32 {
        let total_bin_area = self
            .layout_snapshots
            .values()
            .map(|ls| ls.container.area())
            .sum::<f32>();

        let total_item_area = self
            .layout_snapshots
            .values()
            .map(|ls| ls.placed_item_area(instance))
            .sum::<f32>();

        total_item_area / total_bin_area
    }

    pub fn cost(&self, instance: &BPInstance) -> u64 {
        self.layout_snapshots
            .values()
            .map(|ls| ls.container.id)
            .map(|id| instance.bins[id].cost)
            .sum()
    }
}
