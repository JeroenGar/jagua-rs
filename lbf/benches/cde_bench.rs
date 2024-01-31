use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::SmallRng;
use rand::SeedableRng;
use jaguars::entities::instance::Instance;
use jaguars::entities::solution::Solution;
use jaguars::parse::json::json_instance::JsonInstance;
use jaguars::parse::parser::Parser;
use jaguars::simplification::simplification_config::PolySimplConfig;
use jaguars::util::config::{CDEConfig, HazProxConfig, QuadTreeConfig, SPSurrogateConfig};
use lbf::config::Config;
use lbf::lbf_optimizer::LBFOptimizer;

criterion_main!(benches);
criterion_group!(benches, test_bench);

fn test_bench(c: &mut Criterion) {
    let config = Config{
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
    };


    let json_instance : JsonInstance = serde_json::from_reader(BufReader::new(File::open("assets/shirts.json").unwrap())).unwrap();
    let instance = create_instance(&json_instance, &config);
    let blf_solution = create_blf_solution(instance);

    //c.bench_function("")
}




fn create_instance(json_instance: &JsonInstance, config: &Config) -> Arc<Instance> {
    let parser = Parser::new(config.poly_simpl_config, config.cde_config, true);
    Arc::new(parser.parse(json_instance))
}

fn create_blf_solution(instance: Arc<Instance>) -> Solution {
    let rng = SmallRng::seed_from_u64(0);
    let mut lbf_optimizer = LBFOptimizer::new(instance, Config::default(), rng);
    lbf_optimizer.solve()
}
