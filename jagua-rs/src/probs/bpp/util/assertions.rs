use crate::entities::Item;
use crate::probs::bpp::entities::{BPProblem, BPSolution, Bin};
use crate::util::assertions::snapshot_matches_layout;

pub fn problem_matches_solution(bpp: &BPProblem, sol: &BPSolution) -> bool {
    let BPSolution {
        layout_snapshots,
        time_stamp: _,
    } = sol;

    assert_eq!(bpp.density(), sol.density(&bpp.instance));
    bpp.layouts.iter().for_each(|(lkey, l)| {
        let ls = &layout_snapshots[lkey];
        assert!(snapshot_matches_layout(l, ls))
    });
    sol.layout_snapshots.keys().for_each(|lkey| {
        assert!(bpp.layouts.contains_key(lkey));
    });

    true
}

pub fn instance_item_bin_ids_correct(items: &[(Item, usize)], bins: &[Bin]) -> bool {
    items.iter().enumerate().all(|(i, (item, _))| item.id == i)
        && bins.iter().enumerate().all(|(i, bin)| bin.id == i)
}
