use std::fs::File;
use std::io::BufReader;
use std::ops::RangeInclusive;
use std::path::Path;
use std::sync::Arc;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use log::debug;
use rand::prelude::{IteratorRandom, SmallRng};
use rand::SeedableRng;
use rand_distr::num_traits::real::Real;
use tribool::Tribool;
use jaguars::entities::instance::{Instance, Containers};
use jaguars::entities::placed_item::PlacedItemUID;
use jaguars::entities::placing_option::PlacingOption;
use jaguars::entities::problems::problem::{LayoutIndex, ProblemVariant, Problem};
use jaguars::entities::problems::strip_packing::SPProblem;
use jaguars::geometry::fail_fast::sp_surrogate::SPSurrogate;
use jaguars::geometry::geo_traits::TransformableFrom;
use jaguars::geometry::transformation::Transformation;
use jaguars::io::json_instance::JsonInstance;
use jaguars::io::parser::Parser;
use jaguars::util::config::{CDEConfig, HazProxConfig, QuadTreeConfig, SPSurrogateConfig};
use jaguars::util::polygon_simplification::PolySimplConfig;
use lbf::config::Config;
use lbf::io;
use lbf::io::layout_to_svg::layout_to_svg;
use lbf::io::svg_util::SvgDrawOptions;
use lbf::lbf_optimizer::LBFOptimizer;
use lbf::samplers::hpg_sampler::HPGSampler;
use lbf::samplers::uniform_rect_sampler::UniformAARectSampler;
use crate::util::{create_base_config, N_ITEMS_REMOVED, N_SAMPLES, SWIM_PATH};

criterion_main!(benches);
criterion_group!(benches, fast_fail_query_bench);

mod util;

const FF_POLE_RANGE: RangeInclusive<usize> = 0..=4;
const FF_PIER_RANGE: RangeInclusive<usize> = 0..=0;

/// Benchmark the query operation of the quadtree for different depths
/// We validate 1000 sampled transformations for each of the 5 removed items
fn fast_fail_query_bench(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();

    let mut group = c.benchmark_group("fast_fail_query_bench");

    let config_combos= FF_POLE_RANGE.map(|n_ff_poles| FF_PIER_RANGE.map(|n_ff_piers| (n_ff_poles, n_ff_piers)).collect_vec()).flatten().collect_vec();

    for ff_surr_config in config_combos {
        let mut config = create_base_config();
        config.cde_config.item_surrogate_config.n_ff_poles = ff_surr_config.0;
        config.cde_config.item_surrogate_config.n_ff_piers = ff_surr_config.1;

        let instance = util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let (mut problem, selected_pi_uids) = util::create_blf_problem(instance.clone(), config, N_ITEMS_REMOVED);

        let mut rng = SmallRng::seed_from_u64(0);

        /*{
            let draw_options = SvgDrawOptions{
                quadtree: true,
                surrogate: true,
                ..SvgDrawOptions::default()
            };
            let svg = io::layout_to_svg::layout_to_svg(problem.get_layout(&layout_index), &instance, draw_options);
            io::write_svg(&svg, Path::new("removed_items.svg"));
        }*/

        let (n_ff_poles, n_ff_piers) = ff_surr_config;

        let custom_surrogates = selected_pi_uids.iter().map(|pi_uid| {
            let item = instance.item(pi_uid.item_id);
            let mut surrogate = item.shape().surrogate().clone();
            surrogate.ff_pole_range = 0..n_ff_poles; //also check PoI, which is not standard
            surrogate
        }).collect_vec();

        let layout = problem.get_layout(LayoutIndex::Existing(0));
        let samples = (0..N_ITEMS_REMOVED)
            .map(|i| selected_pi_uids[i].item_id)
            .map(|item_id| {
                let item = instance.item(item_id);
                let sampler = HPGSampler::new(item, layout).unwrap();
                (0..N_SAMPLES).map(
                    |_| sampler.sample(&mut rng)
                ).collect_vec()
            }).collect_vec();

        let mut n_invalid: i64 = 0;
        let mut n_valid : i64 = 0;

        group.bench_function(BenchmarkId::from_parameter(format!("{n_ff_poles}_poles_{n_ff_piers}_piers")), |b| {
            b.iter(|| {
                for i in 0..N_ITEMS_REMOVED {
                    let pi_uid = &selected_pi_uids[i];
                    let item = instance.item(pi_uid.item_id);
                    let surrogate = &custom_surrogates[i];
                    let mut buffer_shape = item.shape().clone();
                    for transf in samples[i].iter() {
                        let collides = match layout.cde().surrogate_collides(surrogate, transf, &[]) {
                            true => true,
                            false => {
                                buffer_shape.transform_from(item.shape(), transf);
                                layout.cde().shape_collides(&buffer_shape, &[])
                            }
                        };
                        match collides {
                            true => n_invalid += 1,
                            false => n_valid += 1
                        }
                    }
                }
            })
        });
        println!("{:.3}% valid", n_valid as f64 / (n_invalid + n_valid) as f64 * 100.0);
    }
    group.finish();
}