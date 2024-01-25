use std::sync::Arc;

use enum_dispatch::enum_dispatch;
use itertools::Itertools;

use crate::entities::instance::Instance;
use crate::entities::layout::Layout;
use crate::entities::placed_item_uid::PlacedItemUID;
use crate::entities::problems::bp_problem::BPProblem;
use crate::entities::problems::problem::private::ProblemPrivate;
use crate::entities::problems::sp_problem::SPProblem;
use crate::entities::solution::Solution;
use crate::geometry::geo_traits::Shape;
use crate::insertion::insertion_option::InsertionOption;

#[enum_dispatch]
pub enum ProblemEnum {
    BPProblem,
    //Bin Packing Problem
    SPProblem, //Strip Packing Problem
}

#[enum_dispatch(ProblemEnum)]
pub trait Problem: ProblemPrivate {
    fn insert_item(&mut self, i_opt: &InsertionOption);

    fn remove_item(&mut self, layout_index: usize, pi_uid: &PlacedItemUID);

    fn create_solution(&mut self, old_solution: &Option<Solution>) -> Solution;

    fn restore_to_solution(&mut self, solution: &Solution);

    fn instance(&self) -> &Arc<Instance>;

    fn layouts(&self) -> &[Layout];

    fn layouts_mut(&mut self) -> &mut [Layout];

    fn empty_layouts(&self) -> &[Layout];

    fn missing_item_qtys(&self) -> &[isize];

    fn usage(&mut self) -> f64 {
        let (total_bin_area, total_used_area) = self.layouts_mut().iter_mut().fold((0.0, 0.0), |acc, l| {
            let bin_area = l.bin().area();
            let used_area = bin_area * l.usage();
            (acc.0 + bin_area, acc.1 + used_area)
        });
        total_used_area / total_bin_area
    }

    fn used_bin_value(&self) -> u64 {
        self.layouts().iter().map(|l| l.bin().value()).sum()
    }

    fn included_item_qtys(&self) -> Vec<usize> {
        (0..self.missing_item_qtys().len())
            .map(|i| (self.instance().item_qty(i) as isize - self.missing_item_qtys()[i]) as usize)
            .collect_vec()
    }

    fn empty_layout_has_stock(&self, index: usize) -> bool {
        let bin_id = self.empty_layouts()[index].bin().id();
        self.bin_qtys()[bin_id] > 0
    }

    fn get_layout(&self, index: &LayoutIndex) -> &Layout {
        match index {
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
    use crate::entities::problems::problem::ProblemEnum;

    #[enum_dispatch(ProblemEnum)]
    pub trait ProblemPrivate {
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
pub enum LayoutIndex {
    Existing(usize),
    Empty(usize),
}