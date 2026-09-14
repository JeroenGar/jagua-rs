use crate::entities::Item;
use crate::probs::bpp::entities::{BPProblem, BPSolution, Bin};
use crate::util::assertions::snapshot_matches_layout;
use std::sync::Arc;

#[must_use]
pub fn problem_matches_solution(bpp: &BPProblem, sol: &BPSolution) -> bool {
    let BPSolution {
        layout_snapshots,
        layout_bins,
        time_stamp: _,
    } = sol;

    assert!((bpp.density() - sol.density()).abs() <= f32::EPSILON);
    assert_eq!(bpp.layouts.len(), layout_snapshots.len());

    // Check that each layout in the problem has a matching snapshot in the solution
    assert_eq!(bpp.bin_cost(), sol.cost(&bpp.instance));
    assert!(bpp.layouts.iter().all(|(key, layout)| {
        layout_snapshots.iter().any(|(saved_key, snapshot)| {
            bpp.layout_bins[key] == layout_bins[saved_key]
                && snapshot_matches_layout(layout, snapshot)
        })
    }));

    true
}

#[must_use]
pub fn instance_item_bin_ids_correct(items: &[(Arc<Item>, usize)], bins: &[Bin]) -> bool {
    items.iter().enumerate().all(|(i, (item, _))| item.idx == i)
        && bins.iter().enumerate().all(|(i, bin)| bin.idx == i)
}
