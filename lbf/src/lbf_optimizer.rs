use std::any::Any;
use std::cmp::{Ordering, Reverse};
use std::time::Instant;

use itertools::Itertools;
use log::{debug, info};
use ordered_float::NotNan;
use rand::prelude::SmallRng;
use rand::Rng;
use thousands::Separable;

use jagua_rs::collision_detection::hazard_filter;
use jagua_rs::entities::instances::bin_packing::BPInstance;
use jagua_rs::entities::instances::instance::Instance;
use jagua_rs::entities::instances::strip_packing::SPInstance;
use jagua_rs::entities::item::Item;
use jagua_rs::entities::placement::{LayoutId, Placement};
use jagua_rs::entities::problems::bin_packing::BPProblem;
use jagua_rs::entities::problems::problem::Problem;
use jagua_rs::entities::problems::strip_packing::SPProblem;
use jagua_rs::entities::solution::{BPSolution, SPSolution};
use jagua_rs::fsize;
use jagua_rs::geometry::convex_hull::convex_hull_from_points;
use jagua_rs::geometry::geo_traits::{Shape, TransformableFrom};
use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;

use crate::lbf_config::LBFConfig;
use crate::lbf_cost::LBFPlacingCost;
use crate::samplers::hpg_sampler::HPGSampler;
use crate::samplers::ls_sampler::LSSampler;

//limits the number of items to be placed, for debugging purposes
pub const ITEM_LIMIT: usize = usize::MAX;

pub struct LBFOptimizer<P: Problem> {
    pub instance: P::Instance,
    pub problem: P,
    pub config: LBFConfig,
    /// SmallRng is a fast, non-cryptographic PRNG <https://rust-random.github.io/book/guide-rngs.html>
    pub rng: SmallRng,
    pub sample_counter: usize,
}

impl LBFOptimizer<SPProblem> {
    pub fn from_sp_instance(instance: SPInstance, config: LBFConfig, rng: SmallRng) -> Self {
        assert!(config.n_samples > 0);
        let strip_width = instance.item_area() * 2.0 / instance.strip_height; //initiate with 50% usage
        let problem = SPProblem::new(instance.clone(), strip_width, config.cde_config).into();
        Self {
            instance,
            problem,
            config,
            rng,
            sample_counter: 0,
        }
    }

    pub fn solve2(&self) -> SPSolution {
        todo!()
    }
}

impl LBFOptimizer<BPProblem> {
    pub fn from_bp_instance(instance: BPInstance, config: LBFConfig, rng: SmallRng) -> Self {
        assert!(config.n_samples > 0);
        let problem = BPProblem::new(instance.clone()).into();
        Self {
            instance,
            problem,
            config,
            rng,
            sample_counter: 0,
        }
    }

    pub fn solve2(&self) -> BPSolution {
        todo!()
    }
}

impl<P: Problem> LBFOptimizer<P> {
    pub fn solve(&mut self) -> P::Solution {
        //sort the items by descending diameter of convex hull
        let sorted_item_indices = (0..self.instance.items().len())
            .sorted_by_cached_key(|i| {
                let item = &self.instance.items()[*i].0;
                let ch = SimplePolygon::new(convex_hull_from_points(item.shape.points.clone()));
                let ch_diam = NotNan::new(ch.diameter()).expect("convex hull diameter is NaN");
                Reverse(ch_diam)
            })
            .collect_vec();

        let start = Instant::now();

        'outer: for item_index in sorted_item_indices {
            let item = &self.instance.items()[item_index].0;
            //place all items of this type
            'inner: while self.problem.missing_item_qtys()[item_index] > 0 {
                //find a position and insert it
                match find_lbf_placement(
                    &self.problem,
                    item,
                    &self.config,
                    &mut self.rng,
                    &mut self.sample_counter,
                ) {
                    Some(i_opt) => {
                        let l_index = self.problem.place_item(i_opt);
                        info!(
                            "[LBF] placing item {}/{} with id {} at [{}] in Layout {:?}",
                            self.problem.placed_item_qtys().sum::<usize>(),
                            self.instance.total_item_qty(),
                            i_opt.item_id,
                            i_opt.d_transf,
                            l_index
                        );
                        #[allow(clippy::absurd_extreme_comparisons)]
                        if self.problem.placed_item_qtys().sum::<usize>() >= ITEM_LIMIT {
                            break 'outer;
                        }
                    }
                    None => {
                        let any = &mut self.problem as &mut dyn Any;
                        if let Some(_) = any.downcast_ref::<BPProblem>() {
                            // items of this type don't fit
                            break 'inner;
                        }
                        if let Some(sp_prob) = any.downcast_mut::<SPProblem>() {
                            let new_width = sp_prob.strip_width() * 1.1;
                            info!(
                                "[LBF] no placement found, extending strip width by 10% to {:.3}",
                                new_width
                            );
                            sp_prob.modify_strip_in_back(new_width);
                        }
                    }
                }
            }
        }

        if let Some(sp_prob) = (&mut self.problem as &mut dyn Any).downcast_mut::<SPProblem>() {
            sp_prob.fit_strip();
            info!("[LBF] fitted strip width to {:.3}", sp_prob.strip_width());
        }

        let solution = self.problem.create_solution();

        info!(
            "[LBF] optimization finished in {:.3}ms ({} samples)",
            start.elapsed().as_secs_f64() * 1000.0,
            self.sample_counter.separate_with_commas()
        );

        // info!(
        //     "[LBF] solution contains {} items with a usage of {:.3}%",
        //     solution.n_items_placed(),
        //     solution.usage() * 100.0
        // );
        solution
    }
}

pub fn find_lbf_placement(
    problem: &impl Problem,
    item: &Item,
    config: &LBFConfig,
    rng: &mut impl Rng,
    sample_counter: &mut usize,
) -> Option<Placement> {
    //search all existing layouts and closed bins with remaining stock
    let open_layouts = problem.layouts().map(|(lk, _)| LayoutId::Open(lk));
    let bins_with_stock = problem
        .bin_qtys()
        .iter()
        .enumerate()
        .filter_map(|(bin_id, qty)| match *qty > 0 {
            true => Some(LayoutId::Closed { bin_id }),
            false => None,
        });

    //sequential search until a valid placement is found
    for layout_id in open_layouts.chain(bins_with_stock) {
        debug!("searching in layout {:?}", layout_id);
        if let Some(placing_opt) = sample_layout(problem, layout_id, item, config, rng, sample_counter)
        {
            return Some(placing_opt);
        }
    }
    None
}

pub fn sample_layout(
    problem: &impl Problem,
    layout_id: LayoutId,
    item: &Item,
    config: &LBFConfig,
    rng: &mut impl Rng,
    sample_counter: &mut usize,
) -> Option<Placement> {
    let cde = match layout_id {
        LayoutId::Open(lkey) => problem.layout(lkey).cde(),
        LayoutId::Closed { bin_id } => problem.instance().bin(bin_id).base_cde.as_ref(),
    };
    let irrel_hazards = match item.hazard_filter.as_ref() {
        None => vec![],
        Some(hf) => hazard_filter::generate_irrelevant_hazards(hf, cde.all_hazards()),
    };

    let surrogate = item.shape.surrogate();
    //create a clone of the shape which will we can use to apply the transformations
    let mut buffer = {
        let mut buffer = (*item.shape).clone();
        buffer.surrogate = None; //strip the surrogate for faster transforms, we don't need it for the buffer shape
        buffer
    };

    let mut best: Option<(Placement, LBFPlacingCost)> = None;

    //calculate the number of uniform and local search samples
    let ls_sample_budget = (config.n_samples as f32 * config.ls_frac) as usize;
    let uni_sample_budget = config.n_samples - ls_sample_budget;

    //uniform sampling within the valid cells of the Hazard Proximity Grid, tracking the best valid insertion option
    let mut hpg_sampler = HPGSampler::new(item, cde)?;

    for i in 0..uni_sample_budget {
        let transform = hpg_sampler.sample(rng);
        if !cde.surrogate_collides(surrogate, &transform, &irrel_hazards) {
            //if no collision is detected on the surrogate, apply the transformation
            buffer.transform_from(&item.shape, &transform);
            let cost = LBFPlacingCost::from_shape(&buffer);

            //only validate the sample if it possibly can replace the current best
            let worth_testing = match (best.as_ref(), &cost) {
                (Some((_, best_cost)), cost) => {
                    cost.partial_cmp(best_cost).unwrap() == Ordering::Less
                }
                (None, _) => true,
            };

            if worth_testing && !cde.poly_collides(&buffer, &irrel_hazards) {
                //sample is valid and improves on the current best
                let p_opt = Placement {
                    layout_id,
                    item_id: item.id,
                    d_transf: transform.decompose(),
                };
                hpg_sampler.tighten(cost);
                debug!(
                    "[UNI: {i}/{uni_sample_budget}] better: {} ",
                    &p_opt.d_transf
                );

                best = Some((p_opt, cost));
            }
        }
    }

    *sample_counter += hpg_sampler.n_samples;

    //if a valid sample was found during the uniform sampling, perform local search around it
    let (best_opt, best_cost) = best.as_mut()?;

    /*
    The local search samples in a normal distribution.
    Throughout the course of the local search, the mean of the distribution is updated to the best found sample.
    And the standard deviation tightens, to focus the search around the best sample.
     */

    let mut ls_sampler = LSSampler::from_defaults(item, &best_opt.d_transf, cde.bbox());

    for i in 0..ls_sample_budget {
        let d_transf = ls_sampler.sample(rng);
        let transf = d_transf.compose();
        if !cde.surrogate_collides(surrogate, &transf, &irrel_hazards) {
            buffer.transform_from(&item.shape, &transf);
            let cost = LBFPlacingCost::from_shape(&buffer);

            //only validate the sample if it possibly can replace the current best
            let worth_testing = cost < *best_cost;

            if worth_testing && !cde.poly_collides(&buffer, &irrel_hazards) {
                //sample is valid and improves on the current best
                let p_opt = Placement {
                    layout_id: layout_id,
                    item_id: item.id,
                    d_transf,
                };
                ls_sampler.shift_mean(&p_opt.d_transf);
                debug!("[LS: {i}/{ls_sample_budget}] better: {}", &p_opt.d_transf);
                (*best_opt, *best_cost) = (p_opt, cost);
            }
        }
        let progress_pct = i as fsize / ls_sample_budget as fsize;
        ls_sampler.decay_stddev(progress_pct);
    }

    *sample_counter += ls_sampler.n_samples;

    best.map(|(p_opt, _)| p_opt)
}
