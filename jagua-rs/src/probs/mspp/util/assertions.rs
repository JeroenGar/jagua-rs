use crate::entities::Item;
use crate::probs::mspp::entities::{MSPProblem, MSPSolution};
use crate::util::assertions::snapshot_matches_layout;

pub fn problem_matches_solution(spp: &MSPProblem, sol: &MSPSolution) -> bool {
    let MSPSolution {
        layout_snapshots,
        strips,
        time_stamp: _,
    } = sol;

    assert_eq!(*strips, spp.strips);
    assert_eq!(spp.density(), sol.density(&spp.instance));
    spp.layouts.iter().for_each(|(lkey, l)| {
        let ls = &layout_snapshots[lkey];
        assert!(snapshot_matches_layout(l, ls))
    });
    sol.layout_snapshots
        .keys()
        .for_each(|lkey| assert!(spp.layouts.contains_key(lkey)));

    true
}

pub fn instance_item_ids_correct(items: &[(Item, usize)]) -> bool {
    items
        .iter()
        .enumerate()
        .all(|(i, (item, _qty))| item.id == i)
}
