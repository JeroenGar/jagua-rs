use std::{iter, slice};

use itertools::Itertools;

use crate::collision_detection::hazard_filter;
use crate::entities::bin::Bin;
use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::instances::strip_packing::SPInstance;
use crate::entities::layout::Layout;
use crate::entities::placed_item::PlacedItemUID;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::problem_generic::private::ProblemGenericPrivate;
use crate::entities::problems::problem_generic::ProblemGeneric;
use crate::entities::problems::problem_generic::{LayoutIndex, STRIP_LAYOUT_IDX};
use crate::entities::solution::Solution;
use crate::fsize;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::util::assertions;
use crate::util::config::CDEConfig;
use crate::util::fpa::FPA;

/// Strip Packing Problem
#[derive(Clone)]
pub struct SPProblem {
    pub instance: SPInstance,
    pub layout: Layout,
    missing_item_qtys: Vec<isize>,
    layout_id_counter: usize,
    solution_id_counter: usize,
}

impl SPProblem {
    pub fn new(instance: SPInstance, strip_width: fsize, cde_config: CDEConfig) -> Self {
        let strip_height = instance.strip_height;
        let missing_item_qtys = instance
            .items
            .iter()
            .map(|(_, qty)| *qty as isize)
            .collect_vec();
        let strip_rect = AARectangle::new(0.0, 0.0, strip_width, strip_height);
        let strip_bin = Bin::from_strip(strip_rect, cde_config);
        let layout_id_counter = 0;
        let layout = Layout::new(layout_id_counter, strip_bin);

        Self {
            instance,
            layout,
            missing_item_qtys,
            layout_id_counter,
            solution_id_counter: 0,
        }
    }

    /// Adds or removes width in the back of the strip.
    pub fn modify_strip_in_back(&mut self, new_width: fsize) {
        let bbox = self.layout.bin().outer.bbox();
        let new_strip_shape =
            AARectangle::new(bbox.x_min, bbox.y_min, bbox.x_min + new_width, bbox.y_max);
        self.modify_strip(new_strip_shape);
    }

    /// Adds or removes width at the front of the strip.
    pub fn modify_strip_at_front(&mut self, new_width: fsize) {
        let bbox = self.layout.bin().outer.bbox();
        let new_strip_shape =
            AARectangle::new(bbox.x_max - new_width, bbox.y_min, bbox.x_max, bbox.y_max);
        self.modify_strip(new_strip_shape);
    }

    /// Adds or removes width, dividing it equally at the front and back of the current items.
    pub fn modify_strip_centered(&mut self, new_width: fsize) {
        let current_range = self.occupied_range().unwrap_or((0.0, 0.0));
        let current_width = self.occupied_width();

        //divide the added or removed width to the left and right of the strip
        let added_width = new_width - current_width;
        let new_x_min = current_range.0 - added_width / 2.0;
        let new_x_max = current_range.1 + added_width / 2.0;

        let new_strip_shape = AARectangle::new(
            new_x_min,
            self.layout.bin().outer.bbox().y_min,
            new_x_max,
            self.layout.bin().outer.bbox().y_max,
        );

        self.modify_strip(new_strip_shape);
    }

    /// Modifies the shape of the strip to a new rectangle.
    /// All items that fit in the new strip are kept, the rest are removed.
    pub fn modify_strip(&mut self, rect: AARectangle) {
        let placed_items_uids = self
            .layout
            .placed_items()
            .iter()
            .map(|(_, pi)| pi.uid.clone())
            .collect_vec();

        //reset the missing item quantities
        self.missing_item_qtys
            .iter_mut()
            .enumerate()
            .for_each(|(i, qty)| *qty = self.instance.item_qty(i) as isize);

        //Modifying the width causes the bin to change, so the layout must be replaced
        self.layout = Layout::new(
            self.next_layout_id(),
            Bin::from_strip(rect, self.layout.bin().base_cde.config().clone()),
        );

        //place the items back in the new layout
        for p_uid in placed_items_uids {
            let item = self.instance.item(p_uid.item_id);
            let entities_to_ignore = item.hazard_filter.as_ref().map_or(vec![], |f| {
                hazard_filter::generate_irrelevant_hazards(f, self.layout.cde().all_hazards())
            });
            let shape = &item.shape;
            let transform = p_uid.d_transf.compose();
            let d_transform = transform.decompose();
            let transformed_shape = shape.transform_clone(&transform);
            let cde = self.layout.cde();
            if !cde.poly_collides(&transformed_shape, entities_to_ignore.as_ref()) {
                let insert_opt = PlacingOption {
                    layout_index: STRIP_LAYOUT_IDX,
                    item_id: p_uid.item_id,
                    transform,
                    d_transform,
                };
                self.place_item(&insert_opt);
            }
        }
    }

    /// Shrinks the strip to the minimum width that fits all items.
    pub fn fit_strip(&mut self) {
        let n_items_in_old_strip = self.layout.placed_items().len();

        let fitted_width = self.occupied_width() * (1.0 + FPA::tolerance()); //add some tolerance to avoid rounding errors or false collision positives
        self.modify_strip_centered(fitted_width);

        assert_eq!(
            n_items_in_old_strip,
            self.layout.placed_items().len(),
            "fitting the strip should not remove any items"
        );
    }

    /// Returns the horizontal range occupied by the placed items. If no items are placed, returns None.
    pub fn occupied_range(&self) -> Option<(fsize, fsize)> {
        occupied_range(&self.layout)
    }

    /// Returns the width occupied by the placed items.
    pub fn occupied_width(&self) -> fsize {
        occupied_width(&self.layout)
    }

    pub fn strip_width(&self) -> fsize {
        self.layout.bin().outer.bbox().width()
    }

    pub fn strip_height(&self) -> fsize {
        self.layout.bin().outer.bbox().height()
    }
}

impl ProblemGeneric for SPProblem {
    fn place_item(&mut self, p_opt: &PlacingOption) -> LayoutIndex {
        assert_eq!(
            p_opt.layout_index, STRIP_LAYOUT_IDX,
            "Strip packing problems only have a single layout"
        );
        let item_id = p_opt.item_id;
        let item = self.instance.item(item_id);
        self.layout.place_item(item, &p_opt.d_transform);

        self.register_included_item(item_id);
        STRIP_LAYOUT_IDX
    }

    fn remove_item(
        &mut self,
        layout_index: LayoutIndex,
        pi_uid: &PlacedItemUID,
        commit_instantly: bool,
    ) {
        assert_eq!(
            layout_index, STRIP_LAYOUT_IDX,
            "strip packing problems only have a single layout"
        );
        self.layout.remove_item_with_uid(pi_uid, commit_instantly);
        self.deregister_included_item(pi_uid.item_id);
    }

    fn create_solution(&mut self, _old_solution: &Option<Solution>) -> Solution {
        let id = self.next_solution_id();
        let included_item_qtys = self.placed_item_qtys().collect_vec();
        let bin_qtys = self.bin_qtys().to_vec();
        let layout_snapshots = vec![self.layout.create_snapshot()];
        let target_item_qtys = self
            .instance
            .items
            .iter()
            .map(|(_, qty)| *qty)
            .collect_vec();

        let solution = Solution::new(
            id,
            layout_snapshots,
            self.usage(),
            included_item_qtys,
            target_item_qtys,
            bin_qtys,
        );

        debug_assert!(assertions::problem_matches_solution(self, &solution));

        solution
    }

    fn restore_to_solution(&mut self, solution: &Solution) {
        debug_assert!(solution.layout_snapshots.len() == 1);

        //restore the layout
        let layout_snapshot = &solution.layout_snapshots[0];
        match self.layout.id() == layout_snapshot.id {
            true => self.layout.restore(layout_snapshot),
            false => self.layout = Layout::from_snapshot(layout_snapshot),
        }

        //restore the missing item quantities
        self.missing_item_qtys
            .iter_mut()
            .enumerate()
            .for_each(|(i, qty)| {
                *qty = (self.instance.item_qty(i) - solution.placed_item_qtys[i]) as isize
            });

        debug_assert!(assertions::problem_matches_solution(self, solution));
    }

    fn layouts(&self) -> &[Layout] {
        slice::from_ref(&self.layout)
    }

    fn layouts_mut(&mut self) -> &mut [Layout] {
        slice::from_mut(&mut self.layout)
    }

    fn template_layouts(&self) -> &[Layout] {
        &[]
    }

    fn missing_item_qtys(&self) -> &[isize] {
        &self.missing_item_qtys
    }

    fn template_layout_indices_with_stock(&self) -> impl Iterator<Item = LayoutIndex> {
        iter::empty::<LayoutIndex>()
    }

    fn bin_qtys(&self) -> &[usize] {
        &[0]
    }

    fn instance(&self) -> &dyn InstanceGeneric {
        &self.instance
    }
}

impl ProblemGenericPrivate for SPProblem {
    fn next_solution_id(&mut self) -> usize {
        self.solution_id_counter += 1;
        self.solution_id_counter
    }

    fn next_layout_id(&mut self) -> usize {
        self.layout_id_counter += 1;
        self.layout_id_counter
    }

    fn missing_item_qtys_mut(&mut self) -> &mut [isize] {
        &mut self.missing_item_qtys
    }
}

/// Returns the horizontal range occupied by the placed items. If no items are placed, returns None.
pub fn occupied_range(layout: &Layout) -> Option<(fsize, fsize)> {
    if layout.placed_items().is_empty() {
        return None;
    }

    let mut min_x = fsize::MAX;
    let mut max_x = fsize::MIN;

    for pi in layout.placed_items().values() {
        let bbox = pi.shape.bbox();
        min_x = min_x.min(bbox.x_min);
        max_x = max_x.max(bbox.x_max);
    }

    Some((min_x, max_x))
}

/// Returns the total width occupied by the placed items.
pub fn occupied_width(layout: &Layout) -> fsize {
    let range = occupied_range(layout);
    match range {
        Some((min_x, max_x)) => max_x - min_x,
        None => 0.0,
    }
}
