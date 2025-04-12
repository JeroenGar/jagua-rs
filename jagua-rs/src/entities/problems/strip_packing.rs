use crate::collision_detection::hazard::HazardEntity;
use crate::entities::bin::Bin;
use crate::entities::instances::instance::Instance;
use crate::entities::instances::strip_packing::SPInstance;
use crate::entities::layout::{LayKey, Layout};
use crate::entities::placed_item::PItemKey;
use crate::entities::placement::{LayoutId, Placement};
use crate::entities::problems::problem::Problem;
use crate::entities::solution::SPSolution;
use crate::fsize;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::util::assertions;
use crate::util::config::CDEConfig;
use itertools::Itertools;
use log::error;
use std::iter;
use std::time::Instant;

/// Strip Packing Problem
#[derive(Clone)]
pub struct SPProblem {
    pub instance: SPInstance,
    pub layout: Layout,
    missing_item_qtys: Vec<isize>,
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

    /// Adds or removes width in the back of the strip.
    pub fn modify_strip_in_back(&mut self, new_width: fsize) {
        let bbox = self.layout.bin.outer.bbox();
        let new_strip_shape =
            AARectangle::new(bbox.x_min, bbox.y_min, bbox.x_min + new_width, bbox.y_max);
        self.modify_strip(new_strip_shape);
    }

    /// Adds or removes width at the front of the strip.
    pub fn modify_strip_at_front(&mut self, new_width: fsize) {
        let bbox = self.layout.bin.outer.bbox();
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
            self.layout.bin.outer.bbox().y_min,
            new_x_max,
            self.layout.bin.outer.bbox().y_max,
        );

        self.modify_strip(new_strip_shape);
    }

    /// Modifies the shape of the strip to a new rectangle.
    /// All items that fit in the new strip are kept, the rest are removed.
    pub fn modify_strip(&mut self, rect: AARectangle) {
        let placed_items = self
            .layout
            .placed_items()
            .iter()
            .map(|(_, pi)| (pi.item_id, pi.d_transf))
            .collect_vec();

        //reset the missing item quantities
        self.missing_item_qtys
            .iter_mut()
            .enumerate()
            .for_each(|(i, qty)| *qty = self.instance.item_qty(i) as isize);

        //Modifying the width causes the bin to change, so the layout must be replaced
        self.layout = Layout::new(Bin::from_strip(0, rect, self.layout.bin.base_cde.config()));

        //place the items back in the new layout
        for (item_id, d_transf) in placed_items {
            let item = self.instance.item(item_id);
            let entities_to_ignore = self
                .layout
                .cde()
                .all_hazards()
                .filter(|h| h.entity != HazardEntity::BinExterior)
                .map(|h| h.entity)
                .collect_vec();
            let shape = &item.shape;
            let transform = d_transf.compose();
            let transformed_shape = shape.transform_clone(&transform);
            let cde = self.layout.cde();
            if !cde.poly_collides(&transformed_shape, entities_to_ignore.as_ref()) {
                let insert_opt = Placement {
                    layout_id: LayoutId::Open(LayKey::default()),
                    item_id,
                    d_transf,
                };
                self.place_item(insert_opt);
            } else {
                let collisions =
                    cde.collect_poly_collisions(&transformed_shape, entities_to_ignore.as_ref());
                error!(
                    "Item {} could not be placed back in the strip after resizing. Collisions: {:?}",
                    item_id, collisions
                );
            }
        }
    }

    /// Shrinks the strip to the minimum width that fits all items.
    pub fn fit_strip(&mut self) {
        let n_items_in_old_strip = self.layout.placed_items().len();

        let fitted_width = self.occupied_width() * 1.00001; //add some tolerance to avoid rounding errors or false collision positives
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
}

impl Problem for SPProblem {
    type Instance = SPInstance;
    type Solution = SPSolution;
    fn place_item(&mut self, p_opt: Placement) -> (LayKey, PItemKey) {
        assert_eq!(
            p_opt.layout_id,
            LayoutId::Open(LayKey::default()),
            "Strip packing problems only have a single layout"
        );
        let item_id = p_opt.item_id;
        let item = self.instance.item(item_id);
        let placed_item_key = self.layout.place_item(item, p_opt.d_transf);

        self.register_included_item(item_id);
        (LayKey::default(), placed_item_key)
    }

    fn remove_item(&mut self, lkey: LayKey, pik: PItemKey, commit_instantly: bool) -> Placement {
        assert_eq!(
            lkey,
            LayKey::default(),
            "strip packing problems only have a single layout"
        );
        let pi = self.layout.remove_item(pik, commit_instantly);
        self.deregister_included_item(pi.item_id);

        Placement::from_placed_item(LayoutId::Open(lkey), &pi)
    }

    fn create_solution(&mut self) -> SPSolution {
        let solution = SPSolution {
            layout_snapshot: self.layout.create_snapshot(),
            usage: self.usage(),
            strip_width: self.strip_width(),
            time_stamp: Instant::now(),
        };

        debug_assert!(assertions::spproblem_matches_solution(self, &solution));

        solution
    }

    fn restore_to_solution(&mut self, solution: &SPSolution) {
        //restore the layout
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

    fn missing_item_qtys(&self) -> &[isize] {
        &self.missing_item_qtys
    }

    fn layout(&self, key: LayKey) -> &Layout {
        assert_eq!(
            key,
            LayKey::default(),
            "strip packing problems only have a single layout"
        );
        &self.layout
    }

    fn layouts(&self) -> impl Iterator<Item = (LayKey, &'_ Layout)> {
        iter::once((LayKey::default(), &self.layout))
    }

    fn layouts_mut(&mut self) -> impl Iterator<Item = (LayKey, &'_ mut Layout)> {
        iter::once((LayKey::default(), &mut self.layout))
    }

    fn bin_qtys(&self) -> &[usize] {
        &[0]
    }

    fn instance(&self) -> &Self::Instance {
        &self.instance
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
