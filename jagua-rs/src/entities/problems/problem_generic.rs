use std::borrow::Borrow;

use crate::entities::layout::Layout;
use crate::entities::placed_item::PlacedItemUID;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::problem_generic::private::ProblemGenericPrivate;
use crate::entities::solution::Solution;

/// Trait for public shared functionality of all problem variants.
pub trait ProblemGeneric: ProblemGenericPrivate {
    /// Places an item into the problem instance according to the given `PlacingOption`.
    /// Returns the index of the layout where the item was placed.
    fn place_item(&mut self, i_opt: &PlacingOption) -> LayoutIndex;

    /// Removes an item with a specific `PlacedItemUID` from a specific `Layout`
    /// For more information about `commit_instantly`, see [`crate::collision_detection::cd_engine::CDEngine::deregister_hazard`].
    fn remove_item(&mut self, layout_index: LayoutIndex, pi_uid: &PlacedItemUID, commit_instantly: bool);

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

    /// The quantity of each item that is requested but currently missing in the problem instance.
    /// Indexed by item id.
    fn missing_item_qtys(&self) -> &[isize];

    fn usage(&mut self) -> f64 {
        let (total_bin_area, total_used_area) = self.layouts_mut().iter_mut().fold((0.0, 0.0), |acc, l| {
            let bin_area = l.bin().area;
            let used_area = bin_area * l.usage();
            (acc.0 + bin_area, acc.1 + used_area)
        });
        total_used_area / total_bin_area
    }

    fn used_bin_value(&self) -> u64 {
        self.layouts().iter().map(|l| l.bin().value).sum()
    }

    fn included_item_qtys(&self) -> Vec<usize>;

    fn layout_indices(&self) -> impl Iterator<Item=LayoutIndex> {
        (0..self.layouts().len()).into_iter().map(|i| LayoutIndex::Real(i))
    }

    fn template_layout_indices_with_stock(&self) -> impl Iterator<Item=LayoutIndex> {
        self.template_layouts().iter().enumerate().filter_map(|(i, l)| {
            match self.bin_qtys()[l.bin().id] {
                0 => None,
                _ => Some(LayoutIndex::Template(i))
            }
        })
    }

    fn get_layout(&self, index: impl Borrow<LayoutIndex>) -> &Layout {
        match index.borrow() {
            LayoutIndex::Real(i) => &self.layouts()[*i],
            LayoutIndex::Template(i) => &self.template_layouts()[*i]
        }
    }

    fn min_usage_layout_index(&mut self) -> Option<usize> {
        (0..self.layouts().len())
            .into_iter()
            .min_by(|&i, &j|
                self.layouts_mut()[i].usage()
                    .partial_cmp(
                        &self.layouts_mut()[j].usage()
                    ).unwrap()
            )
    }

    fn bin_qtys(&self) -> &[usize];

    fn flush_changes(&mut self) {
        self.layouts_mut().iter_mut().for_each(|l| l.flush_changes());
    }
}

pub(super) mod private {
    /// Trait for shared functionality of all problem variants, but not exposed to the public.
    pub trait ProblemGenericPrivate: Clone {
        fn next_solution_id(&mut self) -> usize;

        fn missing_item_qtys_mut(&mut self) -> &mut [isize];

        fn register_included_item(&mut self, item_id: usize) {
            self.missing_item_qtys_mut()[item_id] -= 1;
        }

        fn unregister_included_item(&mut self, item_id: usize) {
            self.missing_item_qtys_mut()[item_id] += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// Unique index for a `Layout` in a problem instance.
pub enum LayoutIndex {
    Real(usize),
    Template(usize),
}