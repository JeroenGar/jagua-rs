use crate::entities::bin_packing::{BPInstance, LayKey};
use crate::entities::general::LayoutSnapshot;
use slotmap::SecondaryMap;
use std::time::Instant;

#[cfg(doc)]
use crate::entities::bin_packing::BPProblem;
use crate::fsize;

/// Snapshot of [`BPProblem`] at a specific moment.
/// Can be used to restore [`BPProblem`] to a previous state.
#[derive(Debug, Clone)]
pub struct BPSolution {
    /// Snapshots of all `Layout`s in the `Problem` at the moment the solution was created
    pub layout_snapshots: SecondaryMap<LayKey, LayoutSnapshot>,
    /// Quantity of placed items for each `Item` in the solution
    pub placed_item_qtys: Vec<usize>,
    /// Target quantity of each `Item` in the solution
    pub target_item_qtys: Vec<usize>,
    /// Quantity of bins used for each type of bin
    pub bin_qtys: Vec<usize>,
    /// Instant the solution was created
    pub time_stamp: Instant,
}

impl BPSolution {
    pub fn density(&self, instance: &BPInstance) -> fsize {
        let total_bin_area = self
            .layout_snapshots
            .values()
            .map(|ls| ls.bin.area())
            .sum::<fsize>();

        let total_item_area = self
            .layout_snapshots
            .values()
            .map(|ls| ls.placed_item_area(instance))
            .sum::<fsize>();

        total_item_area / total_bin_area
    }
}
