use crate::util::create_base_config;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use jagua_rs::collision_detection::hazards::collector::BasicHazardCollector;
use jagua_rs::collision_detection::hazards::filter::NoFilter;
use jagua_rs::geometry::geo_traits::{Transformable, TransformableFrom};
use lbf::samplers::uniform_rect_sampler::UniformRectSampler;
use rand::SeedableRng;
use rand::prelude::SmallRng;
use std::hint::black_box;

criterion_main!(benches);
criterion_group!(benches, reflection_bench);

// Shared with the other benches; this one uses only part of it.
#[allow(dead_code)]
mod util;

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
            let sampler = UniformRectSampler::new(problem.layout().cde().bbox(), item);
            let pose = sampler.sample(&mut rng);
            (item, [pose.compose(), pose.with_reflection(true).compose()])
        })
        .collect();
    let mut buffers: Vec<_> = samples
        .iter()
        .map(|(item, _)| item.shape_cd().as_ref().clone())
        .collect();
    let mut collector =
        BasicHazardCollector::with_capacity(problem.layout().cde().hazards_map().len());
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
                            problem.layout().cde().collect_surrogate_collisions(
                                buffer,
                                &NoFilter,
                                &mut collector,
                            );
                            problem.layout().cde().collect_poly_collisions(
                                buffer,
                                &NoFilter,
                                &mut collector,
                            );
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
