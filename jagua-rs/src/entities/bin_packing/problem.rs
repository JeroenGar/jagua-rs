use crate::entities::bin_packing::BPInstance;
use crate::entities::bin_packing::BPSolution;
use crate::entities::general::Instance;
use crate::entities::general::Layout;
use crate::entities::general::{PItemKey, PlacedItem};
use crate::geometry::DTransformation;
use crate::util::assertions;
use itertools::Itertools;
use slotmap::{SlotMap, new_key_type};
use std::time::Instant;

new_key_type! {
    /// Unique key for each [`Layout`] in a [`BPProblem`] and [`BPSolution`]
    pub struct LayKey;
}

/// Modifiable counterpart of [`BPInstance`]: items can be placed and removed, bins can be opened and closed.
#[derive(Clone)]
pub struct BPProblem {
    pub instance: BPInstance,
    pub layouts: SlotMap<LayKey, Layout>,
    pub missing_item_qtys: Vec<isize>,
    pub bin_qtys: Vec<usize>,
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

    /// Places an item according to the given `BPPlacement` in the problem.
    pub fn place_item(&mut self, p_opt: BPPlacement) -> (LayKey, PItemKey) {
        let lkey = match p_opt.layout_id {
            BPLayoutType::Open(lkey) => lkey,
            BPLayoutType::Closed { bin_id } => {
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

    /// Removes an item from a layout. If the layout is empty, it will be closed.
    /// Set `commit_instantly` to false if there's a high chance that this modification will be reverted.
    pub fn remove_item(
        &mut self,
        lkey: LayKey,
        pik: PItemKey,
        commit_instant: bool,
    ) -> BPPlacement {
        let pi = self.layouts[lkey].remove_item(pik, commit_instant);
        self.deregister_included_item(pi.item_id);
        if self.layouts[lkey].is_empty() {
            //if layout is empty, close it
            let bin_id = self.layouts[lkey].bin.id;
            self.deregister_layout(lkey);
            BPPlacement::from_placed_item(BPLayoutType::Closed { bin_id }, &pi)
        } else {
            BPPlacement::from_placed_item(BPLayoutType::Open(lkey), &pi)
        }
    }

    pub fn save(&mut self) -> BPSolution {
        let layout_snapshots = self
            .layouts
            .iter_mut()
            .map(|(lkey, l)| (lkey, l.save()))
            .collect();

        let target_item_qtys = self
            .instance
            .items()
            .iter()
            .map(|(_, qty)| *qty)
            .collect_vec();

        let solution = BPSolution {
            layout_snapshots,
            placed_item_qtys: self.placed_item_qtys().collect(),
            target_item_qtys,
            bin_qtys: self.bin_qtys.clone(),
            time_stamp: Instant::now(),
        };

        debug_assert!(assertions::bpproblem_matches_solution(self, &solution));

        solution
    }

    pub fn restore(&mut self, solution: &BPSolution) {
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

    pub fn density(&self) -> f32 {
        let total_bin_area = self.layouts.values().map(|l| l.bin.area()).sum::<f32>();

        let total_item_area = self
            .layouts
            .values()
            .map(|l| l.placed_item_area(&self.instance))
            .sum::<f32>();

        total_item_area / total_bin_area
    }

    pub fn placed_item_qtys(&self) -> impl Iterator<Item = usize> {
        self.missing_item_qtys
            .iter()
            .enumerate()
            .map(|(i, missing_qty)| (self.instance.item_qty(i) as isize - missing_qty) as usize)
    }

    fn register_layout(&mut self, layout: Layout) -> LayKey {
        self.register_bin(layout.bin.id);
        layout
            .placed_items()
            .values()
            .for_each(|pi| self.register_included_item(pi.item_id));
        self.layouts.insert(layout)
    }

    fn deregister_layout(&mut self, key: LayKey) {
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

#[derive(Clone, Debug, Copy)]
/// Encapsulates all required information to place an `Item` in a `Problem`
pub struct BPPlacement {
    /// Which layout to place the item in
    pub layout_id: BPLayoutType,
    /// The id of the item to be placed
    pub item_id: usize,
    /// The decomposition of the transformation
    pub d_transf: DTransformation,
}

impl BPPlacement {
    pub fn from_placed_item(layout_id: BPLayoutType, placed_item: &PlacedItem) -> Self {
        BPPlacement {
            layout_id,
            item_id: placed_item.item_id,
            d_transf: placed_item.d_transf,
        }
    }
}

/// Enum to distinguish between both existing layouts, and potential new layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BPLayoutType {
    /// An existing layout
    Open(LayKey),
    /// A layout that does not yet exist, but can be created
    Closed { bin_id: usize },
}
