use crate::config::LBFConfig;
use crate::opt::loss::LBFLoss;
use crate::samplers::hpg_sampler::HPGSampler;
use crate::samplers::ls_sampler::LSSampler;
use itertools::Itertools;
use jagua_rs::collision_detection::CDEngine;
use jagua_rs::collision_detection::hazards::filter;
use jagua_rs::entities::general::{Instance, Item};
use jagua_rs::fsize;
use jagua_rs::geometry::DTransformation;
use jagua_rs::geometry::convex_hull::convex_hull_from_surrogate;
use jagua_rs::geometry::geo_traits::{Shape, TransformableFrom};
use jagua_rs::geometry::primitives::SimplePolygon;
use log::debug;
use ordered_float::NotNan;
use rand::Rng;
use std::cmp::{Ordering, Reverse};

/// Search the layout (i.e. CDE) for a valid placement of the item, with minimal loss.
pub fn search(
    cde: &CDEngine,
    item: &Item,
    config: &LBFConfig,
    rng: &mut impl Rng,
    sample_counter: &mut usize,
) -> Option<(DTransformation, LBFLoss)> {
    // hazards to be ignored when checking for collisions
    let irrel_hazards = match item.hazard_filter.as_ref() {
        None => vec![],
        Some(hf) => filter::generate_irrelevant_hazards(hf, cde.all_hazards()),
    };

    let surrogate = item.shape.surrogate();
    //create a clone of the shape which will we can use to apply the transformations
    let mut buffer = {
        let mut buffer = (*item.shape).clone();
        buffer.surrogate = None; //remove the surrogate for faster transforms, we don't need it for the buffer shape
        buffer
    };

    let mut best: Option<(DTransformation, LBFLoss)> = None;

    //calculate the number of uniform and local search samples
    let ls_sample_budget = (config.n_samples as f32 * config.ls_frac) as usize;
    let uni_sample_budget = config.n_samples - ls_sample_budget;

    //uniform sampling within the valid cells of the Hazard Proximity Grid, tracking the best valid insertion option
    let mut hpg_sampler = HPGSampler::new(item, cde)?;

    for i in 0..uni_sample_budget {
        let d_transf = hpg_sampler.sample(rng);
        let transf = d_transf.compose();
        if !cde.surrogate_collides(surrogate, &transf, &irrel_hazards) {
            //if no collision is detected on the surrogate, apply the transformation
            buffer.transform_from(&item.shape, &transf);
            let cost = LBFLoss::from_shape(&buffer);

            //only validate the sample if it possibly can replace the current best
            let worth_testing = match (best.as_ref(), &cost) {
                (Some((_, best_cost)), cost) => {
                    cost.partial_cmp(best_cost).unwrap() == Ordering::Less
                }
                (None, _) => true,
            };

            if worth_testing && !cde.poly_collides(&buffer, &irrel_hazards) {
                //sample is valid and improves on the current best
                hpg_sampler.tighten(cost);
                debug!("[UNI: {i}/{uni_sample_budget}] better: {} ", &d_transf);

                best = Some((d_transf, cost));
            }
        }
    }

    *sample_counter += hpg_sampler.n_samples;

    //if a valid sample was found during the uniform sampling, perform local search around it
    let (best_sample, best_cost) = best.as_mut()?;

    /*
    The local search samples in a normal distribution.
    Throughout the course of the local search, the mean of the distribution is updated to the best found sample.
    And the standard deviation tightens, to focus the search around the best sample.
     */

    let mut ls_sampler = LSSampler::from_defaults(item, *best_sample, cde.bbox());

    for i in 0..ls_sample_budget {
        let d_transf = ls_sampler.sample(rng);
        let transf = d_transf.compose();
        if !cde.surrogate_collides(surrogate, &transf, &irrel_hazards) {
            buffer.transform_from(&item.shape, &transf);
            let cost = LBFLoss::from_shape(&buffer);

            //only validate the sample if it possibly can replace the current best
            let worth_testing = cost < *best_cost;

            if worth_testing && !cde.poly_collides(&buffer, &irrel_hazards) {
                //sample is valid and improves on the current best
                ls_sampler.shift_mean(d_transf);
                debug!("[LS: {i}/{ls_sample_budget}] better: {}", &d_transf);
                (*best_sample, *best_cost) = (d_transf, cost);
            }
        }
        let progress_pct = i as fsize / ls_sample_budget as fsize;
        ls_sampler.decay_stddev(progress_pct);
    }

    *sample_counter += ls_sampler.n_samples;

    best
}

pub fn item_placement_order(instance: &impl Instance) -> Vec<usize> {
    //sort the items by descending diameter of convex hull
    (0..instance.items().len())
        .sorted_by_cached_key(|i| {
            let item = &instance.items()[*i].0;
            let ch = SimplePolygon::new(
                convex_hull_from_surrogate(&item.shape)
                    .expect("items should have a surrogate generated"),
            );
            let ch_diam = NotNan::new(ch.diameter()).expect("convex hull diameter is NaN");
            Reverse(ch_diam)
        })
        .collect_vec()
}
