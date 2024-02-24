use std::{iter, slice};

use itertools::Itertools;
use ordered_float::NotNan;

use crate::collision_detection::hazard_filter;
use crate::entities::bin::Bin;
use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::instances::strip_packing::SPInstance;
use crate::entities::layout::Layout;
use crate::entities::placed_item::PlacedItemUID;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::problem_generic::private::ProblemGenericPrivate;
use crate::entities::problems::problem_generic::LayoutIndex;
use crate::entities::problems::problem_generic::ProblemGeneric;
use crate::entities::solution::Solution;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::util::assertions;
use crate::util::config::CDEConfig;

/// Strip Packing Problem
#[derive(Clone)]
pub struct SPProblem {
    pub instance: SPInstance,
    pub layout: Layout,
    strip_height: f64,
    strip_width: f64,
    missing_item_qtys: Vec<isize>,
    solution_id_counter: usize,
}

impl SPProblem {
    pub fn new(instance: SPInstance, strip_width: f64, cde_config: CDEConfig) -> Self {
        let height = instance.strip_height;
        let missing_item_qtys = instance
            .items
            .iter()
            .map(|(_, qty)| *qty as isize)
            .collect_vec();
        let strip_bin = Bin::from_strip(0, strip_width, height, cde_config);
        let strip_height = height;
        let layout = Layout::new(0, strip_bin);

        Self {
            instance,
            layout,
            strip_height,
            strip_width,
            missing_item_qtys,
            solution_id_counter: 0,
        }
    }

    pub fn modify_strip_width(&mut self, new_width: f64) {
        let old_p_uids = self
            .layout
            .placed_items()
            .iter()
            .map(|p_i| p_i.uid.clone())
            .collect_vec();
        self.missing_item_qtys
            .iter_mut()
            .enumerate()
            .for_each(|(i, qty)| *qty = self.instance.item_qty(i) as isize);
        let next_id = self.layout.id() + 1;
        self.layout = Layout::new(
            next_id,
            Bin::from_strip(
                next_id,
                new_width,
                self.strip_height,
                self.layout.bin().base_cde.config().clone(),
            ),
        );
        self.strip_width = new_width;

        for p_uid in old_p_uids {
            let item = self.instance.item(p_uid.item_id);
            let entities_to_ignore = item.hazard_filter.as_ref().map_or(vec![], |f| {
                hazard_filter::generate_irrelevant_hazards(f, self.layout.cde().all_hazards())
            });
            let shape = &item.shape;
            let transf = p_uid.d_transf.compose();
            if !self.layout.cde().surrogate_collides(
                shape.surrogate(),
                &transf,
                entities_to_ignore.as_slice(),
            ) {
                let transformed_shape = shape.transform_clone(&transf);
                if !self
                    .layout
                    .cde()
                    .shape_collides(&transformed_shape, entities_to_ignore.as_ref())
                {
                    let insert_opt = PlacingOption {
                        layout_index: LayoutIndex::Real(0),
                        item_id: p_uid.item_id,
                        transform: transf,
                        d_transform: p_uid.d_transf.clone(),
                    };
                    self.place_item(&insert_opt);
                }
            }
        }
    }

    pub fn fit_strip_width(&mut self) {
        let max_x = self
            .layout
            .placed_items()
            .iter()
            .map(|pi| pi.shape.bbox().x_max)
            .map(|x| NotNan::new(x).unwrap())
            .max()
            .map_or(0.0, |x| x.into_inner());

        let strip_width = max_x + f32::EPSILON.sqrt() as f64;
        let n_items_in_old_strip = self.layout.placed_items().len();

        self.modify_strip_width(strip_width);

        assert_eq!(n_items_in_old_strip, self.layout.placed_items().len());
    }

    pub fn strip_height(&self) -> f64 {
        self.strip_height
    }

    pub fn strip_width(&self) -> f64 {
        self.strip_width
    }
}

impl ProblemGeneric for SPProblem {
    fn place_item(&mut self, i_opt: &PlacingOption) -> LayoutIndex {
        assert_eq!(
            i_opt.layout_index,
            LayoutIndex::Real(0),
            "Strip packing problems only have a single layout"
        );
        let item_id = i_opt.item_id;
        let item = self.instance.item(item_id);
        self.layout.place_item(item, &i_opt.d_transform);

        self.register_included_item(item_id);
        LayoutIndex::Real(0)
    }

    fn remove_item(
        &mut self,
        layout_index: LayoutIndex,
        pi_uid: &PlacedItemUID,
        commit_instantly: bool,
    ) {
        assert_eq!(
            layout_index,
            LayoutIndex::Real(0),
            "strip packing problems only have a single layout"
        );
        self.layout.remove_item(pi_uid, commit_instantly);
        self.deregister_included_item(pi_uid.item_id);
    }

    fn create_solution(&mut self, _old_solution: &Option<Solution>) -> Solution {
        let id = self.next_solution_id();
        let included_item_qtys = self.placed_item_qtys().collect_vec();
        let bin_qtys = self.bin_qtys().to_vec();
        let layout_snapshots = vec![self.layout.create_layout_snapshot()];
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
        self.layout.restore(&solution.layout_snapshots[0]);

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

    fn missing_item_qtys_mut(&mut self) -> &mut [isize] {
        &mut self.missing_item_qtys
    }
}
