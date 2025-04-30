use jagua_rs_base::entities::{Item};
use jagua_rs_base::util::assertions::layouts_match;
use crate::entities::{SPProblem, SPSolution};

pub fn problem_matches_solution(spp: &SPProblem, sol: &SPSolution) -> bool {
    let SPSolution {
        strip_width,
        layout_snapshot,
        time_stamp: _,
    } = sol;

    assert_eq!(*strip_width, spp.strip_width());
    assert_eq!(spp.density(), sol.density(&spp.instance));
    assert!(layouts_match(&spp.layout, layout_snapshot));

    true
}

pub fn instance_item_ids_correct(items: &Vec<(Item, usize)>) -> bool {
    items
        .iter()
        .enumerate()
        .all(|(i, (item, _qty))| item.id == i)
}