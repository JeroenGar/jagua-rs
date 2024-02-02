use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::SmallRng;
use rand::SeedableRng;
use rand::seq::IteratorRandom;

use jaguars::entities::instance::{Instance, PackingType};
use jaguars::entities::placed_item::PlacedItemUID;
use jaguars::entities::placing_option::PlacingOption;
use jaguars::entities::problems::problem::{LayoutIndex, Problem, ProblemEnum};
use jaguars::entities::problems::sp_problem::SPProblem;
use jaguars::geometry::geo_traits::TransformableFrom;
use jaguars::io::json_instance::JsonInstance;
use jaguars::io::parser::Parser;
use jaguars::util::config::{CDEConfig, HazProxConfig, QuadTreeConfig, SPSurrogateConfig};
use jaguars::util::polygon_simplification::PolySimplConfig;
use lbf::config::Config;
use lbf::lbf_optimizer::LBFOptimizer;
use lbf::samplers::hpg_sampler::HPGSampler;
use lbf::samplers::uniform_rect_sampler::UniformAARectSampler;

criterion_main!(benches);
criterion_group!(benches, quadtree_query_update_1000_1, quadtree_query_bench, quadtree_update_bench);

const SWIM_PATH: &str = "../assets/swim.json";
const N_ITEMS_REMOVED: usize = 5;

const QT_DEPTHS: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

/// Benchmark the update operation of the quadtree for different depths
/// From a solution, created by the LBF optimizer, 5 items are removed and then inserted back again
fn quadtree_update_bench(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = Config {
        cde_config: CDEConfig {
            quadtree: QuadTreeConfig::FixedDepth(0),
            haz_prox: HazProxConfig::Enabled { n_cells: 1 },
            item_surrogate_config: SPSurrogateConfig {
                pole_coverage_goal: 0.0,
                max_poles: 0,
                n_ff_poles: 0,
                n_ff_piers: 0,
            },
        },
        poly_simpl_config: PolySimplConfig::Disabled,
        ..Config::default()
    };

    let mut group = c.benchmark_group("quadtree_update");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree = QuadTreeConfig::FixedDepth(depth);
        let instance = create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let mut problem = create_blf_problem(instance.clone(), config);

        let layout_index = LayoutIndex::Existing(0);
        let mut rng = SmallRng::seed_from_u64(0);

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let mut selected_pi_uids: [Option<PlacedItemUID>; N_ITEMS_REMOVED] = <[_; N_ITEMS_REMOVED]>::default();
                // Remove 5 items from the layout
                problem.get_layout(&layout_index).placed_items().iter()
                    .map(|p_i| Some(p_i.uid().clone()))
                    .choose_multiple_fill(&mut rng, &mut selected_pi_uids);

                for pi_uid in selected_pi_uids.iter().flatten() {
                    //println!("Removing item with id: {}\n", pi_uid.item_id);
                    problem.remove_item(layout_index, pi_uid);
                }

                // Insert 5 items back into the layout
                for pi_uid in selected_pi_uids.into_iter().flatten() {
                    //println!("Inserting item with id: {}\n", pi_uid.item_id);
                    problem.insert_item(&PlacingOption {
                        layout_index,
                        item_id: pi_uid.item_id,
                        transf: pi_uid.d_transf.compose(),
                        d_transf: pi_uid.d_transf,
                    })
                }
            })
        });
    }
    group.finish();
}

/// Benchmark the query operation of the quadtree for different depths
/// We use the HPG sampler to sample 1000 transformations for each of the 5 removed items
fn quadtree_query_bench(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = Config {
        cde_config: CDEConfig {
            quadtree: QuadTreeConfig::FixedDepth(0),
            haz_prox: HazProxConfig::Enabled { n_cells: 1 },
            item_surrogate_config: SPSurrogateConfig {
                pole_coverage_goal: 0.0,
                max_poles: 0,
                n_ff_poles: 0,
                n_ff_piers: 0,
            },
        },
        poly_simpl_config: PolySimplConfig::Disabled,
        ..Config::default()
    };

    let mut group = c.benchmark_group("quadtree_query");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree = QuadTreeConfig::FixedDepth(depth);
        let instance = create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let mut problem = create_blf_problem(instance.clone(), config);

        let layout_index = LayoutIndex::Existing(0);
        let mut rng = SmallRng::seed_from_u64(0);
        let mut selected_pi_uids: [Option<PlacedItemUID>; N_ITEMS_REMOVED] = <[_; N_ITEMS_REMOVED]>::default();
        // Remove 5 items from the layout
        problem.get_layout(&layout_index).placed_items().iter()
            .map(|p_i| Some(p_i.uid().clone()))
            .choose_multiple_fill(&mut rng, &mut selected_pi_uids);

        for pi_uid in selected_pi_uids.iter().flatten() {
            //println!("Removing item with id: {}\n", pi_uid.item_id);
            problem.remove_item(layout_index, pi_uid);
        }
        problem.flush_changes();

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                for item_id in selected_pi_uids.iter().flatten().map(|pi_uid| pi_uid.item_id) {
                    let item = instance.item(item_id);
                    let layout = problem.get_layout(&layout_index);
                    let mut buffer_shape = item.shape().clone();
                    let sampler = UniformAARectSampler::new(layout.bin().bbox(), item);
                    for _ in 0..1000 {
                        let transf = sampler.sample(&mut rng);
                        buffer_shape.transform_from(item.shape(), &transf.compose());
                        let collides = layout.cde().poly_collides(&buffer_shape, &[]);
                        criterion::black_box(collides); // prevent the compiler from optimizing the loop away
                    }
                }
            })
        });
    }
    group.finish();
}

fn quadtree_query_update_1000_1(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = Config {
        cde_config: CDEConfig {
            quadtree: QuadTreeConfig::FixedDepth(0),
            haz_prox: HazProxConfig::Enabled { n_cells: 1 },
            item_surrogate_config: SPSurrogateConfig {
                pole_coverage_goal: 0.0,
                max_poles: 0,
                n_ff_poles: 0,
                n_ff_piers: 0,
            },
        },
        poly_simpl_config: PolySimplConfig::Disabled,
        ..Config::default()
    };

    let mut group = c.benchmark_group("quadtree_query_update_1000_1");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree = QuadTreeConfig::FixedDepth(depth);
        let instance = create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let mut problem = create_blf_problem(instance.clone(), config);

        let layout_index = LayoutIndex::Existing(0);
        let mut rng = SmallRng::seed_from_u64(0);
        let mut selected_pi_uids: [Option<PlacedItemUID>; N_ITEMS_REMOVED] = <[_; N_ITEMS_REMOVED]>::default();
        // Remove 5 items from the layout
        problem.get_layout(&layout_index).placed_items().iter()
            .map(|p_i| Some(p_i.uid().clone()))
            .choose_multiple_fill(&mut rng, &mut selected_pi_uids);

        for pi_uid in selected_pi_uids.iter().flatten() {
            //println!("Removing item with id: {}\n", pi_uid.item_id);
            problem.remove_item(layout_index, pi_uid);
        }
        problem.flush_changes();

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let mut selected_pi_uids: [Option<PlacedItemUID>; N_ITEMS_REMOVED] = <[_; N_ITEMS_REMOVED]>::default();
                // Remove 5 items from the layout
                problem.get_layout(&layout_index).placed_items().iter()
                    .map(|p_i| Some(p_i.uid().clone()))
                    .choose_multiple_fill(&mut rng, &mut selected_pi_uids);

                for pi_uid in selected_pi_uids.iter().flatten() {
                    //println!("Removing item with id: {}\n", pi_uid.item_id);
                    problem.remove_item(layout_index, pi_uid);
                }

                problem.flush_changes();

                for item_id in selected_pi_uids.iter().flatten().map(|pi_uid| pi_uid.item_id) {
                    let item = instance.item(item_id);
                    let layout = problem.get_layout(&layout_index);
                    let mut buffer_shape = item.shape().clone();
                    let sampler = UniformAARectSampler::new(layout.bin().bbox(), item);
                    for _ in 0..1000 {
                        let transf = sampler.sample(&mut rng);
                        buffer_shape.transform_from(item.shape(), &transf.compose());
                        let collides = layout.cde().poly_collides(&buffer_shape, &[]);
                        criterion::black_box(collides); // prevent the compiler from optimizing the loop away
                    }
                }

                // Insert 5 items back into the layout
                for pi_uid in selected_pi_uids.into_iter().flatten() {
                    //println!("Inserting item with id: {}\n", pi_uid.item_id);
                    problem.insert_item(&PlacingOption {
                        layout_index,
                        item_id: pi_uid.item_id,
                        transf: pi_uid.d_transf.compose(),
                        d_transf: pi_uid.d_transf,
                    })
                }
            })
        });
    }
    group.finish();
}

fn create_instance(json_instance: &JsonInstance, cde_config: CDEConfig, poly_simpl_config: PolySimplConfig) -> Arc<Instance> {
    let parser = Parser::new(poly_simpl_config, cde_config, true);
    Arc::new(parser.parse(json_instance))
}

fn create_blf_problem(instance: Arc<Instance>, config: Config) -> SPProblem {
    assert!(matches!(instance.packing_type(), PackingType::StripPacking {..}));
    let rng = SmallRng::seed_from_u64(0);
    let mut lbf_optimizer = LBFOptimizer::new(instance, config, rng);
    lbf_optimizer.solve();

    match lbf_optimizer.problem().clone() {
        ProblemEnum::SPProblem(sp_problem) => sp_problem,
        _ => panic!("Expected SPProblem")
    }
}
