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
use tribool::Tribool;
use jaguars::entities::instance::{Instance, PackingType};
use jaguars::entities::placed_item::PlacedItemUID;
use jaguars::entities::placing_option::PlacingOption;
use jaguars::entities::problems::problem::{LayoutIndex, Problem, ProblemEnum};
use jaguars::entities::problems::sp_problem::SPProblem;
use jaguars::geometry::fail_fast::sp_surrogate::SPSurrogate;
use jaguars::geometry::geo_traits::TransformableFrom;
use jaguars::io::json_instance::JsonInstance;
use jaguars::io::parser::Parser;
use jaguars::util::config::{CDEConfig, HazProxConfig, QuadTreeConfig, SPSurrogateConfig};
use jaguars::util::polygon_simplification::PolySimplConfig;
use lbf::config::Config;
use lbf::io;
use lbf::io::layout_to_svg::layout_to_svg;
use lbf::io::svg_util::SvgDrawOptions;
use lbf::lbf_optimizer::LBFOptimizer;
use lbf::samplers::uniform_rect_sampler::UniformAARectSampler;
use crate::util::SWIM_PATH;

criterion_main!(benches);
criterion_group!(benches, fast_fail_query_bench);

mod util;

const N_ITEMS_REMOVED: usize = 8;

const FF_POLE_RANGE: RangeInclusive<usize> = 0..=0;
const FF_PIER_RANGE: RangeInclusive<usize> = 0..=0;

/// Benchmark the query operation of the quadtree for different depths
/// We validate 1000 sampled transformations for each of the 5 removed items
fn fast_fail_query_bench(c: &mut Criterion) {
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();

    let mut group = c.benchmark_group("fast_fail_query_bench");

    let config_combos= FF_POLE_RANGE.map(|n_ff_poles| FF_PIER_RANGE.map(|n_ff_piers| (n_ff_poles, n_ff_piers)).collect_vec()).flatten().collect_vec();

    for ff_surr_config in config_combos {
        let config = create_config(&ff_surr_config);
        let instance = util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let mut problem = util::create_blf_problem(instance.clone(), config);

        let layout_index = LayoutIndex::Existing(0);
        let mut rng = SmallRng::seed_from_u64(0);
        let mut selected_pi_uids: [Option<PlacedItemUID>; N_ITEMS_REMOVED] = <[_; N_ITEMS_REMOVED]>::default();

        {
            // Remove 5 items from the layout
            problem.get_layout(&layout_index).placed_items().iter()
                .map(|p_i| Some(p_i.uid().clone()))
                .choose_multiple_fill(&mut rng, &mut selected_pi_uids);

            for pi_uid in selected_pi_uids.iter().flatten() {
                problem.remove_item(layout_index, pi_uid);
                println!("Removed item: {} with {} edges", pi_uid.item_id, instance.item(pi_uid.item_id).shape().number_of_points());
            }
            problem.flush_changes();
        }

        {
            let draw_options = SvgDrawOptions{
                quadtree: true,
                ..SvgDrawOptions::default()
            };
            let svg = io::layout_to_svg::layout_to_svg(problem.get_layout(&layout_index), &instance, draw_options);
            io::write_svg(&svg, Path::new("removed_items.svg"));
        }


        let (n_ff_poles, n_ff_piers) = ff_surr_config;

        let custom_surrogates = selected_pi_uids.clone().map(|pi_uid| {
            let item = instance.item(pi_uid.as_ref().unwrap().item_id);
            let mut surrogate = item.shape().surrogate().clone();
            surrogate.ff_pole_range = 0..n_ff_poles; //also check PoI, which is not standard
            surrogate
        });

        let mut n_invalid: i64 = 0;
        let mut n_valid : i64 = 0;

        group.bench_function(BenchmarkId::from_parameter(format!("{n_ff_poles}_poles_{n_ff_piers}_piers")), |b| {
            b.iter(|| {
                for i in 0..N_ITEMS_REMOVED {
                    let pi_uid = selected_pi_uids[i].as_ref().unwrap();
                    let layout = problem.get_layout(&layout_index);
                    let item = instance.item(pi_uid.item_id);
                    let surrogate = &custom_surrogates[i];
                    let mut buffer_shape = item.shape().clone();
                    let sampler = UniformAARectSampler::new(layout.bin().bbox(), item);
                    for _ in 0..1000 {
                        let d_transf = sampler.sample(&mut rng);
                        let transf = d_transf.compose();
                        let collides = match layout.cde().surrogate_collides(surrogate, &transf, &[]) {
                            true => true,
                            false => {
                                buffer_shape.transform_from(item.shape(), &transf);
                                layout.cde().poly_collides(&buffer_shape, &[])
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

fn create_config(ff_surrogate: &(usize, usize)) -> Config {
     Config {
        cde_config: CDEConfig {
            quadtree: QuadTreeConfig::FixedDepth(10),
            haz_prox: HazProxConfig::Enabled { n_cells: 1 },
            item_surrogate_config: SPSurrogateConfig {
                pole_coverage_goal: 0.9,
                max_poles: 10,
                n_ff_poles: ff_surrogate.0,
                n_ff_piers: ff_surrogate.1,
            },
        },
        poly_simpl_config: PolySimplConfig::Disabled,
        ..Config::default()
    }
}