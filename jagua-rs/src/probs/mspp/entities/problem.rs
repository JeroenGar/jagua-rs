use crate::entities::{Instance, Layout, PItemKey};
use crate::geometry::DTransformation;
use crate::probs::mspp::entities::instance::MSPInstance;
use crate::probs::mspp::entities::strip::MStrip;
use crate::Instant;
use itertools::Itertools;
use slotmap::{new_key_type, SlotMap};

new_key_type! {
    /// Unique key for each [`Layout`] in a [`MSPProblem`] and [`MSPSolution`]
    pub struct LayKey;
}

/// Modifiable counterpart of [`MSPInstance`]: items can be placed and removed, strip can be extended or fitted.
#[derive(Clone)]
pub struct MSPProblem {
    pub instance: MSPInstance,
    pub layouts: SlotMap<LayKey, Layout>,
    pub strip: MStrip,
    pub strip_layout_: Layout,
    pub item_demand_qtys: Vec<usize>,
}

impl MSPProblem {
    pub fn new(instance: MSPInstance) -> Self {
        let item_demand_qtys = instance.items.iter().map(|(_, qty)| *qty).collect_vec();
        let strip = instance.base_strip;
        let strip_layout = Layout::new(strip.into());
        let full_layouts = SlotMap::with_key();

        Self {
            instance,
            full_layouts,
            strip,
            strip_layout,
            item_demand_qtys,
        }
    }

    /// Modifies the width of the strip in the back, keeping the front fixed.
    pub fn change_strip_layout_width(&mut self, new_width: f32) {
        self.strip.set_width(new_width);
        self.strip_layout.swap_container(self.strip.into());
    }

    pub fn remove_layout(&mut self, key: LayKey){
        self.deregister_layout(key);
    }

    /// Shrinks the strip to the minimum width that fits all items.
    pub fn fit_strip(&mut self) {
        let feasible_before = self.layout.is_feasible();

        //Find the rightmost item in the strip and add some tolerance (avoiding false collision positives)
        let item_x_max = self
            .layout
            .placed_items
            .values()
            .map(|pi| pi.shape.bbox.x_max)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            * 1.00001;

        // add the shape offset if any, the strip needs to be at least `offset` wider than the items
        let fitted_width = item_x_max + self.strip.shape_modify_config.offset.unwrap_or(0.0);

        self.change_strip_width(fitted_width);
        debug_assert!(feasible_before == self.layout.is_feasible());
    }

    /// Places an item according to the given `SPPlacement` in the problem.
    pub fn place_item(&mut self, placement: SPPlacement) -> PItemKey {
        self.register_included_item(placement.item_id);
        let item = self.instance.item(placement.item_id);

        self.layout.place_item(item, placement.d_transf)
    }

    /// Removes a placed item from the strip. Returns the placement of the item.
    /// Set `commit_instantly` to false if there's a high chance that this modification will be reverted.
    pub fn remove_item(&mut self, pkey: PItemKey) -> SPPlacement {
        let pi = self.layout.remove_item(pkey);
        self.deregister_included_item(pi.item_id);

        SPPlacement {
            item_id: pi.item_id,
            d_transf: pi.d_transf,
        }
    }

    /// Creates a snapshot of the current state of the problem as a [`SPSolution`].
    pub fn save(&self) -> SPSolution {
        let solution = SPSolution {
            layout_snapshot: self.layout.save(),
            strip: self.strip,
            time_stamp: Instant::now(),
        };

        debug_assert!(problem_matches_solution(self, &solution));

        solution
    }

    /// Restores the state of the problem to the given [`SPSolution`].
    pub fn restore(&mut self, solution: &SPSolution) {
        if self.strip == solution.strip {
            // the strip is the same, restore the layout
            self.layout.restore(&solution.layout_snapshot);
        } else {
            // the strip has changed, rebuild the layout
            self.layout = Layout::from_snapshot(&solution.layout_snapshot);
            self.strip = solution.strip;
        }

        //Restore the item demands
        {
            self.item_demand_qtys
                .iter_mut()
                .enumerate()
                .for_each(|(id, qty)| *qty = self.instance.item_qty(id));

            self.layout
                .placed_items
                .iter()
                .for_each(|(_, pi)| self.item_demand_qtys[pi.item_id] -= 1);
        }
        debug_assert!(problem_matches_solution(self, solution));
    }

    fn register_full_layout(&mut self, layout: Layout) ->LayKey {
        layout
            .placed_items
            .values()
            .for_each(|pi| self.register_included_item(pi.item_id));
        self.full_layouts.insert(layout)
    }

    fn deregister_full_layout(&mut self, key: LayKey) {
        let layout = self.full_layouts.remove(key).expect("layout key not present");
        layout
            .placed_items
            .values()
            .for_each(|pi| self.deregister_included_item(pi.item_id));
    }

    fn register_included_item(&mut self, item_id: usize) {
        self.item_demand_qtys[item_id] -= 1;
    }

    fn deregister_included_item(&mut self, item_id: usize) {
        self.item_demand_qtys[item_id] += 1;
    }

    pub fn density(&self) -> f32 {
        let total_container_area = self.all_layouts()
            .map(|l| l.container.area())
            .sum::<f32>();

        let total_item_area = self.all_layouts()
            .map(|l| l.placed_item_area(&self.instance))
            .sum::<f32>();

        total_item_area / total_container_area
    }

    pub fn all_layouts(&self) -> impl Iterator<Item = &Layout> {
        self.full_layouts.values().chain(std::iter::once(&self.strip_layout))
    }

    pub fn strip_width(&self) -> f32 {
        self.strip.width
    }
}

/// Represents a placement of an item in the strip packing problem.
#[derive(Debug, Clone, Copy)]
pub struct SPPlacement {
    pub item_id: usize,
    pub d_transf: DTransformation,
}
