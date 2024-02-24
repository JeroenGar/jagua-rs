use std::fs::File;
use std::io::BufReader;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use rand::prelude::SmallRng;
use rand::seq::IteratorRandom;
use rand::SeedableRng;

use jagua_rs::entities::instances::instance_generic::InstanceGeneric;
use jagua_rs::entities::placing_option::PlacingOption;
use jagua_rs::entities::problems::problem_generic::{LayoutIndex, ProblemGeneric};
use jagua_rs::geometry::geo_traits::TransformableFrom;
use jagua_rs::io::json_instance::JsonInstance;
use lbf::samplers::uniform_rect_sampler::UniformAARectSampler;

use crate::util::{create_base_config, N_ITEMS_REMOVED, SWIM_PATH};

criterion_main!(benches);
criterion_group!(
    benches,
    quadtree_query_update_1000_1,
    quadtree_query_bench,
    quadtree_update_bench
);

mod util;

const QT_DEPTHS: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

const N_TOTAL_SAMPLES: usize = 100_000;
const N_SAMPLES_PER_ITER: usize = 1000;

/// Benchmark the update operation of the quadtree for different depths
/// From a solution, created by the LBF optimizer, 5 items are removed and then inserted back again
fn quadtree_update_bench(c: &mut Criterion) {
    let json_instance: JsonInstance =
        serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = create_base_config();
    //disable fail fast surrogates
    config.cde_config.item_surrogate_config.n_ff_poles = 0;
    config.cde_config.item_surrogate_config.n_ff_piers = 0;
    //disable haz prox grid
    config.cde_config.hpg_n_cells = 1;

    let mut group = c.benchmark_group("quadtree_update");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance =
            util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let (mut problem, _) = util::create_blf_problem(instance.clone(), config, 0);

        let layout_index = LayoutIndex::Real(0);
        let mut rng = SmallRng::seed_from_u64(0);

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                // Remove an item from the layout
                let pi_uid = problem
                    .get_layout(&layout_index)
                    .placed_items()
                    .iter()
                    .map(|p_i| p_i.uid.clone())
                    .choose(&mut rng)
                    .expect("No items in layout");

                //println!("Removing item with id: {}\n", pi_uid.item_id);
                problem.remove_item(layout_index, &pi_uid, true);

                problem.flush_changes();

                problem.place_item(&PlacingOption {
                    layout_index,
                    item_id: pi_uid.item_id,
                    transform: pi_uid.d_transf.compose(),
                    d_transform: pi_uid.d_transf,
                });
            })
        });
    }
    group.finish();
}

/// Benchmark the query operation of the quadtree for different depths
/// We validate 1000 sampled transformations for each of the 5 removed items
fn quadtree_query_bench(c: &mut Criterion) {
    let json_instance: JsonInstance =
        serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = create_base_config();
    //disable fail fast surrogates
    config.cde_config.item_surrogate_config.n_ff_poles = 0;
    config.cde_config.item_surrogate_config.n_ff_piers = 0;
    //disable haz prox grid
    config.cde_config.hpg_n_cells = 1;

    let mut group = c.benchmark_group("quadtree_query");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance =
            util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let (problem, selected_pi_uids) =
            util::create_blf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let layout = problem.get_layout(LayoutIndex::Real(0));
        let sampler = UniformAARectSampler::new(layout.bin().bbox(), instance.item(0));
        let mut rng = SmallRng::seed_from_u64(0);

        let samples = (0..N_TOTAL_SAMPLES)
            .map(|_| sampler.sample(&mut rng).compose())
            .collect_vec();

        let mut sample_cycler = samples.chunks(N_SAMPLES_PER_ITER).cycle();

        let mut n_invalid: i64 = 0;
        let mut n_valid: i64 = 0;

        let mut item_id_cycler = selected_pi_uids.iter().map(|pi_uid| pi_uid.item_id).cycle();

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let item_id = item_id_cycler.next().unwrap();
                let item = instance.item(item_id);
                let layout = problem.get_layout(LayoutIndex::Real(0));
                let mut buffer_shape = item.shape.as_ref().clone();
                for transf in sample_cycler.next().unwrap() {
                    buffer_shape.transform_from(&item.shape, transf);
                    let collides = layout.cde().shape_collides(&buffer_shape, &[]);
                    if collides {
                        n_invalid += 1;
                    } else {
                        n_valid += 1;
                    }
                }
            })
        });
        println!(
            "valid: {:.3}%",
            n_valid as f64 / (n_valid + n_invalid) as f64 * 100.0
        );
    }
    group.finish();
}

fn quadtree_query_update_1000_1(c: &mut Criterion) {
    let json_instance: JsonInstance =
        serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = create_base_config();
    //disable fail fast surrogates
    config.cde_config.item_surrogate_config.n_ff_poles = 0;
    config.cde_config.item_surrogate_config.n_ff_piers = 0;
    //disable haz prox grid
    config.cde_config.hpg_n_cells = 1;

    let mut group = c.benchmark_group("quadtree_query_update_1000_1");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance =
            util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let (mut problem, _) = util::create_blf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let layout = problem.get_layout(LayoutIndex::Real(0));
        let sampler = UniformAARectSampler::new(layout.bin().bbox(), instance.item(0));
        let mut rng = SmallRng::seed_from_u64(0);

        let samples = (0..N_TOTAL_SAMPLES)
            .map(|_| sampler.sample(&mut rng).compose())
            .collect_vec();

        let mut sample_cycler = samples.chunks(N_SAMPLES_PER_ITER).cycle();

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let pi_uid = problem
                    .get_layout(LayoutIndex::Real(0))
                    .placed_items()
                    .iter()
                    .map(|p_i| p_i.uid.clone())
                    .choose(&mut rng)
                    .expect("No items in layout");

                problem.remove_item(LayoutIndex::Real(0), &pi_uid, true);
                problem.flush_changes();

                let item_id = pi_uid.item_id;
                let item = instance.item(item_id);
                let layout = problem.get_layout(LayoutIndex::Real(0));
                let mut buffer_shape = item.shape.as_ref().clone();
                for transf in sample_cycler.next().unwrap() {
                    buffer_shape.transform_from(&item.shape, &transf);
                    let collides = layout.cde().shape_collides(&buffer_shape, &[]);
                    criterion::black_box(collides); //prevent the compiler from optimizing the loop away
                }

                problem.place_item(&PlacingOption {
                    layout_index: LayoutIndex::Real(0),
                    item_id: pi_uid.item_id,
                    transform: pi_uid.d_transf.compose(),
                    d_transform: pi_uid.d_transf,
                })
            })
        });
    }
    group.finish();
}
