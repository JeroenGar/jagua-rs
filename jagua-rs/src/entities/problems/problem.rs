use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::layout::Layout;
use crate::entities::placed_item::PItemKey;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::bin_packing::BPProblem;
use crate::entities::problems::problem_generic::private::ProblemGenericPrivate;
use crate::entities::problems::problem_generic::{LayoutIndex, ProblemGeneric};
use crate::entities::problems::strip_packing::SPProblem;
use crate::entities::solution::Solution;

/// Enum which contains all the different problem types.
/// A `Problem` represents a problem instance in a modifiable state.
/// It can insert or remove items, create a snapshot from the current state called a `Solution`,
/// and restore its state to a previous `Solution`.
/// <br>
/// Also enables the use of match statements on the `Problem` enum when variant-specific behavior is required,
/// When a new variant is added, compile errors will be generated everywhere specific behaviour is required.
#[derive(Clone)]
pub enum Problem {
    /// Bin Packing Problem
    BP(BPProblem),
    /// Strip Packing Problem
    SP(SPProblem),
}

impl ProblemGeneric for Problem {
    fn place_item(&mut self, p_opt: PlacingOption) -> (LayoutIndex, PItemKey) {
        match self {
            Problem::BP(bp) => bp.place_item(p_opt),
            Problem::SP(sp) => sp.place_item(p_opt),
        }
    }

    fn remove_item(
        &mut self,
        layout_index: LayoutIndex,
        pik: PItemKey,
        commit_instantly: bool,
    ) -> PlacingOption {
        match self {
            Problem::BP(bp) => bp.remove_item(layout_index, pik, commit_instantly),
            Problem::SP(sp) => sp.remove_item(layout_index, pik, commit_instantly),
        }
    }

    fn create_solution(&mut self, old_solution: Option<&Solution>) -> Solution {
        match self {
            Problem::BP(bp) => bp.create_solution(old_solution),
            Problem::SP(sp) => sp.create_solution(old_solution),
        }
    }

    fn restore_to_solution(&mut self, solution: &Solution) {
        match self {
            Problem::BP(bp) => bp.restore_to_solution(solution),
            Problem::SP(sp) => sp.restore_to_solution(solution),
        }
    }

    fn layouts(&self) -> &[Layout] {
        match self {
            Problem::BP(bp) => bp.layouts(),
            Problem::SP(sp) => sp.layouts(),
        }
    }

    fn layouts_mut(&mut self) -> &mut [Layout] {
        match self {
            Problem::BP(bp) => bp.layouts_mut(),
            Problem::SP(sp) => sp.layouts_mut(),
        }
    }

    fn template_layouts(&self) -> &[Layout] {
        match self {
            Problem::BP(bp) => bp.template_layouts(),
            Problem::SP(sp) => sp.template_layouts(),
        }
    }

    fn missing_item_qtys(&self) -> &[isize] {
        match self {
            Problem::BP(bp) => bp.missing_item_qtys(),
            Problem::SP(sp) => sp.missing_item_qtys(),
        }
    }

    fn bin_qtys(&self) -> &[usize] {
        match self {
            Problem::BP(bp) => bp.bin_qtys(),
            Problem::SP(sp) => sp.bin_qtys(),
        }
    }

    fn instance(&self) -> &dyn InstanceGeneric {
        match self {
            Problem::BP(bp) => bp.instance(),
            Problem::SP(sp) => sp.instance(),
        }
    }
}

impl ProblemGenericPrivate for Problem {
    fn next_solution_id(&mut self) -> usize {
        match self {
            Problem::BP(bp) => bp.next_solution_id(),
            Problem::SP(sp) => sp.next_solution_id(),
        }
    }

    fn next_layout_id(&mut self) -> usize {
        match self {
            Problem::BP(bp) => bp.next_layout_id(),
            Problem::SP(sp) => sp.next_layout_id(),
        }
    }

    fn missing_item_qtys_mut(&mut self) -> &mut [isize] {
        match self {
            Problem::BP(bp) => bp.missing_item_qtys_mut(),
            Problem::SP(sp) => sp.missing_item_qtys_mut(),
        }
    }
}

impl From<BPProblem> for Problem {
    fn from(bp: BPProblem) -> Self {
        Problem::BP(bp)
    }
}

impl From<SPProblem> for Problem {
    fn from(sp: SPProblem) -> Self {
        Problem::SP(sp)
    }
}
