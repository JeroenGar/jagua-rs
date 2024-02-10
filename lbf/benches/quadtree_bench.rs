use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use rand::seq::IteratorRandom;

use jaguars::entities::instance::{Instance, PackingType};
use jaguars::entities::placed_item::PlacedItemUID;
use jaguars::entities::placing_option::PlacingOption;
use jaguars::entities::problems::problem::{LayoutIndex, ProblemVariant, Problem};
use jaguars::entities::problems::strip_packing::SPProblem;
use jaguars::geometry::geo_traits::TransformableFrom;
use jaguars::io::json_instance::JsonInstance;
use jaguars::io::parser::Parser;
use jaguars::util::config::{CDEConfig, HazProxConfig, QuadTreeConfig, SPSurrogateConfig};
use jaguars::util::polygon_simplification::PolySimplConfig;
use lbf::config::Config;
use lbf::lbf_optimizer::LBFOptimizer;
use lbf::samplers::uniform_rect_sampler::UniformAARectSampler;
use crate::util::{create_base_config, N_ITEMS_REMOVED, N_SAMPLES, SWIM_PATH};

criterion_main!(benches);
criterion_group!(benches, quadtree_query_update_1000_1, quadtree_query_bench, quadtree_update_bench);

mod util;

const QT_DEPTHS: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

/// Benchmark the update operation of the quadtree for different depths
/// From a solution, created by the LBF optimizer, 5 items are removed and then inserted back again
fn quadtree_update_bench(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = create_base_config();
    //disable fail fast surrogates
    config.cde_config.item_surrogate_config.n_ff_poles = 0;
    config.cde_config.item_surrogate_config.n_ff_piers = 0;
    //disable haz prox grid
    config.cde_config.haz_prox = HazProxConfig::Enabled { n_cells: 1};

    let mut group = c.benchmark_group("quadtree_update");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree = QuadTreeConfig::FixedDepth(depth);
        let instance = util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let (mut problem, _) = util::create_blf_problem(instance.clone(), config, 0);

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
                problem.flush_changes();

                // Insert 5 items back into the layout
                for pi_uid in selected_pi_uids.into_iter().flatten() {
                    //println!("Inserting item with id: {}\n", pi_uid.item_id);
                    problem.place_item(&PlacingOption {
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
/// We validate 1000 sampled transformations for each of the 5 removed items
fn quadtree_query_bench(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = create_base_config();
    //disable fail fast surrogates
    config.cde_config.item_surrogate_config.n_ff_poles = 0;
    config.cde_config.item_surrogate_config.n_ff_piers = 0;
    //disable haz prox grid
    config.cde_config.haz_prox = HazProxConfig::Enabled { n_cells: 1};

    let mut group = c.benchmark_group("quadtree_query");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree = QuadTreeConfig::FixedDepth(depth);
        let instance = util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let (mut problem, selected_pi_uids) = util::create_blf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let layout = problem.get_layout(LayoutIndex::Existing(0));
        let sampler = UniformAARectSampler::new(layout.bin().bbox(), instance.item(0));
        let mut rng = SmallRng::seed_from_u64(0);

        let samples = (0..N_SAMPLES).map(
            |_| sampler.sample(&mut rng).compose()
        ).collect_vec();

        let mut n_invalid: i64 = 0;
        let mut n_valid : i64 = 0;

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                for item_id in selected_pi_uids.iter().map(|pi_uid| pi_uid.item_id) {
                    let item = instance.item(item_id);
                    let layout = problem.get_layout(LayoutIndex::Existing(0));
                    let mut buffer_shape = item.shape().clone();
                    for transf in samples.iter() {
                        buffer_shape.transform_from(item.shape(), transf);
                        let collides = layout.cde().shape_collides(&buffer_shape, &[]);
                        if collides {
                            n_invalid += 1;
                        } else {
                            n_valid += 1;
                        }
                    }
                }
            })
        });
        println!("valid: {:.3}%", n_valid as f64 / (n_valid + n_invalid) as f64 * 100.0);
    }
    group.finish();
}

fn quadtree_query_update_1000_1(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = create_base_config();
    //disable fail fast surrogates
    config.cde_config.item_surrogate_config.n_ff_poles = 0;
    config.cde_config.item_surrogate_config.n_ff_piers = 0;
    //disable haz prox grid
    config.cde_config.haz_prox = HazProxConfig::Enabled { n_cells: 1};


    let mut group = c.benchmark_group("quadtree_query_update_1000_1");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree = QuadTreeConfig::FixedDepth(depth);
        let instance = util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let (mut problem, selected_pi_uids) = util::create_blf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let mut rng = SmallRng::seed_from_u64(0);

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let mut selected_pi_uids: [Option<PlacedItemUID>; N_ITEMS_REMOVED] = <[_; N_ITEMS_REMOVED]>::default();
                // Remove 5 items from the layout
                problem.get_layout(LayoutIndex::Existing(0)).placed_items().iter()
                    .map(|p_i| Some(p_i.uid().clone()))
                    .choose_multiple_fill(&mut rng, &mut selected_pi_uids);

                for pi_uid in selected_pi_uids.iter().flatten() {
                    //println!("Removing item with id: {}\n", pi_uid.item_id);
                    problem.remove_item(LayoutIndex::Existing(0), pi_uid);
                }

                problem.flush_changes();

                for item_id in selected_pi_uids.iter().flatten().map(|pi_uid| pi_uid.item_id) {
                    let item = instance.item(item_id);
                    let layout = problem.get_layout(LayoutIndex::Existing(0));
                    let mut buffer_shape = item.shape().clone();
                    let sampler = UniformAARectSampler::new(layout.bin().bbox(), item);
                    for _ in 0..1000 {
                        let transf = sampler.sample(&mut rng);
                        buffer_shape.transform_from(item.shape(), &transf.compose());
                        let collides = layout.cde().shape_collides(&buffer_shape, &[]);
                        criterion::black_box(collides); // prevent the compiler from optimizing the loop away
                    }
                }

                // Insert 5 items back into the layout
                for pi_uid in selected_pi_uids.into_iter().flatten() {
                    //println!("Inserting item with id: {}\n", pi_uid.item_id);
                    problem.place_item(&PlacingOption {
                        layout_index: LayoutIndex::Existing(0),
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