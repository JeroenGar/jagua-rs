use std::fs::File;
use std::io::BufReader;

use crate::util::{N_ITEMS_REMOVED, SWIM_PATH, create_base_config};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use jagua_rs::collision_detection::hazards::filter::NoHazardFilter;
use jagua_rs::entities::general::Instance;
use jagua_rs::entities::strip_packing::SPPlacement;
use jagua_rs::entities::strip_packing::SPProblem;
use jagua_rs::geometry::geo_traits::Shape;
use jagua_rs::geometry::geo_traits::TransformableFrom;
use jagua_rs::io::json_instance::JsonInstance;
use lbf::samplers::hpg_sampler::HPGSampler;
use rand::SeedableRng;
use rand::rngs::SmallRng;

criterion_main!(benches);
criterion_group!(benches, hpg_update_bench, hpg_query_bench);

mod util;

const N_HPG_CELLS: [usize; 6] = [100, 500, 1000, 2000, 5000, 10000];
const SELECTED_ITEM_ID: usize = 1; // relatively small and "round" item, guaranteed to find valid samples even without HPG

const N_VALID_SAMPLES: usize = 1000;

fn hpg_query_bench(c: &mut Criterion) {
    //HPG density has side effects on the LBF optimize, so we create a single problem instance and create a solution from it.
    let json_instance: JsonInstance =
        serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let base_config = create_base_config();
    let base_instance = util::create_instance(
        &json_instance,
        base_config.cde_config,
        base_config.poly_simpl_tolerance,
    );
    let (base_problem, _) =
        util::create_lbf_problem(base_instance.clone(), base_config, N_ITEMS_REMOVED);
    let base_p_opts = base_problem
        .layout
        .placed_items()
        .values()
        .map(|pi| SPPlacement {
            item_id: pi.item_id,
            d_transf: pi.d_transf,
        })
        .collect_vec();

    let mut group = c.benchmark_group("hpg_bench_query");
    for n_hpg_cells in N_HPG_CELLS {
        let mut config = base_config;
        config.cde_config.hpg_n_cells = n_hpg_cells;
        //create the instance and problem with the specific HPG config
        let instance = util::create_instance(
            &json_instance,
            config.cde_config,
            config.poly_simpl_tolerance,
        );
        let mut problem = SPProblem::new(
            instance.clone(),
            base_problem.strip_width(),
            config.cde_config,
        );
        // Place the items in exactly the same way as the base problem
        for p_opt in base_p_opts.iter() {
            problem.place_item(*p_opt);
        }

        /*{
            let draw_options = SvgDrawOptions {
                quadtree: false,
                surrogate: false,
                hpg: true,
                ..SvgDrawOptions::default()
            };
            let svg = io::layout_to_svg::layout_to_svg(problem.get_layout(LayoutIndex::Existing(0)), &instance, draw_options);
            io::write_svg(&svg, Path::new(&format!("removed_items_{n_hpg_cells}.svg")));
        }*/

        let mut rng = SmallRng::seed_from_u64(0);

        // Search N_VALID_SAMPLES for each item
        let item = instance.item(SELECTED_ITEM_ID);
        let layout = &problem.layout;
        let surrogate = item.shape.surrogate();
        let mut buffer_shape = item.shape.as_ref().clone();
        let mut sampler = HPGSampler::new(item, layout.cde()).unwrap();
        println!(
            "[{}] sampler coverage: {:.3}% with {} samplers",
            n_hpg_cells,
            sampler.coverage_area / layout.bin.bbox().area() * 100.0,
            sampler.cell_samplers.len()
        );

        group.bench_function(BenchmarkId::from_parameter(n_hpg_cells), |b| {
            b.iter(|| {
                let mut n_valid_samples = 0;
                while n_valid_samples < N_VALID_SAMPLES {
                    let dtransf = sampler.sample(&mut rng);
                    let transf = dtransf.compose();
                    if !layout
                        .cde()
                        .surrogate_collides(surrogate, &transf, &NoHazardFilter)
                    {
                        buffer_shape.transform_from(&item.shape, &transf);
                        if !layout.cde().poly_collides(&buffer_shape, &NoHazardFilter) {
                            n_valid_samples += 1;
                        }
                    }
                }
            })
        });
    }
    group.finish();
}

fn hpg_update_bench(c: &mut Criterion) {
    //HPG density has side effects on the LBF optimize, so we create a single problem instance and create a solution from it.
    let json_instance: JsonInstance =
        serde_json::from_reader(BufReader::new(File::open(SWIM_PATH).unwrap())).unwrap();
    let base_config = create_base_config();
    let base_instance = util::create_instance(
        &json_instance,
        base_config.cde_config,
        base_config.poly_simpl_tolerance,
    );
    let (base_problem, _) =
        util::create_lbf_problem(base_instance.clone(), base_config, N_ITEMS_REMOVED);
    let base_p_opts = base_problem
        .layout
        .placed_items()
        .values()
        .map(|pi| SPPlacement {
            item_id: pi.item_id,
            d_transf: pi.d_transf,
        })
        .collect_vec();

    let mut group = c.benchmark_group("hpg_bench_update");
    for n_hpg_cells in N_HPG_CELLS {
        let mut config = base_config;
        config.cde_config.hpg_n_cells = n_hpg_cells;
        //create the instance and problem with the specific HPG config
        let instance = util::create_instance(
            &json_instance,
            config.cde_config,
            config.poly_simpl_tolerance,
        );
        let mut problem = SPProblem::new(
            instance.clone(),
            base_problem.strip_width(),
            config.cde_config,
        );
        // Place the items in exactly the same way as the base problem
        for p_opt in base_p_opts.iter() {
            problem.place_item(*p_opt);
        }

        /*{
            let draw_options = SvgDrawOptions {
                quadtree: false,
                surrogate: false,
                haz_prox_grid: true,
                ..SvgDrawOptions::default()
            };
            let svg = io::layout_to_svg::layout_to_svg(problem.get_layout(LayoutIndex::Existing(0)), &instance, draw_options);
            io::write_svg(&svg, Path::new(&format!("removed_items_{n_hpg_cells}.svg")));
        }*/

        let mut rng = SmallRng::seed_from_u64(0);

        // Search N_VALID_SAMPLES for each item
        let item = instance.item(SELECTED_ITEM_ID);
        let layout = &problem.layout;
        let surrogate = item.shape.surrogate();
        let mut buffer_shape = item.shape.as_ref().clone();
        let mut sampler = HPGSampler::new(item, layout.cde()).unwrap();
        println!(
            "[{}] sampler coverage: {:.3}% with {} samplers",
            n_hpg_cells,
            sampler.coverage_area / layout.bin.bbox().area() * 100.0,
            sampler.cell_samplers.len()
        );

        //collect N_VALID_SAMPLES
        let mut valid_placements = vec![];
        while valid_placements.len() < N_VALID_SAMPLES {
            let dtransf = sampler.sample(&mut rng);
            let transf = dtransf.compose();
            if !layout
                .cde()
                .surrogate_collides(surrogate, &transf, &NoHazardFilter)
            {
                buffer_shape.transform_from(&item.shape, &transf);
                if !layout.cde().poly_collides(&buffer_shape, &NoHazardFilter) {
                    let d_transf = transf.decompose();
                    valid_placements.push(SPPlacement {
                        item_id: SELECTED_ITEM_ID,
                        d_transf,
                    });
                }
            }
        }

        let mut valid_samples_cycler = valid_placements.iter().cycle();

        group.bench_function(BenchmarkId::from_parameter(n_hpg_cells), |b| {
            b.iter(|| {
                let opt = valid_samples_cycler.next().unwrap();
                let pkey = problem.place_item(*opt);
                problem.remove_item(pkey, true);
                problem.flush_changes();
            })
        });
    }
    group.finish();
}
