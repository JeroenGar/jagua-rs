use crate::entities::Item;
use crate::probs::mspp::entities::{MSPProblem, MSPSolution};
use crate::util::assertions::layouts_match;

pub fn problem_matches_solution(spp: &MSPProblem, sol: &MSPSolution) -> bool {
    // let SPSolution {
    //     strip,
    //     layout_snapshot,
    //     time_stamp: _,
    // } = sol;
    //
    // assert_eq!(*strip, spp.strip);
    // assert_eq!(spp.density(), sol.density(&spp.instance));
    // assert!(layouts_match(&spp.layout, layout_snapshot));

    todo!()

    true
}

pub fn instance_item_ids_correct(items: &[(Item, usize)]) -> bool {
    items
        .iter()
        .enumerate()
        .all(|(i, (item, _qty))| item.id == i)
}
