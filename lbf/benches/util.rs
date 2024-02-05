use std::sync::Arc;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use jaguars::entities::instance::{Instance, PackingType};
use jaguars::entities::problems::problem::ProblemEnum;
use jaguars::entities::problems::sp_problem::SPProblem;
use jaguars::io::json_instance::JsonInstance;
use jaguars::io::parser::Parser;
use jaguars::util::config::CDEConfig;
use jaguars::util::polygon_simplification::PolySimplConfig;
use lbf::config::Config;
use lbf::lbf_optimizer::LBFOptimizer;

pub const SWIM_PATH: &str = "../assets/swim.json";
pub fn create_instance(json_instance: &JsonInstance, cde_config: CDEConfig, poly_simpl_config: PolySimplConfig) -> Arc<Instance> {
    let parser = Parser::new(poly_simpl_config, cde_config, true);
    Arc::new(parser.parse(json_instance))
}

pub fn create_blf_problem(instance: Arc<Instance>, config: Config) -> SPProblem {
    assert!(matches!(instance.packing_type(), PackingType::StripPacking {..}));
    let rng = SmallRng::seed_from_u64(0);
    let mut lbf_optimizer = LBFOptimizer::new(instance, config, rng);
    lbf_optimizer.solve();

    match lbf_optimizer.problem().clone() {
        ProblemEnum::SPProblem(sp_problem) => sp_problem,
        _ => panic!("Expected SPProblem")
    }
}
