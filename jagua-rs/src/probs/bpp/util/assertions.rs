use crate::entities::Item;
use crate::probs::bpp::entities::{BPProblem, BPSolution, Bin};
use crate::util::assertions::snapshot_matches_layout;

pub fn problem_matches_solution(bpp: &BPProblem, sol: &BPSolution) -> bool {
    let BPSolution {
        layout_snapshots,
        time_stamp: _,
    } = sol;

    assert_eq!(bpp.density(), sol.density(&bpp.instance));
    assert_eq!(bpp.layouts.len(), layout_snapshots.len());

    // Check that each layout in the problem has a matching snapshot in the solution
    bpp.layouts.iter().all(|(_, l)| {
        layout_snapshots
            .iter()
            .any(|(_, ls)| snapshot_matches_layout(l, ls))
    });

    true
}

pub fn instance_item_bin_ids_correct(items: &[(Item, usize)], bins: &[Bin]) -> bool {
    items.iter().enumerate().all(|(i, (item, _))| item.id == i)
        && bins.iter().enumerate().all(|(i, bin)| bin.id == i)
}
