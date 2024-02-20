use std::borrow::Borrow;

use enum_dispatch::enum_dispatch;

use crate::entities::layout::Layout;
use crate::entities::placed_item::PlacedItemUID;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::bin_packing::BPProblem;
use crate::entities::problems::problem::private::ProblemGenericPrivate;
use crate::entities::problems::strip_packing::SPProblem;
use crate::entities::solution::Solution;

/// Enum which contains all the different problem types.
/// A `Problem` represents a problem instance in a modifiable state.
/// It can insert or remove items, create a snapshot from the current state called a `Solution`,
/// and restore its state to a previous `Solution`.
/// <br>
/// Uses the `enum_dispatch` crate to have polymorphism with static dispatch, see
/// <https://docs.rs/enum_dispatch/latest/enum_dispatch/> for more information on enum_dispatch.
/// Also enables use match statements on the `Problem` enum when variant-specific behavior is required,
/// When a new variant is added, compile errors will be generated everywhere specific behaviour is required.
#[derive(Clone)]
#[enum_dispatch]
pub enum Problem {
    /// Bin Packing Problem
    BP(BPProblem),
    /// Strip Packing Problem
    SP(SPProblem),
}

/// Trait for public shared functionality of all problem variants.
#[enum_dispatch(Problem)]
pub trait ProblemGeneric: ProblemGenericPrivate {

    /// Places an item into the problem instance according to the given `PlacingOption`.
    fn place_item(&mut self, i_opt: &PlacingOption);

    /// Removes an item with a specific `PlacedItemUID` from a specific `Layout`
    fn remove_item(&mut self, layout_index: LayoutIndex, pi_uid: &PlacedItemUID, commit_instantly: bool);

    /// Saves the current state into a `Solution`.
    fn create_solution(&mut self, old_solution: &Option<Solution>) -> Solution;

    /// Restores the state of the problem to a previous `Solution`.
    fn restore_to_solution(&mut self, solution: &Solution);

    /// Returns the layouts of the problem instance, with at least one item placed in them.
    fn layouts(&self) -> &[Layout];

    fn layouts_mut(&mut self) -> &mut [Layout];


    /// Returns layouts of the problem instance, which are currently empty (no items placed in them).
    /// These layouts can be used to place items into.
    fn empty_layouts(&self) -> &[Layout];

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

    fn empty_layout_has_stock(&self, index: usize) -> bool {
        let bin_id = self.empty_layouts()[index].bin().id;
        self.bin_qtys()[bin_id] > 0
    }

    fn get_layout(&self, index: impl Borrow<LayoutIndex>) -> &Layout {
        match index.borrow() {
            LayoutIndex::Existing(i) => &self.layouts()[*i],
            LayoutIndex::Empty(i) => &self.empty_layouts()[*i]
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
    use enum_dispatch::enum_dispatch;
    use crate::entities::problems::problem::Problem;

    /// Trait for shared functionality of all problem variants, but not exposed to the public.
    #[enum_dispatch(Problem)]
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
/// Unique identifier for a layout in a problem instance.
pub enum LayoutIndex {
    /// Existing layout (at least one item) and its index in the `Problem`'s `layouts` vector.
    Existing(usize),
    /// Empty layout (no items) and its index in the `Problem`'s `empty_layouts` vector.
    Empty(usize),
}