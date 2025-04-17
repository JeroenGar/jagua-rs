use std::time::Instant;

use crate::config::LBFConfig;
use crate::opt::ITEM_LIMIT;
use crate::opt::search::{item_placement_order, search};
use jagua_rs::collision_detection::hazards::filter::NoHazardFilter;
use jagua_rs::entities::general::Instance;
use jagua_rs::entities::strip_packing::SPPlacement;
use jagua_rs::entities::strip_packing::{SPInstance, SPProblem, SPSolution};
use log::info;
use rand::prelude::SmallRng;
use thousands::Separable;

/// Left-Bottom-Fill (LBF) optimizer for Strip Packing problems.
pub struct LBFOptimizerSP {
    pub instance: SPInstance,
    pub problem: SPProblem,
    pub config: LBFConfig,
    /// SmallRng is a fast, non-cryptographic PRNG <https://rust-random.github.io/book/guide-rngs.html>
    pub rng: SmallRng,
    pub sample_counter: usize,
}

impl LBFOptimizerSP {
    pub fn new(instance: SPInstance, config: LBFConfig, rng: SmallRng) -> Self {
        assert!(config.n_samples > 0);
        let strip_width = instance.item_area * 2.0 / instance.strip_height; //initiate with 50% usage
        let problem = SPProblem::new(instance.clone(), strip_width, config.cde_config).into();
        Self {
            instance,
            problem,
            config,
            rng,
            sample_counter: 0,
        }
    }

    pub fn solve(&mut self) -> SPSolution {
        let start = Instant::now();

        'outer: for item_index in item_placement_order(&self.instance) {
            let item = &self.instance.items()[item_index].0;
            //place all items of this type
            while self.problem.missing_item_qtys[item_index] > 0 {
                let placement = match &item.hazard_filter {
                    None => search(
                        &self.problem.layout.cde(),
                        item,
                        &self.config,
                        &mut self.rng,
                        &mut self.sample_counter,
                        &NoHazardFilter,
                    ),
                    Some(hf) => search(
                        &self.problem.layout.cde(),
                        item,
                        &self.config,
                        &mut self.rng,
                        &mut self.sample_counter,
                        hf,
                    ),
                };

                match placement {
                    Some((d_transf, _)) => {
                        self.problem.place_item(SPPlacement {
                            item_id: item.id,
                            d_transf,
                        });
                        info!(
                            "[LBF] placing item {}/{} with id {} at [{}]",
                            self.problem.layout.placed_items().len(),
                            self.instance.total_item_qty(),
                            item.id,
                            d_transf,
                        );
                        #[allow(clippy::absurd_extreme_comparisons)]
                        if self.problem.layout.placed_items().len() >= ITEM_LIMIT {
                            break 'outer;
                        }
                    }
                    None => {
                        // item does not fit anywhere, increase the strip width
                        self.problem
                            .change_strip_width(self.problem.strip_width() * 1.1);
                        info!(
                            "[LBF] no placement found, extended strip by 10% to {:.3}",
                            self.problem.strip_width()
                        );
                    }
                }
            }
        }

        self.problem.fit_strip();
        info!(
            "[LBF] fitted strip width to {:.3}",
            self.problem.strip_width()
        );

        let solution = self.problem.save();

        info!(
            "[LBF] optimization finished in {:.3}ms ({} samples)",
            start.elapsed().as_secs_f64() * 1000.0,
            self.sample_counter.separate_with_commas()
        );

        info!(
            "[LBF] solution contains {} items with a density of {:.3}%",
            solution.layout_snapshot.placed_items.len(),
            solution.density(&self.instance) * 100.0
        );
        solution
    }
}
