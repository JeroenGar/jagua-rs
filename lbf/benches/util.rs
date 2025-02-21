use itertools::Itertools;
use log::info;
use rand::SeedableRng;
use rand::prelude::{IteratorRandom, SmallRng};
use std::path::Path;

use jagua_rs::entities::instances::instance::Instance;
use jagua_rs::entities::instances::instance_generic::InstanceGeneric;
use jagua_rs::entities::placing_option::PlacingOption;
use jagua_rs::entities::problems::problem::Problem;
use jagua_rs::entities::problems::problem_generic::{ProblemGeneric, STRIP_LAYOUT_IDX};
use jagua_rs::entities::problems::strip_packing::SPProblem;
use jagua_rs::fsize;
use jagua_rs::io::json_instance::JsonInstance;
use jagua_rs::io::parser::Parser;
use jagua_rs::util::config::{CDEConfig, SPSurrogateConfig};
use jagua_rs::util::polygon_simplification::PolySimplConfig;
use lbf::io;
use lbf::io::svg_util::SvgDrawOptions;
use lbf::lbf_config::LBFConfig;
use lbf::lbf_optimizer::LBFOptimizer;

pub const SWIM_PATH: &str = "../assets/swim.json";
pub const N_ITEMS_REMOVED: usize = 5;

pub fn create_instance(
    json_instance: &JsonInstance,
    cde_config: CDEConfig,
    poly_simpl_tolerance: Option<fsize>,
) -> Instance {
    let poly_simpl_config = match poly_simpl_tolerance {
        Some(tolerance) => PolySimplConfig::Enabled { tolerance },
        None => PolySimplConfig::Disabled,
    };
    let parser = Parser::new(poly_simpl_config, cde_config, true);
    parser.parse(json_instance)
}

/// Creates a Strip Packing Problem, fill the layout using with the LBF Optimizer and removes some items from the layout
/// Returns the problem and the removed items
/// Simulates a common scenario in iterative optimization algorithms: dense packing with a few items removed
pub fn create_blf_problem(
    instance: Instance,
    config: LBFConfig,
    n_items_removed: usize,
) -> (SPProblem, Vec<PlacingOption>) {
    assert!(matches!(&instance, &Instance::SP(_)));
    let mut lbf_optimizer = LBFOptimizer::new(instance.clone(), config, SmallRng::seed_from_u64(0));
    lbf_optimizer.solve();

    let mut problem = match lbf_optimizer.problem.clone() {
        Problem::SP(sp_problem) => sp_problem,
        _ => panic!("Expected SPProblem"),
    };

    let mut rng = SmallRng::seed_from_u64(0);
    // Remove some items from the layout
    let placed_items_to_remove = problem
        .get_layout(&STRIP_LAYOUT_IDX)
        .placed_items()
        .iter()
        .map(|(k, _)| k)
        .choose_multiple(&mut rng, n_items_removed);

    let p_opts = placed_items_to_remove
        .iter()
        .map(|k| {
            let pi = &problem.layout.placed_items()[*k];
            PlacingOption {
                layout_idx: STRIP_LAYOUT_IDX,
                item_id: pi.item_id,
                d_transf: pi.d_transf,
            }
        })
        .collect_vec();

    for pik in placed_items_to_remove {
        let item_id = problem.layout.placed_items()[pik].item_id;
        problem.remove_item(STRIP_LAYOUT_IDX, pik, true);
        info!(
            "Removed item: {} with {} edges",
            item_id,
            lbf_optimizer
                .instance
                .item(item_id)
                .shape
                .number_of_points()
        );
    }
    problem.flush_changes();

    {
        let draw_options = SvgDrawOptions {
            quadtree: true,
            surrogate: true,
            haz_prox_grid: false,
            ..SvgDrawOptions::default()
        };
        let svg = io::layout_to_svg::layout_to_svg(
            problem.get_layout(&STRIP_LAYOUT_IDX),
            &instance,
            draw_options,
        );
        io::write_svg(&svg, Path::new("bench_layout.svg"));
    }

    (problem, p_opts)
}

pub fn create_base_config() -> LBFConfig {
    LBFConfig {
        cde_config: CDEConfig {
            quadtree_depth: 5,
            hpg_n_cells: 2000,
            item_surrogate_config: SPSurrogateConfig {
                pole_coverage_goal: 0.9,
                max_poles: 10,
                n_ff_poles: 4,
                n_ff_piers: 0,
            },
        },
        poly_simpl_tolerance: Some(0.001),
        prng_seed: Some(0),
        n_samples: 5000,
        ls_frac: 0.2,
        svg_draw_options: Default::default(),
    }
}
