use std::cmp::{Ordering, Reverse};
use std::time::Instant;

use itertools::Itertools;
use log::{debug, info};
use ordered_float::NotNan;
use rand::prelude::SmallRng;
use rand::Rng;
use thousands::Separable;

use jagua_rs::collision_detection::hazard_filter;
use jagua_rs::entities::instances::instance::Instance;
use jagua_rs::entities::instances::instance_generic::InstanceGeneric;
use jagua_rs::entities::item::Item;
use jagua_rs::entities::layout::Layout;
use jagua_rs::entities::placing_option::PlacingOption;
use jagua_rs::entities::problems::bin_packing::BPProblem;
use jagua_rs::entities::problems::problem::Problem;
use jagua_rs::entities::problems::problem_generic::{LayoutIndex, ProblemGeneric};
use jagua_rs::entities::problems::strip_packing::SPProblem;
use jagua_rs::entities::solution::Solution;
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

pub struct LBFOptimizer {
    pub instance: Instance,
    pub problem: Problem,
    pub config: LBFConfig,
    /// SmallRng is a fast, non-cryptographic PRNG <https://rust-random.github.io/book/guide-rngs.html>
    pub rng: SmallRng,
    pub sample_counter: usize,
}

impl LBFOptimizer {
    pub fn new(instance: Instance, config: LBFConfig, rng: SmallRng) -> Self {
        assert!(config.n_samples > 0);
        let problem = match instance.clone() {
            Instance::BP(bpi) => BPProblem::new(bpi.clone()).into(),
            Instance::SP(spi) => {
                let strip_width = instance.item_area() * 2.0 / spi.strip_height; //initiate with 50% usage
                SPProblem::new(spi.clone(), strip_width, config.cde_config).into()
            }
        };

        Self {
            instance,
            problem,
            config,
            rng,
            sample_counter: 0,
        }
    }

    pub fn solve(&mut self) -> Solution {
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
            while self.problem.missing_item_qtys()[item_index] > 0 {
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
                        match &mut self.problem {
                            Problem::BP(_) => break,
                            Problem::SP(sp_problem) => {
                                let new_width = sp_problem.strip_width() * 1.1;
                                info!("[LBF] no placement found, extending strip width by 10% to {:.3}", new_width);
                                sp_problem.modify_strip_in_back(new_width);
                            }
                        }
                    }
                }
            }
        }
        match &mut self.problem {
            Problem::BP(_) => {}
            Problem::SP(sp_problem) => {
                sp_problem.fit_strip();
                info!(
                    "[LBF] fitted strip width to {:.3}",
                    sp_problem.strip_width()
                );
            }
        }

        let solution: Solution = self.problem.create_solution(None);

        info!(
            "[LBF] optimization finished in {:.3}ms ({} samples)",
            start.elapsed().as_secs_f64() * 1000.0,
            self.sample_counter.separate_with_commas()
        );

        info!(
            "[LBF] solution contains {} items with a usage of {:.3}%",
            solution.n_items_placed(),
            solution.usage * 100.0
        );
        solution
    }
}

pub fn find_lbf_placement(
    problem: &Problem,
    item: &Item,
    config: &LBFConfig,
    rng: &mut impl Rng,
    sample_counter: &mut usize,
) -> Option<PlacingOption> {
    //search all existing layouts and template layouts with remaining stock
    let existing_layouts = problem.layout_indices();
    let template_layouts = problem.template_layout_indices_with_stock();

    //sequential search until a valid placement is found
    for layout in existing_layouts.chain(template_layouts) {
        debug!("searching in layout {:?}", layout);
        if let Some(placing_opt) = sample_layout(problem, layout, item, config, rng, sample_counter)
        {
            return Some(placing_opt);
        }
    }
    None
}

pub fn sample_layout(
    problem: &Problem,
    layout_idx: LayoutIndex,
    item: &Item,
    config: &LBFConfig,
    rng: &mut impl Rng,
    sample_counter: &mut usize,
) -> Option<PlacingOption> {
    let layout: &Layout = problem.get_layout(layout_idx);
    let cde = layout.cde();
    let irrel_hazards = match item.hazard_filter.as_ref() {
        None => vec![],
        Some(hf) => hazard_filter::generate_irrelevant_hazards(hf, layout.cde().all_hazards()),
    };

    let surrogate = item.shape.surrogate();
    //create a clone of the shape which will we can use to apply the transformations
    let mut buffer = {
        let mut buffer = (*item.shape).clone();
        buffer.surrogate = None; //strip the surrogate for faster transforms, we don't need it for the buffer shape
        buffer
    };

    let mut best: Option<(PlacingOption, LBFPlacingCost)> = None;

    //calculate the number of uniform and local search samples
    let ls_sample_budget = (config.n_samples as f32 * config.ls_frac) as usize;
    let uni_sample_budget = config.n_samples - ls_sample_budget;

    //uniform sampling within the valid cells of the Hazard Proximity Grid, tracking the best valid insertion option
    let mut hpg_sampler = HPGSampler::new(item, layout)?;

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
                let p_opt = PlacingOption {
                    layout_idx,
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
    The local search samplers in a normal distribution.
    Throughout the course of the local search, the mean of the distribution is updated to the best found sample.
    And the standard deviation tightens, to focus the search around the best sample.
     */

    let mut ls_sampler = LSSampler::from_defaults(item, &best_opt.d_transf, &layout.bin().bbox());

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
                let p_opt = PlacingOption {
                    layout_idx,
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
