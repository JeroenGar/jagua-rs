use crate::Instant;
use crate::entities::{Container, Instance, Layout, PItemKey};
use crate::geometry::DTransformation;
use crate::probs::mspp::entities::MSPSolution;
use crate::probs::mspp::entities::instance::MSPInstance;
use crate::probs::mspp::entities::strip::Strip;
use crate::probs::mspp::util::assertions::problem_matches_solution;
use itertools::Itertools;
use slotmap::{SecondaryMap, SlotMap, new_key_type};

new_key_type! {
    /// Unique key for each [`Layout`] in a [`MSPProblem`] and [`MSPSolution`]
    pub struct LayKey;
}

/// Modifiable counterpart of [`MSPInstance`]: items can be placed and removed; layouts can be added, removed, and modified.
#[derive(Clone)]
pub struct MSPProblem {
    /// The underlying instance
    pub instance: MSPInstance,
    /// The layouts in the problem
    pub layouts: SlotMap<LayKey, Layout>,
    /// The strips associated with each layout
    pub strips: SecondaryMap<LayKey, Strip>,
    /// The remaining demand quantities for each item
    pub item_demand_qtys: Vec<usize>,
}

impl MSPProblem {
    pub fn new(instance: MSPInstance) -> Self {
        let item_demand_qtys = instance.items.iter().map(|(_, qty)| *qty).collect_vec();

        Self {
            instance,
            layouts: SlotMap::with_key(),
            strips: SecondaryMap::new(),
            item_demand_qtys,
        }
    }

    /// Modifies the width of the strip in the back, keeping the front fixed.
    pub fn change_strip_width(&mut self, lk: LayKey, new_width: f32) {
        let bin_strip = &mut self.strips[lk];
        bin_strip.set_width(new_width);
        self.layouts[lk].swap_container(Container::from(*bin_strip));
    }

    pub fn remove_layout(&mut self, key: LayKey) {
        self.deregister_layout(key);
    }

    pub fn add_layout_from_strip(&mut self, strip: Strip) -> LayKey {
        let layout = Layout::new(Container::from(strip));
        let lk = self.register_layout(layout);
        self.strips.insert(lk, strip);
        lk
    }

    /// Shrinks the strip to the minimum width that fits all items.
    pub fn fit_strip(&mut self, lk: LayKey) {
        let feasible_before = self.layouts[lk].is_feasible();

        //Find the rightmost item in the strip and add some tolerance (avoiding false collision positives)
        let item_x_max = self.layouts[lk]
            .placed_items
            .values()
            .map(|pi| pi.shape.bbox.x_max)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            * 1.00001;

        // add the shape offset if any, the strip needs to be at least `offset` wider than the items
        let fitted_width = item_x_max + self.strips[lk].shape_modify_config.offset.unwrap_or(0.0);

        self.change_strip_width(lk, fitted_width);
        debug_assert!(feasible_before == self.layouts[lk].is_feasible());
    }

    /// Places an item according to the given `SPPlacement` in the problem.
    pub fn place_item(&mut self, placement: MSPPlacement) -> (LayKey, PItemKey) {
        let lk = placement.lk;

        let layout = &mut self.layouts[lk];
        let item = self.instance.item(placement.item_id);
        let pik = layout.place_item(item, placement.d_transf);

        self.register_included_item(placement.item_id);

        (lk, pik)
    }

    /// Removes a placed item from the strip. Returns the placement of the item.
    pub fn remove_item(&mut self, lk: LayKey, pk: PItemKey) -> MSPPlacement {
        let pi = self.layouts[lk].remove_item(pk);
        self.deregister_included_item(pi.item_id);

        MSPPlacement {
            lk,
            item_id: pi.item_id,
            d_transf: pi.d_transf,
        }
    }

    /// Creates a snapshot of the current state of the problem as a [`MSPSolution`].
    pub fn save(&self) -> MSPSolution {
        let solution = MSPSolution {
            layout_snapshots: self.layouts.iter().map(|(lk, l)| (lk, l.save())).collect(),
            strips: self.strips.clone(),
            time_stamp: Instant::now(),
        };

        debug_assert!(problem_matches_solution(self, &solution));

        solution
    }

    /// Restores the state of the problem to the given [`MSPSolution`].
    pub fn restore(&mut self, solution: &MSPSolution) {
        let mut layouts_to_remove = vec![];

        //Check which layouts from the problem are also present in the solution.
        //If a layout is present we might be able to do a (partial) restore instead of fully rebuilding everything.
        for (lk, layout) in self.layouts.iter_mut() {
            match solution.layout_snapshots.get(lk) {
                Some(ls) => {
                    //If the container (strip) still matches, we can do a restore
                    match self.strips[lk] == solution.strips[lk] {
                        true => layout.restore(ls),
                        false => layouts_to_remove.push(lk),
                    }
                }
                None => {
                    layouts_to_remove.push(lk);
                }
            }
        }

        //Remove all layouts that were not present in the solution (or have a different bin)
        for lk in layouts_to_remove {
            self.layouts.remove(lk);
        }

        //Create new layouts for all keys present in solution but not in problem
        for (lk, ls) in solution.layout_snapshots.iter() {
            if !self.layouts.contains_key(lk) {
                self.layouts.insert(Layout::from_snapshot(ls));
            }
        }

        //Restore the item demands and strips
        {
            self.item_demand_qtys
                .iter_mut()
                .enumerate()
                .for_each(|(id, demand)| {
                    *demand = self.instance.item_qty(id);
                });

            self.strips.clear();
            solution.strips.iter().for_each(|(lk, strip)| {
                self.strips.insert(lk, *strip);
            });
        }

        debug_assert!(problem_matches_solution(self, solution));
    }

    fn register_layout(&mut self, layout: Layout) -> LayKey {
        layout
            .placed_items
            .values()
            .for_each(|pi| self.register_included_item(pi.item_id));
        self.layouts.insert(layout)
    }

    fn deregister_layout(&mut self, key: LayKey) {
        let layout = self.layouts.remove(key).expect("layout key not present");
        layout
            .placed_items
            .values()
            .for_each(|pi| self.deregister_included_item(pi.item_id));

        self.strips.remove(key);
    }

    fn register_included_item(&mut self, item_id: usize) {
        self.item_demand_qtys[item_id] -= 1;
    }

    fn deregister_included_item(&mut self, item_id: usize) {
        self.item_demand_qtys[item_id] += 1;
    }

    pub fn density(&self) -> f32 {
        let total_container_area = self.all_layouts().map(|l| l.container.area()).sum::<f32>();

        let total_item_area = self
            .all_layouts()
            .map(|l| l.placed_item_area(&self.instance))
            .sum::<f32>();

        total_item_area / total_container_area
    }

    pub fn all_layouts(&self) -> impl Iterator<Item = &Layout> {
        self.layouts.values()
    }

    pub fn extendable_strips(&self) -> impl Iterator<Item = LayKey> {
        self.strips
            .iter()
            .filter(|(_, s)| s.width < s.max_width)
            .map(|(lk, _)| lk)
    }

    pub fn total_strip_width(&self) -> f32 {
        self.strips.iter().map(|(_, s)| s.width).sum()
    }
}

/// Represents a placement of an item in the strip packing problem.
#[derive(Debug, Clone, Copy)]
pub struct MSPPlacement {
    /// Which [`Layout`] to place the item in
    pub lk: LayKey,
    /// The id of the [`Item`](crate::entities::Item) to be placed
    pub item_id: usize,
    /// The transformation to apply to the item when placing it
    pub d_transf: DTransformation,
}
