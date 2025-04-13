use std::time::Instant;

use crate::config::LBFConfig;
use crate::opt::ITEM_LIMIT;
use crate::opt::search::{item_placement_order, search};
use jagua_rs::entities::bin_packing::{BPInstance, BPProblem, BPSolution};
use jagua_rs::entities::bin_packing::{BPLayoutType, BPPlacement};
use jagua_rs::entities::general::{Instance, Item};
use log::{debug, info};
use rand::Rng;
use rand::prelude::SmallRng;
use thousands::Separable;

/// Left-Bottom-Fill (LBF) optimizer for Bin Packing problems.
pub struct LBFOptimizerBP {
    pub instance: BPInstance,
    pub problem: BPProblem,
    pub config: LBFConfig,
    /// SmallRng is a fast, non-cryptographic PRNG <https://rust-random.github.io/book/guide-rngs.html>
    pub rng: SmallRng,
    pub sample_counter: usize,
}

impl LBFOptimizerBP {
    pub fn new(instance: BPInstance, config: LBFConfig, rng: SmallRng) -> Self {
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

    pub fn solve(&mut self) -> BPSolution {
        let start = Instant::now();

        'outer: for item_index in item_placement_order(&self.instance) {
            let item = &self.instance.items()[item_index].0;
            //place all items of this type
            'inner: while self.problem.missing_item_qtys[item_index] > 0 {
                //find a position and insert it
                let placement = search_layouts(
                    &self.problem,
                    item,
                    &self.config,
                    &mut self.rng,
                    &mut self.sample_counter,
                );

                match placement {
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
                    None => break 'inner, // items of this type do not fit anywhere
                }
            }
        }

        let solution = self.problem.save();

        info!(
            "[LBF] optimization finished in {:.3}ms ({} samples)",
            start.elapsed().as_secs_f64() * 1000.0,
            self.sample_counter.separate_with_commas()
        );

        info!(
            "[LBF] solution contains {} items with a usage of {:.3}%",
            solution
                .layout_snapshots
                .values()
                .map(|ls| ls.placed_items.len())
                .sum::<usize>(),
            solution.usage * 100.0
        );
        solution
    }
}

fn search_layouts(
    problem: &BPProblem,
    item: &Item,
    config: &LBFConfig,
    rng: &mut impl Rng,
    sample_counter: &mut usize,
) -> Option<BPPlacement> {
    //search all existing layouts and closed bins with remaining stock
    let open_layouts = problem.layouts.keys().map(|lk| BPLayoutType::Open(lk));
    let bins_with_stock =
        problem
            .bin_qtys
            .iter()
            .enumerate()
            .filter_map(|(bin_id, qty)| match *qty > 0 {
                true => Some(BPLayoutType::Closed { bin_id }),
                false => None,
            });

    //sequential search until a valid placement is found
    for layout_id in open_layouts.chain(bins_with_stock) {
        debug!("searching in layout {:?}", layout_id);
        let cde = match layout_id {
            BPLayoutType::Open(lkey) => problem.layouts[lkey].cde(),
            BPLayoutType::Closed { bin_id } => problem.instance.bin(bin_id).base_cde.as_ref(),
        };

        let placement = search(cde, item, config, rng, sample_counter);

        if let Some(placement) = placement {
            return Some(BPPlacement {
                layout_id,
                item_id: item.id,
                d_transf: placement.0,
            });
        }
    }
    None
}
