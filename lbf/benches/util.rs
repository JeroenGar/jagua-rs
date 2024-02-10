use std::path::Path;
use std::sync::Arc;
use criterion::Criterion;
use log::{info, Level, LevelFilter};
use rand::prelude::{IteratorRandom, SmallRng};
use rand::SeedableRng;
use jaguars::entities::instance::{Instance, Containers};
use jaguars::entities::placed_item::PlacedItemUID;
use jaguars::entities::problems::problem::{LayoutIndex, ProblemVariant, Problem};
use jaguars::entities::problems::strip_packing::SPProblem;
use jaguars::io::json_instance::JsonInstance;
use jaguars::io::parser::Parser;
use jaguars::util::config::{CDEConfig, HazProxConfig, SPSurrogateConfig};
use jaguars::util::config::QuadTreeConfig::FixedDepth;
use jaguars::util::polygon_simplification::PolySimplConfig;
use lbf::config::Config;
use lbf::io;
use lbf::io::init_logger;
use lbf::io::svg_util::SvgDrawOptions;
use lbf::lbf_optimizer::LBFOptimizer;

pub const SWIM_PATH: &str = "../assets/swim.json";
pub const N_ITEMS_REMOVED: usize = 5;
pub const N_SAMPLES: usize = 10_000;

pub const N_VALID_SAMPLES: usize = 10_000;

pub fn create_instance(json_instance: &JsonInstance, cde_config: CDEConfig, poly_simpl_config: PolySimplConfig) -> Arc<Instance> {
    let parser = Parser::new(poly_simpl_config, cde_config, true);
    Arc::new(parser.parse(json_instance))
}

/// Creates a Strip Packing Problem, fill the layout using with the LBF Optimizer and removes some items from the layout
/// Returns the problem and the removed items
/// Simulates a common scenario in iterative optimization algorithms: dense packing with a few items removed
pub fn create_blf_problem(instance: Arc<Instance>, config: Config, n_items_removed: usize) -> (SPProblem, Vec<PlacedItemUID>) {
    assert!(matches!(instance.containers(), Containers::Strip {..}));
    let mut lbf_optimizer = LBFOptimizer::new(instance.clone(), config, SmallRng::seed_from_u64(0));
    lbf_optimizer.solve();

    let mut problem = match lbf_optimizer.problem().clone() {
        Problem::SP(sp_problem) => sp_problem,
        _ => panic!("Expected SPProblem")
    };

    let mut rng = SmallRng::seed_from_u64(0);
    let layout_index = LayoutIndex::Existing(0);
    // Remove some items from the layout
    let removed_pi_uids = problem.get_layout(&layout_index).placed_items().iter()
        .map(|p_i| p_i.uid().clone())
        .choose_multiple(&mut rng, n_items_removed);

    for pi_uid in removed_pi_uids.iter() {
        problem.remove_item(layout_index, pi_uid);
        info!("Removed item: {} with {} edges", pi_uid.item_id, lbf_optimizer.instance().item(pi_uid.item_id).shape().number_of_points());
    }
    problem.flush_changes();

    {
        let draw_options = SvgDrawOptions{
            quadtree: true,
            surrogate: false,
            haz_prox_grid: false,
            ..SvgDrawOptions::default()
        };
        let svg = io::layout_to_svg::layout_to_svg(problem.get_layout(&layout_index), &instance, draw_options);
        io::write_svg(&svg, Path::new("bench_layout.svg"));
    }

    (problem, removed_pi_uids)
}

pub fn create_base_config() -> Config {
    Config {
        cde_config: CDEConfig {
            quadtree: FixedDepth(6),
            haz_prox: HazProxConfig::Enabled {n_cells: 5000},
            item_surrogate_config: SPSurrogateConfig {
                pole_coverage_goal: 0.9,
                max_poles: 10,
                n_ff_poles: 2,
                n_ff_piers: 1,
            },
        },
        poly_simpl_config: PolySimplConfig::Disabled,
        deterministic_mode: true,
        n_samples_per_item: 5000,
        ls_samples_fraction: 0.2,
        svg_draw_options: Default::default(),
    }
}