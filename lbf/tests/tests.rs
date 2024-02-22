#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Once;

    use log::info;
    use rand::{Rng, SeedableRng};
    use rand::prelude::IteratorRandom;
    use rand::prelude::SmallRng;
    use test_case::test_case;

    use jagua_rs::entities::problems::problem_generic::LayoutIndex;
    use jagua_rs::entities::problems::problem_generic::ProblemGeneric;
    use jagua_rs::io::parser::Parser;
    use lbf::config::Config;
    use lbf::io;
    use lbf::lbf_optimizer::LBFOptimizer;

    const N_ITEMS_TO_REMOVE: usize = 5;

    static INIT_LOGGER : Once = Once::new();

    #[test_case("../assets/swim.json"; "swim")]
    #[test_case("../assets/shirts.json"; "shirts")]
    #[test_case("../assets/trousers.json"; "trousers")]
    #[test_case("../assets/mao.json"; "mao.json")]
    #[test_case("../assets/Baldacci/Test1.json"; "Baldacci/Test1")]
    #[test_case("../assets/Baldacci/Test2.json"; "Baldacci/Test2")]
    #[test_case("../assets/Baldacci/Test3.json"; "Baldacci/Test3")]
    #[test_case("../assets/Baldacci/Test4.json"; "Baldacci/Test4")]
    #[test_case("../assets/Baldacci/Test5.json"; "Baldacci/Test5")]
    #[test_case("../assets/Baldacci/Test6.json"; "Baldacci/Test6")]
    fn test_instance(instance_path: &str) {
        INIT_LOGGER.call_once(|| {
            io::init_logger(Some(log::LevelFilter::Info));
        });

        let instance = Path::new(instance_path);

        info!("Testing instance: {:?}", instance);
        // parse the instance
        let mut config = Config::default();
        config.n_samples_per_item = 100;
        let json_instance = io::read_json_instance(&instance);
        let parser = Parser::new(config.poly_simpl_config, config.cde_config, true);
        let instance = parser.parse(&json_instance);

        let mut optimizer = LBFOptimizer::new(instance.clone(), config, SmallRng::seed_from_u64(0));

        let mut rng = SmallRng::seed_from_u64(0);

        // a first optimization run
        optimizer.solve();

        {
            // remove some items
            let problem = &mut optimizer.problem;
            for _ in 0..N_ITEMS_TO_REMOVE {
                //pick random existing layout
                let layout_index = LayoutIndex::Existing(rng.gen_range(0..problem.layouts().len()));
                let random_placed_item = match problem.get_layout(&layout_index).placed_items().iter().choose(&mut rng) {
                    Some(pi) => pi.uid.clone(),
                    None => break,
                };
                // remove the item
                problem.remove_item(layout_index, &random_placed_item, false);
            }
            // flush changes
            problem.flush_changes();
            // second optimization run
            optimizer.solve();
        }
    }
}