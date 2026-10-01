use crate::entities::Item;
use crate::probs::spp::entities::{SPProblem, SPSolution};
use crate::util::assertions::snapshot_matches_layout;
use std::sync::Arc;

#[must_use]
pub fn problem_matches_solution(spp: &SPProblem, sol: &SPSolution) -> bool {
    let SPSolution {
        strip,
        layout_snapshot,
        time_stamp: _,
    } = sol;

    assert_eq!(*strip, spp.strip);
    assert!((spp.density() - sol.density()).abs() <= f32::EPSILON);
    assert!(snapshot_matches_layout(&spp.layout, layout_snapshot));

    true
}

#[must_use]
pub fn instance_item_indices_correct(items: &[(Arc<Item>, usize)]) -> bool {
    items
        .iter()
        .enumerate()
        .all(|(i, (item, _qty))| item.idx == i)
}
