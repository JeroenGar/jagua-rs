use crate::entities::Item;
use crate::probs::spp::entities::{SPProblem, SPSolution};
use crate::util::assertions::layouts_match;

pub fn problem_matches_solution(spp: &SPProblem, sol: &SPSolution) -> bool {
    let SPSolution {
        strip,
        layout_snapshot,
        time_stamp: _,
    } = sol;

    assert_eq!(*strip, spp.strip);
    assert_eq!(spp.density(), sol.density(&spp.instance));
    assert!(layouts_match(&spp.layout, layout_snapshot));

    true
}

pub fn instance_item_ids_correct(items: &[(Item, usize)]) -> bool {
    items
        .iter()
        .enumerate()
        .all(|(i, (item, _qty))| item.id == i)
}
