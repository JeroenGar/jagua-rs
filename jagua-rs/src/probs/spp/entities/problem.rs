use crate::Instant;
use crate::entities::{Layout, PItemKey};
use crate::geometry::DTransformation;
use crate::probs::spp::entities::strip::Strip;
use crate::probs::spp::entities::{SPInstance, SPSolution};
use crate::probs::spp::util::assertions::problem_matches_solution;
use anyhow::{Result, ensure};
use itertools::Itertools;

/// Modifiable counterpart of [`SPInstance`]: items can be placed and removed, strip can be extended or fitted.
#[derive(Clone)]
pub struct SPProblem {
    pub instance: SPInstance,
    pub strip: Strip,
    pub layout: Layout,
    pub item_demand_qtys: Vec<usize>,
}

impl SPProblem {
    /// Creates a problem, returning an error if its strip cannot form a container.
    pub fn new(instance: SPInstance) -> Result<Self> {
        let item_demand_qtys = instance.items.iter().map(|(_, qty)| *qty).collect_vec();
        let strip = instance.base_strip;
        let layout = Layout::new(strip.try_into()?);

        Ok(Self {
            instance,
            strip,
            layout,
            item_demand_qtys,
        })
    }

    /// Modifies the width of the strip in the back, keeping the front fixed.
    /// Returns an error without changing the problem if the new container is invalid.
    pub fn change_strip_width(&mut self, new_width: f32) -> Result<()> {
        ensure!(new_width > 0.0, "strip width must be positive");
        let mut strip = self.strip;
        strip.set_width(new_width);
        let container = strip.try_into()?;
        self.layout.swap_container(container);
        self.strip = strip;
        Ok(())
    }

    /// Shrinks the strip to the minimum width that fits all items.
    pub fn fit_strip(&mut self) -> Result<()> {
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

        self.change_strip_width(fitted_width)?;
        debug_assert_eq!(feasible_before, self.layout.is_feasible());
        Ok(())
    }

    /// Places an item according to the given `SPPlacement` in the problem.
    pub fn place_item(&mut self, placement: SPPlacement) -> PItemKey {
        self.register_included_item(placement.item_idx);
        let item = self.instance.item(placement.item_idx);

        self.layout.place_item(item, placement.d_transf)
    }

    /// Removes a placed item from the strip. Returns the placement of the item.
    pub fn remove_item(&mut self, pkey: PItemKey) -> SPPlacement {
        let pi = self.layout.remove_item(pkey);
        self.deregister_included_item(pi.item.idx);

        SPPlacement {
            item_idx: pi.item.idx,
            d_transf: pi.d_transf,
        }
    }

    /// Creates a snapshot of the current state of the problem as a [`SPSolution`].
    #[must_use]
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
        // A saved solution may use a different strip width.
        if self.layout.restore(&solution.layout_snapshot).is_err() {
            self.layout = Layout::from_snapshot(&solution.layout_snapshot);
        }
        self.strip = solution.strip;

        //Restore the item demands
        {
            self.item_demand_qtys
                .iter_mut()
                .enumerate()
                .for_each(|(idx, qty)| *qty = self.instance.item_qty(idx));

            self.layout
                .placed_items
                .iter()
                .for_each(|(_, pi)| self.item_demand_qtys[pi.item.idx] -= 1);
        }
        debug_assert!(problem_matches_solution(self, solution));
    }

    fn register_included_item(&mut self, item_idx: usize) {
        self.item_demand_qtys[item_idx] -= 1;
    }

    fn deregister_included_item(&mut self, item_idx: usize) {
        self.item_demand_qtys[item_idx] += 1;
    }

    #[must_use]
    pub fn density(&self) -> f32 {
        self.layout.density()
    }

    #[must_use]
    pub fn strip_width(&self) -> f32 {
        self.strip.width
    }

    #[must_use]
    pub fn n_placed_items(&self) -> usize {
        self.layout.placed_items.len()
    }
}

/// Represents a placement of an item in the strip packing problem.
#[derive(Debug, Clone, Copy)]
pub struct SPPlacement {
    pub item_idx: usize,
    pub d_transf: DTransformation,
}
