use std::fs::File;
use std::io::BufReader;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use jagua_rs::entities::instances::instance_generic::InstanceGeneric;

use jagua_rs::entities::problems::problem_generic::{LayoutIndex, ProblemGeneric};
use jagua_rs::geometry::convex_hull;
use jagua_rs::geometry::fail_fast::{piers, poi};
use jagua_rs::geometry::fail_fast::sp_surrogate::SPSurrogate;
use jagua_rs::geometry::geo_traits::TransformableFrom;
use jagua_rs::geometry::primitives::circle::Circle;
use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;
use jagua_rs::io::json_instance::JsonInstance;
use lbf::samplers::hpg_sampler::HPGSampler;

use crate::util::{create_base_config, N_ITEMS_REMOVED, SWIM_PATH};

criterion_main!(benches);
criterion_group!(benches, fast_fail_query_bench);

mod util;

const FF_POLES: &[usize] = &[0,1,2,3,4];
const FF_PIERS: &[usize] = &[0,1,2,3,4];

const ITEMS_ID_TO_TEST: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

const N_TOTAL_SAMPLES: usize = 100_000;
const N_SAMPLES_PER_ITER: usize = 1000;

/// Benchmark the query operation of the quadtree for different depths
/// We validate 1000 sampled transformations for each of the 5 removed items
fn fast_fail_query_bench(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();

    let mut group = c.benchmark_group("fast_fail_query_bench");

    let config_combos = FF_POLES.iter().map(|n_ff_poles|
        FF_PIERS.iter().map(|n_ff_piers| (*n_ff_poles, *n_ff_piers)).collect_vec())
        .flatten().collect_vec();

    let mut config = create_base_config();
    config.cde_config.quadtree_depth = 5;
    config.cde_config.hpg_n_cells = 2000;

    let instance = util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
    let (problem, _) = util::create_blf_problem(instance.clone(), config, N_ITEMS_REMOVED);

    println!("avg number of edges per item: {}", ITEMS_ID_TO_TEST.iter().map(|&item_id| instance.item(item_id).shape.number_of_points()).sum::<usize>() as f64 / ITEMS_ID_TO_TEST.len() as f64);

    let mut rng = SmallRng::seed_from_u64(0);
    let layout = problem.get_layout(LayoutIndex::Real(0));
    let samples = ITEMS_ID_TO_TEST.iter()
        .map(|&item_id| {
            let sampler = HPGSampler::new(instance.item(item_id), layout).unwrap();
            (0..N_TOTAL_SAMPLES).map(|_| sampler.sample(&mut rng)).collect_vec()
        }).collect_vec();

    for ff_surr_config in config_combos {
        let (n_ff_poles, n_ff_piers) = ff_surr_config;

        let custom_surrogates = ITEMS_ID_TO_TEST.iter()
            .map(|&item_id| create_custom_surrogate(&instance.item(item_id).shape, n_ff_poles, n_ff_piers))
            .collect_vec();

        let mut samples_cyclers = samples.iter()
            .map(|s| s.chunks(N_SAMPLES_PER_ITER).cycle())
            .collect_vec();

        let mut n_invalid: i64 = 0;
        let mut n_valid: i64 = 0;

        let mut i_cycler = ITEMS_ID_TO_TEST.iter().enumerate().cycle();

        let mut buffer_shapes = ITEMS_ID_TO_TEST.iter()
            .map(|&item_id| instance.item(item_id))
            .map(|item| item.shape.clone_and_strip_surrogate())
            .collect_vec();

        group.bench_function(BenchmarkId::from_parameter(format!("{n_ff_poles}_poles_{n_ff_piers}_piers")), |b| {
            b.iter(|| {
                let (i, &item_id) = i_cycler.next().unwrap();
                let item = instance.item(item_id);
                let surrogate = &custom_surrogates[i];
                let buffer_shape = &mut buffer_shapes[i];
                for transf in samples_cyclers[i].next().unwrap() {
                    let collides = match layout.cde().surrogate_collides(surrogate, transf, &[]) {
                        true => true,
                        false => {
                            buffer_shape.transform_from(&item.shape, transf);
                            layout.cde().shape_collides(&buffer_shape, &[])
                        }
                    };
                    match collides {
                        true => n_invalid += 1,
                        false => n_valid += 1
                    }
                }
            })
        });
        println!("{:.3}% valid", n_valid as f64 / (n_invalid + n_valid) as f64 * 100.0);
    }
    group.finish();
}

pub fn create_custom_surrogate(simple_poly: &SimplePolygon, n_poles: usize, n_piers: usize) -> SPSurrogate {
    let convex_hull_indices = convex_hull::convex_hull_indices(simple_poly);
    let mut poles = vec![simple_poly.poi.clone()];
    poles.extend(poi::generate_additional_surrogate_poles(simple_poly, n_poles.saturating_sub(1), 0.9));
    let poles_bounding_circle = Circle::bounding_circle(&poles);

    let n_ff_poles = usize::min(n_poles, poles.len());
    let relevant_poles_for_piers = &poles[0..n_ff_poles];
    let piers = piers::generate(simple_poly, n_piers, relevant_poles_for_piers);

    let surrogate = SPSurrogate {
        convex_hull_indices,
        poles,
        piers,
        poles_bounding_circle,
        n_ff_poles,
    };

    dbg!(surrogate.ff_poles().len(), surrogate.ff_piers().len());
    surrogate
}