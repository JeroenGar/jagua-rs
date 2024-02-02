use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use jaguars::entities::instance::Instance;
use jaguars::entities::solution::Solution;
use jaguars::io::json::json_instance::JsonInstance;
use jaguars::io::parser::Parser;
use jaguars::simplification::simplification_config::PolySimplConfig;
use jaguars::util::config::{CDEConfig, HazProxConfig, QuadTreeConfig, SPSurrogateConfig};
use lbf::config::Config;
use lbf::lbf_optimizer::LBFOptimizer;

criterion_main!(benches);
criterion_group!(benches, test_bench);

fn quadtree_update_bench(c: &mut Criterion) {
    let (instance, mut layout) = create_benchmark_instance_and_layout();
    let mut rng = SmallRng::seed_from_u64(0);

    c.bench_function("quadtree_update", |b| {
        b.iter(|| {
            let remove_items_uids = remove_items(N_ITEMS_REMOVED, &mut layout, &mut rng);
            insert_items(&mut layout, &remove_items_uids, &instance);
        })
    });
}

fn quadtree_query_bench(c: &mut Criterion) {
    let (instance, mut layout) = create_benchmark_instance_and_layout();
    let mut rng = SmallRng::seed_from_u64(0);

    remove_items(N_ITEMS_REMOVED, &mut layout, &mut rng);

    c.bench_function("quadtree_query", |b| {
        b.iter(|| {
            let random_item = rng.gen_range(0..instance.items().len());
            query(instance.item(random_item), &layout, N_SAMPLES_PER_ITEM, &mut rng);
        })
    });
}

fn get_config() -> Config {
    Config{
        cde_config: CDEConfig {
            quadtree: QuadTreeConfig::FixedDepth(0),
            haz_prox: HazProxConfig::Disabled,
            item_surrogate_config: SPSurrogateConfig {
                pole_coverage_goal: 0.0,
                max_poles: 0,
                n_ff_poles: 0,
                n_ff_piers: 0,
            },
        },
        poly_simpl_config: PolySimplConfig::Disabled,
        deterministic_mode: true,
        n_samples_per_item: 1000,
        ls_samples_fraction: 0.5,
        svg_draw_options: Default::default(),
    }
}


fn create_instance(json_instance: &JsonInstance, cde_config: CDEConfig, poly_simpl_config: PolySimplConfig) -> Arc<Instance> {
    let parser = Parser::new(poly_simpl_config, cde_config, true);
    Arc::new(parser.parse(json_instance))
}

fn create_blf_solution(instance: Arc<Instance>) -> Solution {
    let rng = SmallRng::seed_from_u64(0);
    let mut lbf_optimizer = LBFOptimizer::new(instance, Config::default(), rng);
    lbf_optimizer.solve()
}
