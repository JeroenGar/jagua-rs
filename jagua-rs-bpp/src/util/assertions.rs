use crate::entities::{BPProblem, BPSolution};
use jagua_rs_base::entities::{Container, Item};
use jagua_rs_base::util::assertions::layouts_match;

pub fn problem_matches_solution(bpp: &BPProblem, sol: &BPSolution) -> bool {
    let BPSolution {
        layout_snapshots,
        placed_item_qtys,
        target_item_qtys: _,
        bin_qtys,
        time_stamp: _,
    } = sol;

    assert_eq!(bpp.density(), sol.density(&bpp.instance));
    assert_eq!(*placed_item_qtys, bpp.placed_item_qtys().collect_vec());
    assert_eq!(bin_qtys, &bpp.bin_qtys);

    for (lkey, l) in &bpp.layouts {
        let ls = &layout_snapshots[lkey];
        if !layouts_match(l, ls) {
            return false;
        }
    }

    true
}

pub fn instance_item_bin_ids_correct(items: &Vec<(Item, usize)>, bins: &Vec<(Container, usize)>) -> bool {
    items.iter().enumerate().all(|(i, (item, _))| item.id == i)
        && bins.iter().enumerate().all(|(i, (bin, _qty))| bin.id == i)
}
