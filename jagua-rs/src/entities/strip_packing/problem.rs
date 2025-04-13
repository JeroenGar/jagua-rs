use crate::fsize;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::util::assertions;
use crate::util::config::CDEConfig;
use itertools::Itertools;
use std::time::Instant;
use crate::entities::general::bin::Bin;
use crate::entities::general::instance::Instance;
use crate::entities::general::layout::Layout;
use crate::entities::general::placed_item::PItemKey;
use crate::entities::strip_packing::instance::SPInstance;
use crate::entities::strip_packing::solution::SPSolution;
use crate::geometry::d_transformation::DTransformation;

/// Modifiable counterpart of [`SPInstance`]: items can be placed and removed, strip can be extended or fitted.
#[derive(Clone)]
pub struct SPProblem {
    pub instance: SPInstance,
    pub layout: Layout,
    pub missing_item_qtys: Vec<isize>,
}

impl SPProblem {
    pub fn new(instance: SPInstance, strip_width: fsize, cde_config: CDEConfig) -> Self {
        let strip_height = instance.strip_height;
        let missing_item_qtys = instance
            .items
            .iter()
            .map(|(_, qty)| *qty as isize)
            .collect_vec();
        let strip_bbox = AARectangle::new(0.0, 0.0, strip_width, strip_height);
        let strip_bin = Bin::from_strip(0, strip_bbox, cde_config);
        let layout = Layout::new(strip_bin);

        Self {
            instance,
            layout,
            missing_item_qtys,
        }
    }

    /// Adds width to the strip in the back, keeping the front fixed.
    pub fn extend_strip(&mut self, extra_width: usize) {
        let new_bbox = AARectangle::new(0.0, 0.0, self.strip_width() + extra_width as fsize, self.strip_height());
        let new_bin = Bin::from_strip(0, new_bbox, self.layout.bin.base_cde.config());
        self.layout.change_bin(new_bin);
    }

    /// Shrinks the strip to the minimum width that fits all items.
    pub fn fit_strip(&mut self) {
        let feasible_before = self.layout.is_feasible();
        //find the rightmost item in the strip and add some tolerance (avoiding false collision positives)
        let fitted_width = self.layout.placed_items.values()
            .map(|pi| pi.shape.bbox.x_max)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap() * 1.00001;

        let new_bbox = AARectangle::new(0.0, 0.0, fitted_width, self.strip_height());
        let new_bin = Bin::from_strip(0, new_bbox, self.layout.bin.base_cde.config());
        self.layout.change_bin(new_bin);
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
    pub fn remove_item(&mut self, pkey: PItemKey, commit_instant: bool) -> SPPlacement {
        let pi = self.layout.remove_item(pkey, commit_instant);
        self.deregister_included_item(pi.item_id);

        SPPlacement {
            item_id: pi.item_id,
            d_transf: pi.d_transf,
        }
    }
    
    pub fn save(&mut self) -> SPSolution {
        let solution = SPSolution {
            layout_snapshot: self.layout.save(),
            usage: self.usage(),
            strip_width: self.strip_width(),
            time_stamp: Instant::now(),
        };

        debug_assert!(assertions::spproblem_matches_solution(self, &solution));

        solution
    }

    pub fn restore(&mut self, solution: &SPSolution) {
        // restore or recreate the layout
        if self.strip_width() == solution.strip_width {
            self.layout.restore(&solution.layout_snapshot);
        } else {
            self.layout = Layout::from_snapshot(&solution.layout_snapshot);
        }

        //restore the missing item quantities
        self.missing_item_qtys
            .iter_mut()
            .enumerate()
            .for_each(|(id, qty)| *qty = self.instance.item_qty(id) as isize);

        self.layout
            .placed_items()
            .iter()
            .for_each(|(_, pi)| self.missing_item_qtys[pi.item_id] -= 1);

        debug_assert!(assertions::spproblem_matches_solution(self, solution));
    }

    pub fn strip_width(&self) -> fsize {
        self.layout.bin.outer.bbox().width()
    }

    pub fn strip_height(&self) -> fsize {
        self.layout.bin.outer.bbox().height()
    }
    
    fn register_included_item(&mut self, item_id: usize) {
        self.missing_item_qtys[item_id] -= 1;
    }

    fn deregister_included_item(&mut self, item_id: usize) {
        self.missing_item_qtys[item_id] += 1;
    }

    pub fn usage(&self) -> fsize {
        self.layout.usage()
    }
}

/// Represents a placement of an item in the strip packing problem.
#[derive(Debug, Clone, Copy)]
pub struct SPPlacement {
    item_id: usize,
    d_transf: DTransformation,
}