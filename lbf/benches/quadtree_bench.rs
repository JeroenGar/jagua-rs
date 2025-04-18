use std::fs::File;
use std::io::BufReader;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use rand::SeedableRng;
use rand::prelude::SmallRng;
use rand::seq::IteratorRandom;

use jagua_rs::collision_detection::hazards::detector::{BasicHazardDetector, HazardDetector};
use jagua_rs::collision_detection::hazards::filter::NoHazardFilter;
use jagua_rs::entities::general::Instance;
use jagua_rs::entities::strip_packing::SPPlacement;
use jagua_rs::geometry::geo_traits::{Shape, TransformableFrom};
use jagua_rs::io::json_instance::JsonInstance;
use lbf::samplers::uniform_rect_sampler::UniformRectSampler;

use crate::util::{N_ITEMS_REMOVED, SWIM_PATH, create_base_config};

criterion_main!(benches);
criterion_group!(
    benches,
    quadtree_query_update_1000_1,
    quadtree_query_bench,
    quadtree_update_bench,
    quadtree_collect_query_bench
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

    let mut group = c.benchmark_group("quadtree_update");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance = util::create_instance(
            &json_instance,
            config.cde_config,
            config.poly_simpl_tolerance,
        );
        let (mut problem, _) = util::create_lbf_problem(instance.clone(), config, 0);

        let mut rng = SmallRng::seed_from_u64(0);

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                // Remove an item from the layout
                let (pkey, pi) = problem
                    .layout
                    .placed_items()
                    .iter()
                    .choose(&mut rng)
                    .expect("No items in layout");

                let p_opt = SPPlacement {
                    item_id: pi.item_id,
                    d_transf: pi.d_transf,
                };

                //println!("Removing item with id: {}\n", pi_uid.item_id);
                problem.remove_item(pkey, true);

                problem.place_item(p_opt);
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

    let mut group = c.benchmark_group("quadtree_query");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance = util::create_instance(
            &json_instance,
            config.cde_config,
            config.poly_simpl_tolerance,
        );
        let (problem, selected_pi_uids) =
            util::create_lbf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let layout = &problem.layout;
        let sampler = UniformRectSampler::new(layout.bin.outer_cd.bbox(), instance.item(0));
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
                let layout = &problem.layout;
                let mut buffer_shape = item.shape_cd.as_ref().clone();
                for transf in sample_cycler.next().unwrap() {
                    buffer_shape.transform_from(&item.shape_cd, transf);
                    let collides = layout.cde().poly_collides(&buffer_shape, &NoHazardFilter);
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
            n_valid as f32 / (n_valid + n_invalid) as f32 * 100.0
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

    let mut group = c.benchmark_group("quadtree_query_update_1000_1");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance = util::create_instance(
            &json_instance,
            config.cde_config,
            config.poly_simpl_tolerance,
        );
        let (mut problem, _) = util::create_lbf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let layout = &problem.layout;
        let sampler = UniformRectSampler::new(layout.bin.outer_cd.bbox(), instance.item(0));
        let mut rng = SmallRng::seed_from_u64(0);

        let samples = (0..N_TOTAL_SAMPLES)
            .map(|_| sampler.sample(&mut rng).compose())
            .collect_vec();

        let mut sample_cycler = samples.chunks(N_SAMPLES_PER_ITER).cycle();

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let (pkey, pi) = problem
                    .layout
                    .placed_items()
                    .iter()
                    .choose(&mut rng)
                    .expect("No items in layout");

                let p_opt = SPPlacement {
                    item_id: pi.item_id,
                    d_transf: pi.d_transf,
                };

                problem.remove_item(pkey, true);

                let item_id = p_opt.item_id;
                let item = instance.item(item_id);
                let layout = &problem.layout;
                let mut buffer_shape = item.shape_cd.as_ref().clone();
                for transf in sample_cycler.next().unwrap() {
                    buffer_shape.transform_from(&item.shape_cd, &transf);
                    let collides = layout.cde().poly_collides(&buffer_shape, &NoHazardFilter);
                    criterion::black_box(collides); //prevent the compiler from optimizing the loop away
                }

                problem.place_item(p_opt)
            })
        });
    }
    group.finish();
}

/// Benchmark the query operation of the quadtree for different depths
/// Instead of merely detecting whether any collisions occur for collisions, we collect all entities that collide
/// We validate 1000 sampled transformations for each of the 5 removed items
fn quadtree_collect_query_bench(c: &mut Criterion) {
    let json_instance: JsonInstance =
        serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut config = create_base_config();
    //disable fail fast surrogates
    config.cde_config.item_surrogate_config.n_ff_poles = 0;
    config.cde_config.item_surrogate_config.n_ff_piers = 0;

    let mut group = c.benchmark_group("quadtree_collect_query");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance = util::create_instance(
            &json_instance,
            config.cde_config,
            config.poly_simpl_tolerance,
        );
        let (problem, selected_pi_uids) =
            util::create_lbf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let layout = &problem.layout;
        let sampler = UniformRectSampler::new(layout.bin.outer_cd.bbox(), instance.item(0));
        let mut rng = SmallRng::seed_from_u64(0);

        let samples = (0..N_TOTAL_SAMPLES)
            .map(|_| sampler.sample(&mut rng).compose())
            .collect_vec();

        let mut sample_cycler = samples.chunks(N_SAMPLES_PER_ITER).cycle();

        let mut n_invalid: i64 = 0;
        let mut n_valid: i64 = 0;
        let mut n_detected: i64 = 0;

        let mut item_id_cycler = selected_pi_uids.iter().map(|pi_uid| pi_uid.item_id).cycle();

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let item_id = item_id_cycler.next().unwrap();
                let item = instance.item(item_id);
                let layout = &problem.layout;
                let mut buffer_shape = item.shape_cd.as_ref().clone();
                let mut detected = BasicHazardDetector::new();
                for transf in sample_cycler.next().unwrap() {
                    buffer_shape.transform_from(&item.shape_cd, transf);
                    layout
                        .cde()
                        .collect_poly_collisions(&buffer_shape, &mut detected);
                    if detected.len() > 0 {
                        n_invalid += 1;
                    } else {
                        n_valid += 1;
                    }
                    n_detected += detected.len() as i64;
                    detected.clear();
                }
            })
        });
        println!(
            "valid: {:.3}%, avg # detected: {:.3}",
            n_valid as f32 / (n_valid + n_invalid) as f32 * 100.0,
            n_detected as f32 / (n_valid + n_invalid) as f32
        );
    }
    group.finish();
}
