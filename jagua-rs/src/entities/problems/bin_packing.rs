use std::collections::HashSet;

use itertools::Itertools;

use crate::entities::instances::bin_packing::BPInstance;
use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::layout::Layout;
use crate::entities::placed_item::PlacedItemUID;
use crate::entities::placing_option::PlacingOption;
use crate::entities::problems::problem_generic::{LayoutIndex, ProblemGeneric};
use crate::entities::problems::problem_generic::private::ProblemGenericPrivate;
use crate::entities::solution::Solution;
use crate::util::assertions;

/// Bin Packing Problem
#[derive(Clone)]
pub struct BPProblem {
    pub instance: BPInstance,
    layouts: Vec<Layout>,
    template_layouts: Vec<Layout>,
    missing_item_qtys: Vec<isize>,
    bin_qtys: Vec<usize>,
    layout_id_counter: usize,
    solution_id_counter: usize,
    unchanged_layouts: Vec<usize>,
    unchanged_layouts_solution_id: Option<usize>,
    uncommitted_removed_layouts: Vec<Layout>,
}

impl BPProblem {
    pub fn new(instance: BPInstance) -> Self {
        let missing_item_qtys = instance.items.iter().map(|(_, qty)| *qty as isize).collect_vec();
        let bin_qtys = instance.bins.iter().map(|(_, qty)| *qty).collect_vec();
        let layouts = vec![];
        let template_layouts = instance.bins.iter().enumerate()
            .map(|(i, (bin, _))| Layout::new(i, bin.clone()))
            .collect_vec();
        let layout_id_counter = template_layouts.len();
        let unchanged_layouts = vec![];
        let unchanged_layouts_solution_id = None;
        let uncommitted_removed_layouts = vec![];

        Self {
            instance,
            layouts,
            template_layouts,
            missing_item_qtys,
            bin_qtys,
            layout_id_counter,
            solution_id_counter: 0,
            unchanged_layouts,
            unchanged_layouts_solution_id,
            uncommitted_removed_layouts,
        }
    }

    pub fn remove_layout(&mut self, layout_index: LayoutIndex) {
        self.unregister_layout(layout_index);
    }

    pub fn register_layout(&mut self, layout: Layout) -> LayoutIndex {
        self.register_bin(layout.bin().id);
        layout.placed_items().iter().for_each(
            |p_i| {
                self.register_included_item(p_i.item_id())
            }
        );
        self.layouts.push(layout);
        LayoutIndex::Real(self.layouts.len() - 1)
    }

    pub fn unregister_layout(&mut self, layout_index: LayoutIndex) {
        match layout_index {
            LayoutIndex::Real(i) => {
                let layout = self.layouts.remove(i);
                self.layout_has_changed(layout.id());
                self.unregister_bin(layout.bin().id);
                layout.placed_items().iter().for_each(
                    |v| { self.unregister_included_item(v.item_id()) });
                self.uncommitted_removed_layouts.push(layout);
            }
            LayoutIndex::Template(_) => unreachable!("cannot remove template layout")
        }
    }

    fn next_layout_id(&mut self) -> usize {
        self.layout_id_counter += 1;
        self.layout_id_counter
    }

    fn reset_unchanged_layouts(&mut self, unchanged_layouts_solution_id: usize) {
        self.unchanged_layouts = self.layouts.iter().map(|l| l.id()).collect();
        self.unchanged_layouts_solution_id = Some(unchanged_layouts_solution_id);
    }

    fn register_bin(&mut self, bin_id: usize) {
        assert!(self.bin_qtys[bin_id] > 0);
        self.bin_qtys[bin_id] -= 1
    }

    fn unregister_bin(&mut self, bin_id: usize) {
        self.bin_qtys[bin_id] += 1
    }

    fn layout_has_changed(&mut self, l_id: usize) {
        let index = self.unchanged_layouts.iter().position(|v| *v == l_id);
        if let Some(index) = index {
            self.unchanged_layouts.remove(index);
        }
    }
}

impl ProblemGeneric for BPProblem {
    fn place_item(&mut self, i_opt: &PlacingOption) -> LayoutIndex {
        let layout_index = match &i_opt.layout_index {
            LayoutIndex::Real(i) => LayoutIndex::Real(*i),
            LayoutIndex::Template(i) => {
                //Layout is empty, clone it and add it to `layouts`
                let next_layout_id = self.next_layout_id();
                let template = &self.template_layouts[*i];
                let copy = template.clone_with_id(next_layout_id);
                self.register_layout(copy)
            }
        };
        let layout = match layout_index {
            LayoutIndex::Real(i) => &mut self.layouts[i],
            LayoutIndex::Template(_) => unreachable!("cannot place item in template layout")
        };
        let item = self.instance.item(i_opt.item_id);
        layout.place_item(item, &i_opt.d_transf);
        let layout_id = layout.id();

        self.register_included_item(i_opt.item_id);
        self.layout_has_changed(layout_id);

        layout_index
    }

    fn remove_item(&mut self, layout_index: LayoutIndex, pi_uid: &PlacedItemUID, commit_instantly: bool) {
        match layout_index {
            LayoutIndex::Real(i) => {
                self.layout_has_changed(self.layouts[i].id());
                let layout = &mut self.layouts[i];
                layout.remove_item(pi_uid, commit_instantly);
                if layout.is_empty() {
                    //if layout is empty, remove it
                    self.unregister_layout(layout_index);
                }
                self.unregister_included_item(pi_uid.item_id);
            }
            LayoutIndex::Template(_) => unreachable!("cannot remove item from template layout")
        }
    }

    fn create_solution(&mut self, old_solution: &Option<Solution>) -> Solution {
        let id = self.next_solution_id();
        let included_item_qtys = self.included_item_qtys();
        let bin_qtys = self.bin_qtys().to_vec();
        let layout_snapshots = match old_solution {
            Some(old_solution) => {
                assert_eq!(old_solution.id, self.unchanged_layouts_solution_id.unwrap());
                self.layouts.iter_mut().map(|l| {
                    match self.unchanged_layouts.contains(&l.id()) {
                        true => old_solution.layout_snapshots.iter().find(|sl| sl.id == l.id()).unwrap().clone(),
                        false => l.create_layout_snapshot()
                    }
                }).collect()
            }
            None => {
                self.layouts.iter_mut().map(|l| l.create_layout_snapshot()).collect()
            }
        };

        let target_item_qtys = self.instance.items().iter().map(|(_, qty)| *qty).collect_vec();

        let solution = Solution::new(id, layout_snapshots, self.usage(), included_item_qtys, target_item_qtys, bin_qtys);

        debug_assert!(assertions::problem_matches_solution(self, &solution));
        self.reset_unchanged_layouts(solution.id);

        solution
    }

    fn restore_to_solution(&mut self, solution: &Solution) {
        match self.unchanged_layouts_solution_id == Some(solution.id) {
            true => {
                //A partial restore is possible.
                //TODO: hashset here is probably slower than just using a (sorted) vector
                let mut layout_ids_in_problem = HashSet::new();
                let mut layout_ids_not_in_solution = vec![];
                for layout in self.layouts.iter_mut() {
                    //For all layouts in the problem, check which ones occur in the solution
                    match solution.layout_snapshots.iter().position(|sl| sl.id == layout.id()) {
                        Some(i) => {
                            //layout is present in the solution
                            match self.unchanged_layouts.contains(&layout.id()) {
                                true => {
                                    //the layout is unchanged with respect to the solution, nothing needs to change
                                }
                                false => {
                                    //layout was changed, needs to be restored
                                    layout.restore(&solution.layout_snapshots[i]);
                                }
                            }
                            layout_ids_in_problem.insert(layout.id());
                        }
                        None => {
                            //layout is not present in the solution, remove it
                            layout_ids_not_in_solution.push(layout.id());
                        }
                    }
                }
                layout_ids_not_in_solution.iter().for_each(|id| { self.layouts.retain(|l| l.id() != *id); });

                //Some layouts are present in the solution, but not in the problem
                for sl in solution.layout_snapshots.iter() {
                    if !layout_ids_in_problem.contains(&sl.id) {
                        //It is possible they are in the uncommitted_removed_layouts
                        match self.uncommitted_removed_layouts.iter().position(|l| l.id() == sl.id) {
                            Some(i) => {
                                //original layout is still present, restore it and add it to the problem
                                let mut layout = self.uncommitted_removed_layouts.swap_remove(i);
                                layout.restore(sl);
                                self.layouts.push(layout);
                            }
                            None => {
                                //If not, the layout will have to be rebuilt from scratch
                                let layout = Layout::new_from_stored(sl.id, sl);
                                self.layouts.push(layout);
                            }
                        }
                    }
                }
            }
            false => {
                //The id of the solution does not match unchanged_layouts_solution_id, a partial restore is not possible
                self.layouts.clear();
                for sl in solution.layout_snapshots.iter() {
                    let layout = Layout::new_from_stored(sl.id, sl);
                    self.layouts.push(layout);
                }
            }
        }

        self.missing_item_qtys.iter_mut().enumerate().for_each(|(i, qty)| {
            *qty = (self.instance.item_qty(i) - solution.placed_item_qtys[i]) as isize
        });
        self.bin_qtys.clone_from_slice(&solution.bin_qtys);
        self.uncommitted_removed_layouts.clear();

        self.reset_unchanged_layouts(solution.id);

        debug_assert!(assertions::problem_matches_solution(self, solution));
    }

    fn layouts(&self) -> &[Layout] {
        &self.layouts
    }

    fn layouts_mut(&mut self) -> &mut [Layout] {
        &mut self.layouts
    }

    fn template_layouts(&self) -> &[Layout] {
        &self.template_layouts
    }

    fn missing_item_qtys(&self) -> &[isize] {
        &self.missing_item_qtys
    }

    fn included_item_qtys(&self) -> Vec<usize> {
        (0..self.missing_item_qtys().len())
            .map(|i| (self.instance.item_qty(i) as isize - self.missing_item_qtys()[i]) as usize)
            .collect_vec()
    }

    fn bin_qtys(&self) -> &[usize] {
        &self.bin_qtys
    }
}


impl ProblemGenericPrivate for BPProblem {
    fn next_solution_id(&mut self) -> usize {
        self.solution_id_counter += 1;
        self.solution_id_counter
    }

    fn missing_item_qtys_mut(&mut self) -> &mut [isize] {
        &mut self.missing_item_qtys
    }
}