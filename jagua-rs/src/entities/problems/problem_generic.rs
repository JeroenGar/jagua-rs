use std::borrow::Borrow;

use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::layout::Layout;
use crate::entities::placed_item::PItemKey;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::problem_generic::private::ProblemGenericPrivate;
use crate::entities::solution::Solution;
use crate::fsize;

/// Trait for public shared functionality of all problem variants.
pub trait ProblemGeneric: ProblemGenericPrivate {
    /// Places an item into the problem instance according to the given `PlacingOption`.
    /// Returns the index of the layout where the item was placed.
    fn place_item(&mut self, p_opt: PlacingOption) -> (LayoutIndex, PItemKey);

    /// Removes an item with a specific `PIKey` from a specific `Layout`
    /// For more information about `commit_instantly`, see [`crate::collision_detection::cd_engine::CDEngine::deregister_hazard`].
    fn remove_item(&mut self, layout_index: LayoutIndex, pi_key: PItemKey, commit_instantly: bool);

    /// Saves the current state of the problem as a `Solution`.
    fn create_solution(&mut self, old_solution: &Option<Solution>) -> Solution;

    /// Restores the state of the problem to a previous `Solution`.
    fn restore_to_solution(&mut self, solution: &Solution);

    fn layouts(&self) -> &[Layout];

    fn layouts_mut(&mut self) -> &mut [Layout];

    /// Template layouts are empty and immutable.
    /// For every unique bin in the problem instance, there is a template layout.
    /// When an item is placed in a template layout, it is cloned into a real layout.
    fn template_layouts(&self) -> &[Layout];

    /// The quantity of each item that is requested but currently missing in the problem instance, indexed by item id.
    fn missing_item_qtys(&self) -> &[isize];

    /// The quantity of each item that is currently placed in the problem instance, indexed by item id.
    fn placed_item_qtys(&self) -> impl Iterator<Item=usize> {
        self.missing_item_qtys()
            .iter()
            .enumerate()
            .map(|(i, missing_qty)| (self.instance().item_qty(i) as isize - missing_qty) as usize)
    }

    fn usage(&mut self) -> fsize {
        let (total_bin_area, total_used_area) =
            self.layouts_mut().iter_mut().fold((0.0, 0.0), |acc, l| {
                let bin_area = l.bin().area;
                let used_area = bin_area * l.usage();
                (acc.0 + bin_area, acc.1 + used_area)
            });
        total_used_area / total_bin_area
    }

    fn used_bin_cost(&self) -> u64 {
        self.layouts().iter().map(|l| l.bin().value).sum()
    }

    /// Returns the `LayoutIndex` of all layouts.
    fn layout_indices(&self) -> impl Iterator<Item=LayoutIndex> {
        (0..self.layouts().len())
            .into_iter()
            .map(|i| LayoutIndex::Real(i))
    }

    /// Returns the `LayoutIndex` of all template layouts that have remaining stock.
    fn template_layout_indices_with_stock(&self) -> impl Iterator<Item=LayoutIndex> {
        self.template_layouts()
            .iter()
            .enumerate()
            .filter_map(|(i, l)| match self.bin_qtys()[l.bin().id] {
                0 => None,
                _ => Some(LayoutIndex::Template(i)),
            })
    }

    fn get_layout(&self, index: impl Borrow<LayoutIndex>) -> &Layout {
        match index.borrow() {
            LayoutIndex::Real(i) => &self.layouts()[*i],
            LayoutIndex::Template(i) => &self.template_layouts()[*i],
        }
    }

    fn bin_qtys(&self) -> &[usize];

    /// Makes sure that the all collision detection engines are completely updated with the changes made to the layouts.
    fn flush_changes(&mut self) {
        self.layouts_mut()
            .iter_mut()
            .for_each(|l| l.flush_changes());
    }

    fn instance(&self) -> &dyn InstanceGeneric;
}

pub(super) mod private {
    /// Trait for shared functionality of all problem variants, but not exposed to the public.
    pub trait ProblemGenericPrivate: Clone {
        fn next_solution_id(&mut self) -> usize;

        fn next_layout_id(&mut self) -> usize;

        fn missing_item_qtys_mut(&mut self) -> &mut [isize];

        fn register_included_item(&mut self, item_id: usize) {
            self.missing_item_qtys_mut()[item_id] -= 1;
        }

        fn deregister_included_item(&mut self, item_id: usize) {
            self.missing_item_qtys_mut()[item_id] += 1;
        }
    }
}

pub const STRIP_LAYOUT_IDX: LayoutIndex = LayoutIndex::Real(0);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Unique index for a `Layout` in a problem instance.
pub enum LayoutIndex {
    Real(usize),
    Template(usize),
}

impl Into<usize> for LayoutIndex {
    fn into(self) -> usize {
        match self {
            LayoutIndex::Real(i) | LayoutIndex::Template(i) => i,
        }
    }
}
