use std::time::Instant;

use itertools::Itertools;
use slotmap::SecondaryMap;
use crate::entities::instances::instance::Instance;
use crate::entities::layout::{LayKey, LayoutSnapshot};
use crate::fsize;
use crate::geometry::geo_traits::Shape;

/// Represents a snapshot of a `Problem` at a specific moment.
/// Solutions can be used to restore the state of a `Problem` to a previous state.
#[derive(Debug, Clone)]
pub struct Solution {
    /// Snapshots of all `Layout`s in the `Problem` at the moment the solution was created
    pub layout_snapshots: SecondaryMap<LayKey, LayoutSnapshot>,
    /// Average usage of bins in the solution
    pub usage: fsize,
    /// Quantity of placed items for each `Item` in the solution
    pub placed_item_qtys: Vec<usize>,
    /// Target quantity of each `Item` in the solution
    pub target_item_qtys: Vec<usize>,
    /// Quantity of bins used for each type of bin
    pub bin_qtys: Vec<usize>,
    /// Instant the solution was created
    pub time_stamp: Instant,
}

impl Solution {

    /// Whether all items demanded in the `instance` are placed
    pub fn is_complete(&self, instance: &impl Instance) -> bool {
        self.placed_item_qtys
            .iter()
            .enumerate()
            .all(|(i, &qty)| qty >= instance.item_qty(i))
    }

    /// Ratio of included item area vs total demanded item area in the instance
    pub fn completeness(&self, instance: &impl Instance) -> fsize {
        let total_item_area = instance.item_area();
        let included_item_area = self
            .placed_item_qtys
            .iter()
            .enumerate()
            .map(|(i, qty)| instance.item(i).shape.area() * *qty as fsize)
            .sum::<fsize>();
        included_item_area / total_item_area
    }

    /// Returns the quantities of the items that still need to be placed to reach a complete solution.
    pub fn missing_item_qtys(&self, instance: &impl Instance) -> Vec<isize> {
        debug_assert!(instance.items().len() == self.placed_item_qtys.len());
        self.placed_item_qtys
            .iter()
            .enumerate()
            .map(|(i, &qty)| instance.item_qty(i) as isize - qty as isize)
            .collect_vec()
    }

    pub fn n_items_placed(&self) -> usize {
        self.placed_item_qtys.iter().sum()
    }
}
