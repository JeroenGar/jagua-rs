use itertools::Itertools;
use jagua_rs::collision_detection::CDEConfig;
use jagua_rs::entities::general::Instance;
use jagua_rs::entities::strip_packing::SPPlacement;
use jagua_rs::entities::strip_packing::{SPInstance, SPProblem};
use jagua_rs::geometry::fail_fast::SPSurrogateConfig;
use jagua_rs::io::json_instance::JsonInstance;
use jagua_rs::io::parser::Parser;
use lbf::config::LBFConfig;
use lbf::io;
use lbf::io::svg_util::SvgDrawOptions;
use lbf::opt::lbf_opt_spp::LBFOptimizerSP;
use log::info;
use rand::SeedableRng;
use rand::prelude::{IteratorRandom, SmallRng};
use std::any::Any;
use std::path::Path;

pub const SWIM_PATH: &str = "../assets/swim.json";
pub const N_ITEMS_REMOVED: usize = 5;

pub fn create_instance(
    json_instance: &JsonInstance,
    cde_config: CDEConfig,
    poly_simpl_tolerance: Option<f32>,
) -> SPInstance {
    let parser = Parser::new(cde_config, poly_simpl_tolerance, None);
    let instance = parser.parse(json_instance);
    (instance.as_ref() as &dyn Any)
        .downcast_ref::<SPInstance>()
        .expect("Expected SPInstance")
        .clone()
}

/// Creates a Strip Packing Problem, fill the layout using with the LBF Optimizer and removes some items from the layout
/// Returns the problem and the removed items
/// Simulates a common scenario in iterative optimization algorithms: dense packing with a few items removed
pub fn create_lbf_problem(
    instance: SPInstance,
    config: LBFConfig,
    n_items_removed: usize,
) -> (SPProblem, Vec<SPPlacement>) {
    let mut lbf_optimizer =
        LBFOptimizerSP::new(instance.clone(), config, SmallRng::seed_from_u64(0));
    lbf_optimizer.solve();

    let mut problem = lbf_optimizer.problem;

    let mut rng = SmallRng::seed_from_u64(0);
    // Remove some items from the layout
    let placed_items_to_remove = problem
        .layout
        .placed_items()
        .iter()
        .map(|(k, _)| k)
        .choose_multiple(&mut rng, n_items_removed);

    let p_opts = placed_items_to_remove
        .iter()
        .map(|k| {
            let pi = &problem.layout.placed_items()[*k];
            SPPlacement {
                item_id: pi.item_id,
                d_transf: pi.d_transf,
            }
        })
        .collect_vec();

    for pkey in placed_items_to_remove {
        let item_id = problem.layout.placed_items()[pkey].item_id;
        problem.remove_item(pkey, true);
        info!(
            "Removed item: {} with {} edges",
            item_id,
            lbf_optimizer.instance.item(item_id).shape_cd.n_vertices()
        );
    }

    {
        let draw_options = SvgDrawOptions {
            quadtree: true,
            surrogate: true,
            ..SvgDrawOptions::default()
        };
        let svg = io::layout_to_svg::layout_to_svg(&problem.layout, &instance, draw_options);
        io::write_svg(&svg, Path::new("bench_layout.svg"));
    }

    (problem, p_opts)
}

pub fn create_base_config() -> LBFConfig {
    LBFConfig {
        cde_config: CDEConfig {
            quadtree_depth: 5,
            item_surrogate_config: SPSurrogateConfig {
                n_pole_limits: [(100, 0.0), (20, 0.75), (10, 0.90)],
                n_ff_poles: 4,
                n_ff_piers: 0,
            },
        },
        poly_simpl_tolerance: Some(0.001),
        min_item_separation: None,
        prng_seed: Some(0),
        n_samples: 5000,
        ls_frac: 0.2,
        svg_draw_options: Default::default(),
    }
}
