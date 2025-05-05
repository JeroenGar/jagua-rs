use crate::entities::Item;
use crate::probs::bpp::entities::{BPProblem, BPSolution, Bin};
use crate::util::assertions::layouts_match;

pub fn problem_matches_solution(bpp: &BPProblem, sol: &BPSolution) -> bool {
    let BPSolution {
        layout_snapshots,
        time_stamp: _,
    } = sol;

    for (lkey, l) in &bpp.layouts {
        let ls = &layout_snapshots[lkey];
        if !layouts_match(l, ls) {
            return false;
        }
    }

    true
}

pub fn instance_item_bin_ids_correct(items: &[(Item, usize)], bins: &[Bin]) -> bool {
    items.iter().enumerate().all(|(i, (item, _))| item.id == i)
        && bins.iter().enumerate().all(|(i, bin)| bin.id == i)
}
