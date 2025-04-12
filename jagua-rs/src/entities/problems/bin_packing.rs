use crate::entities::instances::bin_packing::BPInstance;
use crate::entities::instances::instance::Instance;
use crate::entities::layout::{LayKey, Layout};
use crate::entities::placed_item::PItemKey;
use crate::entities::placement::{LayoutId, Placement};
use crate::entities::problems::problem::Problem;
use crate::util::assertions;
use itertools::Itertools;
use slotmap::SlotMap;
use std::time::Instant;
use crate::entities::solution::BPSolution;

/// Bin Packing Problem
#[derive(Clone)]
pub struct BPProblem {
    pub instance: BPInstance,
    pub layouts: SlotMap<LayKey, Layout>,
    missing_item_qtys: Vec<isize>,
    bin_qtys: Vec<usize>,
}

impl BPProblem {
    pub fn new(instance: BPInstance) -> Self {
        let missing_item_qtys = instance
            .items
            .iter()
            .map(|(_, qty)| *qty as isize)
            .collect_vec();
        let bin_qtys = instance.bins.iter().map(|(_, qty)| *qty).collect_vec();

        Self {
            instance,
            layouts: SlotMap::with_key(),
            missing_item_qtys,
            bin_qtys,
        }
    }

    pub fn remove_layout(&mut self, key: LayKey) {
        self.deregister_layout(key);
    }

    pub fn register_layout(&mut self, layout: Layout) -> LayKey {
        self.register_bin(layout.bin.id);
        layout
            .placed_items()
            .values()
            .for_each(|pi| self.register_included_item(pi.item_id));
        self.layouts.insert(layout)
    }

    pub fn deregister_layout(&mut self, key: LayKey) {
        let layout = self.layouts.remove(key).expect("layout key not present");
        self.deregister_bin(layout.bin.id);
        layout
            .placed_items()
            .values()
            .for_each(|pi| self.deregister_included_item(pi.item_id));
    }

    fn register_bin(&mut self, bin_id: usize) {
        assert!(self.bin_qtys[bin_id] > 0);
        self.bin_qtys[bin_id] -= 1
    }

    fn deregister_bin(&mut self, bin_id: usize) {
        self.bin_qtys[bin_id] += 1
    }

    fn register_included_item(&mut self, item_id: usize) {
        self.missing_item_qtys[item_id] -= 1;
    }

    fn deregister_included_item(&mut self, item_id: usize) {
        self.missing_item_qtys[item_id] += 1;
    }
}

impl Problem for BPProblem {
    type Instance = BPInstance;
    type Solution = BPSolution;
    fn place_item(&mut self, p_opt: Placement) -> (LayKey, PItemKey) {
        let lkey = match p_opt.layout_id {
            LayoutId::Open(lkey) => lkey,
            LayoutId::Closed { bin_id } => {
                //open a new layout
                let bin = self.instance.bins[bin_id].clone().0;
                let layout = Layout::new(bin);
                self.register_layout(layout)
            }
        };
        let layout = &mut self.layouts[lkey];
        let item = self.instance.item(p_opt.item_id);
        let pik = layout.place_item(item, p_opt.d_transf);

        self.register_included_item(p_opt.item_id);

        (lkey, pik)
    }

    fn remove_item(
        &mut self,
        lkey: LayKey,
        pik: PItemKey,
        commit_instantly: bool,
    ) -> Placement {
        let pi = self.layouts[lkey].remove_item(pik, commit_instantly);
        self.deregister_included_item(pi.item_id);
        if self.layouts[lkey].is_empty() {
            //if layout is empty, close it
            let bin_id = self.layouts[lkey].bin.id;
            self.deregister_layout(lkey);
            Placement::from_placed_item(LayoutId::Closed { bin_id }, &pi)
        } else {
            Placement::from_placed_item(LayoutId::Open(lkey), &pi)
        }
    }

    fn create_solution(&mut self) -> BPSolution {
        let layout_snapshots = self
            .layouts
            .iter_mut()
            .map(|(lkey, l)| (lkey, l.create_snapshot()))
            .collect();

        let target_item_qtys = self
            .instance
            .items()
            .iter()
            .map(|(_, qty)| *qty)
            .collect_vec();

        let solution = BPSolution {
            layout_snapshots,
            usage: self.usage(),
            placed_item_qtys: self.placed_item_qtys().collect(),
            target_item_qtys,
            bin_qtys: self.bin_qtys.clone(),
            time_stamp: Instant::now(),
        };

        debug_assert!(assertions::bpproblem_matches_solution(self, &solution));

        solution
    }

    fn restore_to_solution(&mut self, solution: &BPSolution) {
        let mut layouts_to_remove = vec![];

        //Check which layouts from the problem are also present in the solution.
        //If a layout is present we might be able to do a (partial) restore instead of fully rebuilding everything.
        for (lkey, layout) in self.layouts.iter_mut() {
            match solution.layout_snapshots.get(lkey) {
                Some(ls) => match layout.bin.id == ls.bin.id {
                    true => layout.restore(ls),
                    false => layouts_to_remove.push(lkey),
                },
                None => {
                    layouts_to_remove.push(lkey);
                }
            }
        }

        //Remove all layouts that were not present in the solution (or have a different bin)
        for lkey in layouts_to_remove {
            self.layouts.remove(lkey);
        }

        //Create new layouts for all keys present in solution but not in problem
        for (lkey, ls) in solution.layout_snapshots.iter() {
            if !self.layouts.contains_key(lkey) {
                self.layouts.insert(Layout::from_snapshot(ls));
            }
        }

        //Restore missing item quantities to the state of the solution
        self.missing_item_qtys
            .iter_mut()
            .enumerate()
            .for_each(|(i, missing_qty)| {
                *missing_qty = (self.instance.item_qty(i) - solution.placed_item_qtys[i]) as isize
            });

        self.bin_qtys.clone_from_slice(&solution.bin_qtys);

        debug_assert!(assertions::bpproblem_matches_solution(self, solution));
    }

    fn missing_item_qtys(&self) -> &[isize] {
        &self.missing_item_qtys
    }

    fn layout(&self, key: LayKey) -> &Layout {
        &self.layouts[key]
    }

    fn layouts(&self) -> impl Iterator<Item = (LayKey, &'_ Layout)> {
        self.layouts.iter()
    }

    fn layouts_mut(&mut self) -> impl Iterator<Item=(LayKey, &'_ mut Layout)> {
        self.layouts.iter_mut()
    }

    fn bin_qtys(&self) -> &[usize] {
        &self.bin_qtys
    }

    fn instance(&self) -> &Self::Instance {
        &self.instance
    }
}
