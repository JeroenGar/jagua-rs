use std::time::Instant;

use itertools::Itertools;

use crate::entities::instance::Instance;
use crate::entities::instance::InstanceGeneric;
use crate::entities::layout::LayoutSnapshot;
use crate::geometry::geo_traits::Shape;

/// Represents a snapshot of a `Problem` at a specific moment.
/// Solutions can be used to restore the state of a `Problem` to a previous state.
#[derive(Debug, Clone)]
pub struct Solution {
    /// Unique identifier for the solution
    pub id: usize,
    /// Snapshots of all `Layout`s in the `Problem` at the moment the solution was created
    pub layout_snapshots: Vec<LayoutSnapshot>,
    /// Average usage of bins in the solution
    pub usage: f64,
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
    pub fn new(id: usize, layout_snapshots: Vec<LayoutSnapshot>, usage: f64, placed_item_qtys: Vec<usize>, target_item_qtys: Vec<usize>, bin_qtys: Vec<usize>) -> Self {
        Solution {
            id,
            layout_snapshots,
            usage,
            placed_item_qtys,
            target_item_qtys,
            bin_qtys,
            time_stamp: Instant::now(),
        }
    }

    pub fn is_complete(&self, instance: &Instance) -> bool {
        self.placed_item_qtys.iter().enumerate().all(|(i, &qty)| qty >= instance.item_qty(i))
    }

    pub fn completeness(&self, instance: &Instance) -> f64 {
        //ratio of included item area vs total instance item area
        let total_item_area = instance.item_area();
        let included_item_area = self.placed_item_qtys.iter().enumerate()
            .map(|(i, qty)| instance.item(i).shape().area() * *qty as f64)
            .sum::<f64>();
        let completeness = included_item_area / total_item_area;
        completeness
    }

    pub fn missing_item_qtys(&self, instance: &Instance) -> Vec<isize> {
        debug_assert!(instance.items().len() == self.placed_item_qtys.len());
        self.placed_item_qtys.iter().enumerate()
            .map(|(i, &qty)| instance.item_qty(i) as isize - qty as isize)
            .collect_vec()
    }

    //TODO: clean this up properly
    pub fn is_best_possible(&self, instance: &Instance) -> bool {
        match &instance {
            Instance::SP(_) => false,
            Instance::BP(bp_instance) => {
                match self.layout_snapshots.len() {
                    0 => panic!("No stored layouts in solution"),
                    1 => {
                        let bins = &bp_instance.bins;
                        let cheapest_bin = &bins.iter().min_by(|(b1, _), (b2, _)| b1.value().cmp(&b2.value())).unwrap().0;
                        self.layout_snapshots[0].bin.id() == cheapest_bin.id()
                    }
                    _ => false
                }
            }
        }
    }

    pub fn n_items_placed(&self) -> usize {
        self.placed_item_qtys.iter().sum()
    }
}
