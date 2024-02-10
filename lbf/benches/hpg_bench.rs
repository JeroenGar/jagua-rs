use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use rand::prelude::IteratorRandom;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use jaguars::entities::layout::Layout;

use jaguars::entities::placing_option::PlacingOption;
use jaguars::entities::problems::problem::{LayoutIndex, ProblemVariant};
use jaguars::entities::problems::strip_packing::SPProblem;
use jaguars::geometry::geo_traits::{Shape, TransformableFrom};
use jaguars::io::json_instance::JsonInstance;
use jaguars::util::config::HazProxConfig;
use lbf::io;
use lbf::io::svg_util::SvgDrawOptions;
use lbf::samplers::hpg_sampler::HPGSampler;

use crate::util::{create_base_config, N_ITEMS_REMOVED, N_VALID_SAMPLES, SWIM_PATH};

criterion_main!(benches);
criterion_group!(benches, hpg_bench);

mod util;

//pub const N_HPG_CELLS: [usize; 8] = [1, 50, 100, 500, 1000, 5000, 10000, 20000];

pub const N_HPG_CELLS: [usize; 6] = [100, 500, 1000, 5000, 10000, 20000];
pub const SELECTED_ITEM_ID: usize = 1; // relatively small and "round" item, guaranteed to find valid samples even without HPG

fn hpg_bench(c: &mut Criterion) {
    ///HPG density has side effects on the LBF optimize, so we create a single problem instance and create a solution from it.
    let json_instance: JsonInstance = serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let mut base_config = create_base_config();
    let base_instance = util::create_instance(&json_instance, base_config.cde_config, base_config.poly_simpl_config);
    let (mut base_problem, removed_items) = util::create_blf_problem(base_instance.clone(), base_config, N_ITEMS_REMOVED);
    let base_pi_uids = base_problem.get_layout(LayoutIndex::Existing(0)).placed_items().iter().map(|pi| pi.uid().clone()).collect_vec();

    let mut group = c.benchmark_group("hpg_bench");
    for n_hpg_cells in N_HPG_CELLS {
        let mut config = base_config;
        config.cde_config.haz_prox = HazProxConfig::Enabled { n_cells: n_hpg_cells };
        //create the instance and problem with the specific HPG config
        let instance = util::create_instance(&json_instance, config.cde_config, config.poly_simpl_config);
        let mut problem = SPProblem::new(instance.clone(), base_problem.strip_width(), config.cde_config);
        // Place the items in exactly the same way as the base problem
        for pi_uid in base_pi_uids.iter() {
            problem.place_item(&PlacingOption {
                layout_index: LayoutIndex::Existing(0),
                item_id: pi_uid.item_id,
                transf: pi_uid.d_transf.compose(),
                d_transf: pi_uid.d_transf.clone(),
            });
        }

        {
           let draw_options = SvgDrawOptions{
               quadtree: false,
               surrogate: false,
               haz_prox_grid: true,
               ..SvgDrawOptions::default()
           };
           let svg = io::layout_to_svg::layout_to_svg(problem.get_layout(LayoutIndex::Existing(0)), &instance, draw_options);
           io::write_svg(&svg, Path::new(&format!("removed_items_{n_hpg_cells}.svg")));
       }


        let mut rng = SmallRng::seed_from_u64(0);

        // Search N_VALID_SAMPLES for each item
        let item = instance.item(SELECTED_ITEM_ID);
        let layout = problem.get_layout(LayoutIndex::Existing(0));
        let surrogate = item.shape().surrogate();
        let mut buffer_shape = item.shape().clone();
        let sampler = HPGSampler::new(item, layout).unwrap();
        println!("[{}] sampler coverage: {:.3}% with {} samplers", n_hpg_cells, sampler.coverage_area / layout.bin().bbox().area() * 100.0, sampler.cell_samplers.len());

        group.bench_function(BenchmarkId::from_parameter(n_hpg_cells), |b| {
            b.iter(|| {
                let mut n_valid_samples = 0;
                let mut n_samples = 0;
                while n_valid_samples < N_VALID_SAMPLES {
                    let transf = sampler.sample(&mut rng);
                    if !layout.cde().surrogate_collides(surrogate, &transf, &[]) {
                        buffer_shape.transform_from(item.shape(), &transf);
                        if !layout.cde().shape_collides(&buffer_shape, &[]) {
                            n_valid_samples += 1;
                        }
                    }
                    n_samples += 1;
                }
                //println!("n_samples: {}", n_samples);
            })
        });
    }
    group.finish();
}