use crate::util::{N_ITEMS_REMOVED, create_base_config};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use jagua_rs::collision_detection::hazards::collector::BasicHazardCollector;
use jagua_rs::collision_detection::hazards::filter::NoFilter;
use jagua_rs::geometry::geo_traits::{Transformable, TransformableFrom};
use jagua_rs::probs::spp::entities::SPPlacement;
use lbf::samplers::uniform_rect_sampler::UniformRectSampler;
use rand::SeedableRng;
use rand::prelude::{IteratorRandom, SmallRng};
use std::hint::black_box;

criterion_main!(benches);
criterion_group!(
    benches,
    cde_collect_bench,
    cde_update_bench,
    cde_detect_bench,
    reflection_bench,
);

mod util;

const QT_DEPTHS: [u8; 3] = [3, 4, 5];
const N_SAMPLES_PER_ITER: usize = 1000;

/// Compare buffer transforms and complete queries on the same swim shapes and poses.
/// Reflection is forced to measure both geometry paths independently of item permissions.
fn reflection_bench(c: &mut Criterion) {
    let config = create_base_config();
    let instance = util::create_instance(config.cde_config, config.poly_simpl_tolerance);
    let (problem, _) = util::create_lbf_problem(instance.clone(), config, 0);
    let mut rng = SmallRng::seed_from_u64(42);
    let samples: Vec<_> = instance
        .items
        .iter()
        .map(|(item, _)| item.as_ref())
        .map(|item| {
            let sampler = UniformRectSampler::new(problem.layout.cde().bbox(), item);
            let pose = sampler.sample(&mut rng);
            (item, [pose.compose(), pose.with_reflection(true).compose()])
        })
        .collect();
    let mut buffers: Vec<_> = samples
        .iter()
        .map(|(item, _)| item.shape_cd().as_ref().clone())
        .collect();
    let mut collector =
        BasicHazardCollector::with_capacity(problem.layout.cde().hazards_map().len());
    let mut group = c.benchmark_group("reflection");
    group.throughput(criterion::Throughput::Elements(N_SAMPLES_PER_ITER as u64));
    for mode in ["unreflected", "reflected", "alternating"] {
        for operation in ["transform_from", "transform_clone", "transform_and_collect"] {
            group.bench_function(BenchmarkId::new(operation, mode), |b| {
                let mut iteration = 0;
                b.iter(|| {
                    for i in 0..N_SAMPLES_PER_ITER {
                        let index = i % samples.len();
                        let (item, transforms) = &samples[index];
                        let reflected = match mode {
                            "unreflected" => false,
                            "reflected" => true,
                            "alternating" => (i + iteration).is_multiple_of(2),
                            _ => unreachable!(),
                        };
                        let transform = &transforms[usize::from(reflected)];
                        if operation == "transform_clone" {
                            black_box(item.shape_cd().transform_clone(transform));
                            continue;
                        }
                        let buffer = &mut buffers[index];
                        buffer.transform_from(item.shape_cd(), transform);
                        if operation == "transform_and_collect" {
                            problem
                                .layout
                                .cde()
                                .collect_surrogate_collisions(buffer, &mut collector);
                            problem
                                .layout
                                .cde()
                                .collect_poly_collisions(buffer, &mut collector);
                            black_box(collector.len());
                            collector.clear();
                        }
                        black_box(&buffer);
                    }
                    iteration += 1;
                });
            });
        }
    }
    group.finish();
}

/// Benchmark how many complete collision collection queries can be performed every second with different quadtree depths. (no early exit)
/// The layout is dense.
fn cde_collect_bench(c: &mut Criterion) {
    let mut config = create_base_config();

    let mut group = c.benchmark_group("cde_collect_1k");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance = util::create_instance(config.cde_config, config.poly_simpl_tolerance);
        let (problem, _) = util::create_lbf_problem(instance.clone(), config, 0);

        let mut rng = SmallRng::seed_from_u64(0);

        let mut n_detected = 0;

        // Configure throughput measurement - this tells Criterion each iteration performs N_SAMPLES_PER_ITER operations
        group.throughput(criterion::Throughput::Elements(N_SAMPLES_PER_ITER as u64));

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let search_for = problem
                    .layout
                    .placed_items()
                    .iter()
                    .choose(&mut rng)
                    .expect("No items in layout");
                let item = &search_for.1.item();
                let cde = &problem.layout.cde();
                let mut buffer_shape = item.shape_cd().as_ref().clone();
                let mut collector = BasicHazardCollector::with_capacity(cde.hazards_map().len());
                let sampler = UniformRectSampler::new(cde.bbox(), item);
                for _ in 0..N_SAMPLES_PER_ITER {
                    let d_transf = sampler.sample(&mut rng);
                    let transf = d_transf.compose();
                    //detect collisions with the surrogate
                    buffer_shape.transform_from(item.shape_cd(), &transf);
                    cde.collect_surrogate_collisions(&buffer_shape, &mut collector);
                    //detect collisions with the actual shape
                    cde.collect_poly_collisions(&buffer_shape, &mut collector);
                    n_detected += collector.len();
                    collector.clear();
                }
            })
        });
    }
    group.finish();
}

/// Benchmark how many complete collision detection queries can be performed every second with different quadtree depths.
/// The layout has a couple of items removed.
fn cde_detect_bench(c: &mut Criterion) {
    let mut config = util::create_base_config();

    let mut group = c.benchmark_group("cde_detect_1k");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance = util::create_instance(config.cde_config, config.poly_simpl_tolerance);
        let (problem, _) = util::create_lbf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let mut rng = SmallRng::seed_from_u64(0);

        let mut n_detected = 0;

        // Configure throughput measurement - this tells Criterion each iteration performs N_SAMPLES_PER_ITER operations
        group.throughput(criterion::Throughput::Elements(N_SAMPLES_PER_ITER as u64));

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                let item_to_move = problem
                    .layout
                    .placed_items()
                    .iter()
                    .choose(&mut rng)
                    .expect("No items in layout");
                let item = &item_to_move.1.item();
                let cde = &problem.layout.cde();
                let mut buffer_shape = item.shape_cd().as_ref().clone();
                let sampler = UniformRectSampler::new(cde.bbox(), item);
                for _ in 0..N_SAMPLES_PER_ITER {
                    let d_transf = sampler.sample(&mut rng);
                    let transf = d_transf.compose();
                    //detect collisions with the surrogate
                    if !cde.detect_surrogate_collision(
                        item.shape_cd().surrogate(),
                        &transf,
                        &NoFilter,
                    ) {
                        buffer_shape.transform_from(item.shape_cd(), &transf);
                        if !cde.detect_poly_collision(&buffer_shape, &NoFilter) {
                            n_detected += 1;
                        }
                    }
                }
            })
        });
    }
    group.finish();
}

/// Benchmarks updating the state of the CDEngine by removing an item and placing it again.
fn cde_update_bench(c: &mut Criterion) {
    let mut config = create_base_config();

    let mut group = c.benchmark_group("cde_update_1k");
    for depth in QT_DEPTHS {
        config.cde_config.quadtree_depth = depth;
        let instance = util::create_instance(config.cde_config, config.poly_simpl_tolerance);
        let (mut problem, _) = util::create_lbf_problem(instance.clone(), config, 0);

        let mut rng = SmallRng::seed_from_u64(0);

        group.throughput(criterion::Throughput::Elements(N_SAMPLES_PER_ITER as u64));

        group.bench_function(BenchmarkId::from_parameter(depth), |b| {
            b.iter(|| {
                for _ in 0..N_SAMPLES_PER_ITER {
                    // Remove an item from the layout
                    let (pkey, pi) = problem
                        .layout
                        .placed_items()
                        .iter()
                        .choose(&mut rng)
                        .expect("No items in layout");

                    let p_opt = SPPlacement {
                        item_idx: pi.item().idx(),
                        d_transf: pi.d_transf(),
                    };

                    //println!("Removing item with idx: {}\n", pi_uid.item_idx);
                    problem.remove_item(pkey);

                    problem.place_item(p_opt);
                }
            })
        });
    }
    group.finish();
}
