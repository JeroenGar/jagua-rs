use std::any::Any;

use crate::entities::instances::instance::Instance;
use crate::entities::layout::{LayKey, Layout};
use crate::entities::placed_item::PItemKey;
use crate::entities::placing_option::PlacingOption;
use crate::entities::solution::Solution;
use crate::fsize;

/// A `Problem` represents a problem instance in a modifiable state.
/// It can insert or remove items, create a snapshot from the current state called a `Solution`,
/// and restore its state to a previous `Solution`.
/// This trait defines shared functionality of all problem variants.
pub trait Problem : Any {
    /// Places an item into the problem instance according to the given `PlacingOption`.
    /// Returns the index of the layout where the item was placed.
    fn place_item(&mut self, p_opt: PlacingOption) -> (LayKey, PItemKey);

    /// Removes a placed item (with its unique key) from a specific `Layout`.
    /// Returns a `PlacingOption` that can be used to place the item back in the same configuration.
    /// For more information about `commit_instantly`, see [`crate::collision_detection::cd_engine::CDEngine::deregister_hazard`].
    fn remove_item(
        &mut self,
        lkey: LayKey,
        pik: PItemKey,
        commit_instantly: bool,
    ) -> PlacingOption;

    /// Saves the current state of the problem as a `Solution`.
    fn create_solution(&mut self) -> Solution;

    /// Restores the state of the problem to a previous `Solution`.
    fn restore_to_solution(&mut self, solution: &Solution);

    /// The quantity of each item that is requested but currently missing in the problem instance, indexed by item id.
    fn missing_item_qtys(&self) -> &[isize];

    /// The quantity of each item that is currently placed in the problem instance, indexed by item id.
    fn placed_item_qtys(&self) -> impl Iterator<Item = usize> {
        self.missing_item_qtys()
            .iter()
            .enumerate()
            .map(|(i, missing_qty)| (self.instance().item_qty(i) as isize - missing_qty) as usize)
    }

    fn usage(&mut self) -> fsize {
        let (total_bin_area, total_used_area) =
            self.layouts().fold((0.0, 0.0), |acc, (_, l)| {
                let bin_area = l.bin.area;
                let used_area = bin_area * l.usage();
                (acc.0 + bin_area, acc.1 + used_area)
            });
        total_used_area / total_bin_area
    }

    fn used_bin_cost(&self) -> u64 {
        self.layouts().map(|(_,l)| l.bin.value).sum()
    }

    fn layout(&self, key: LayKey) -> &Layout;

    fn layouts(&self) -> impl Iterator<Item = (LayKey, &'_ Layout)>;

    fn layouts_mut(&mut self) -> impl Iterator<Item = (LayKey, &'_ mut Layout)>;

    fn bin_qtys(&self) -> &[usize];

    /// Makes sure that the all collision detection engines are completely updated with the changes made to the layouts.
    fn flush_changes(&mut self) {
        self.layouts_mut()
            .for_each(|(_, l)| l.flush_changes());
    }

    fn instance(&self) -> &dyn Instance;
}