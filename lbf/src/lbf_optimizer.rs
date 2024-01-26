use std::cmp::{Ordering, Reverse};
use std::f64::consts::PI;
use std::sync::Arc;
use std::time::Instant;

use itertools::Itertools;
use log::{info};
use ordered_float::NotNan;
use rand::prelude::SmallRng;

use jaguars::collision_detection::hazards::filters::hazard_filter;
use jaguars::entities::instance::{Instance, PackingType};
use jaguars::entities::item::Item;
use jaguars::entities::problems::bp_problem::BPProblem;
use jaguars::entities::problems::problem::{LayoutIndex, Problem, ProblemEnum};

use jaguars::entities::problems::sp_problem::SPProblem;
use jaguars::entities::solution::Solution;
use jaguars::geometry::convex_hull::convex_hull_from_points;
use jaguars::geometry::geo_traits::{Shape, TransformableFrom};
use jaguars::geometry::primitives::simple_polygon::SimplePolygon;
use jaguars::insertion::insertion_option::InsertionOption;
use crate::config::Config;
use crate::lbf_cost::LBFCost;

use crate::samplers::hpg_sampler::HPGSampler;
use crate::samplers::ls_sampler::LSSampler;

pub const STDDEV_TRANSL_START_FRAC: f64 = 0.01;
pub const STDDEV_TRANSL_END_FRAC: f64 = 0.001;
pub const STDDEV_ROT_START_FRAC: f64 = 2.0 * (PI / 180.0);
pub const STDDEV_ROT_END_FRAC: f64 = 0.5 * (PI / 180.0);

pub const ITEM_LIMIT : usize = 1000;

pub struct LBFOptimizer {
    instance: Arc<Instance>,
    problem: ProblemEnum,
    config: Config,
    rng: SmallRng,
}

impl LBFOptimizer {
    pub fn new(instance: Arc<Instance>, config: Config, rng: SmallRng) -> Self {
        let problem = match instance.packing_type() {
            PackingType::BinPacking(_) => BPProblem::new(instance.clone()).into(),
            PackingType::StripPacking { height } => {
                let strip_width = instance.item_area() / height; //initiate with usage 100%
                SPProblem::new(instance.clone(), strip_width, config.cde_config).into()
            }
        };

        Self {
            instance,
            problem,
            config,
            rng,
        }
    }

    pub fn solve(&mut self) -> Solution {
        let start = Instant::now();
        //sort the items by descending diameter of convex hull
        let sorted_item_indices = (0..self.instance.items().len())
            .sorted_by_cached_key(|i| {
                let item = &self.instance.items()[*i].0;
                let ch = SimplePolygon::new(
                    convex_hull_from_points(item.shape().points().clone())
                );
                let ch_diam = NotNan::new(ch.diameter()).expect("convex hull diameter is NaN");
                Reverse(ch_diam)
            })
            .collect_vec();

        'outer: for item_index in sorted_item_indices {
            let item = &self.instance.items()[item_index].0;
            //place all items of this type
            while self.problem.missing_item_qtys()[item_index] > 0 {
                //find a position and insert it
                match find_placement(&self.problem, item, &self.config, &mut self.rng){
                    Some(i_opt) => {
                        info!("inserting item {}", i_opt.item_id());
                        self.problem.insert_item(&i_opt);
                        if self.problem.included_item_qtys().iter().sum::<usize>() >= ITEM_LIMIT {
                            break 'outer;
                        }
                    },
                    None => {
                        match &mut self.problem{
                            ProblemEnum::BPProblem(_) => break,
                            ProblemEnum::SPProblem(sp_problem) => {
                                let new_width = sp_problem.strip_width() * 1.1;
                                info!("extending the strip by 10%: {:.3}", new_width);
                                sp_problem.modify_strip_width(new_width);
                            }
                        }
                    }
                }
            }
        }

        match &mut self.problem{
            ProblemEnum::BPProblem(_) => {},
            ProblemEnum::SPProblem(sp_problem) => {
                sp_problem.fit_strip_width();
                info!("final strip width: {:.3}", sp_problem.strip_width());
            }
        }

        let solution = self.problem.create_solution(&None);

        info!("BLFOptimizer finished in {}ms, usage: {:.3}%", start.elapsed().as_millis(), solution.usage() * 100.0);
        solution
    }


    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }
    pub fn problem(&self) -> &ProblemEnum {
        &self.problem
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
}

fn find_placement(problem: &ProblemEnum, item: &Item, config: &Config, rng: &mut SmallRng) -> Option<InsertionOption> {
    let layouts_to_sample =
        (0..problem.layouts().len()).map(|i| (LayoutIndex::Existing(i)))
            .chain((0..problem.empty_layouts().len())
                    .filter(|&i| problem.empty_layout_has_stock(i))
                    .map(|i| (LayoutIndex::Empty(i)))
            ).collect_vec();

    let best_i_opt = layouts_to_sample.iter().filter_map(|l_i| {
        sample_layout(problem, l_i, item, config, rng)
    }).next();

    best_i_opt
}

fn sample_layout(problem: &ProblemEnum,layout_index: &LayoutIndex, item: &Item, config: &Config, rng: &mut SmallRng) -> Option<InsertionOption> {
    let layout = problem.get_layout(&layout_index);
    let entities_to_ignore = item.hazard_filter()
        .map_or(None, |hf| hazard_filter::ignored_entities(hf, layout.cde().all_hazards()));

    let shape = item.shape();
    let mut buffer_shape = item.shape().clone();

    let mut best = None;

    let n_ls_samples = (config.n_samples_per_item as f32 * config.ls_samples_fraction) as usize;
    let n_random_samples = config.n_samples_per_item - n_ls_samples;
    dbg!(n_random_samples, n_ls_samples);

    //random sampling within the valid cells of the Hazard Proximity Grid, tracking the best valid insertion option
    match HPGSampler::new(item, layout) {
        None => {},
        Some(mut sampler) => {
            for _ in 0..n_random_samples {
                let sampled_transf = sampler.sample(rng);
                if !layout.cde().surrogate_collides(shape.surrogate(), &sampled_transf, entities_to_ignore.as_ref()) {
                    buffer_shape.transform_from(shape, &sampled_transf);
                    let cost = LBFCost::new(&buffer_shape);

                    //only validate the sample if it possibly can replace the current best
                    let worth_testing = best.as_ref()
                        .map_or(true, |(_, best_cost)| cost.cmp(best_cost) == Ordering::Less);

                    if worth_testing && !layout.cde().poly_collides(&buffer_shape, entities_to_ignore.as_ref()) {
                        //sample is a valid option
                        let d_transf = sampled_transf.decompose();
                        let i_opt = InsertionOption::new(layout_index.clone(), item.id(), sampled_transf, d_transf);
                        sampler.tighten_x_bound(&cost);
                        println!("new random best: {}", &cost);
                        best = Some((i_opt, cost));
                    }
                }
            }
        }
    }

    //add some local search on top to push it into place
    /*if let Some((best_iopt, best_cost)) = &best {
        let ls_sampler = LSSampler::new(item, best_iopt.d_transformation(), )
    }*/

    //local search samples
    if best.is_some() {
        let bbox_max_dim = f64::max(layout.cde().bbox().width(), layout.cde().bbox().height());
        let (stddev_transl_start, stddev_transl_end) = (bbox_max_dim * STDDEV_TRANSL_START_FRAC, bbox_max_dim * STDDEV_TRANSL_END_FRAC);
        let (stddev_rot_start, stddev_rot_end) = (STDDEV_ROT_START_FRAC, STDDEV_ROT_END_FRAC);

        let ref_transformation = best.as_ref().unwrap().0.d_transformation();

        let mut ls_sampler = LSSampler::new(item, ref_transformation, stddev_transl_start, stddev_rot_start);

        // x = 0 => first sample
        // x = 1 => last sample
        // f(0) = start
        // f(1) = end
        // => f(x) = start * (end/start)^(x)
        let calc_stddev = |init: f64, end: f64, pct: f64| init * (end / init).powf(pct);

        for i in 0..n_ls_samples {
            let sampled_transf = ls_sampler.sample(rng);
            if !layout.cde().surrogate_collides(shape.surrogate(), &sampled_transf, entities_to_ignore.as_ref()) {
                buffer_shape.transform_from(shape, &sampled_transf);
                let cost = LBFCost::new(&buffer_shape);

                //only validate the sample if it possibly can replace the current best
                let worth_testing = best.as_ref()
                    .map_or(true, |(_, best_cost)| cost.cmp(best_cost) == Ordering::Less);

                if worth_testing && !layout.cde().poly_collides(&buffer_shape, entities_to_ignore.as_ref()) {
                    //sample is a valid option
                    let d_transf = sampled_transf.decompose();
                    ls_sampler.set_mean(&d_transf);
                    let i_opt = InsertionOption::new(layout_index.clone(), item.id(), sampled_transf, d_transf);
                    println!("new ls best: {}", &cost);
                    best = Some((i_opt, cost));
                }
            }
            let runs_pct = i as f64 / n_ls_samples as f64;

            ls_sampler.set_stddev(
                calc_stddev(stddev_transl_start, stddev_transl_end, runs_pct),
                calc_stddev(stddev_rot_start, stddev_rot_end, runs_pct),
            );
        }
    }

    best.map(|b| b.0)
}


