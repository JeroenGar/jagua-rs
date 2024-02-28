#[cfg(test)]
mod tests {
    use std::path::Path;

    use rand::prelude::IteratorRandom;
    use rand::prelude::SmallRng;
    use rand::{Rng, SeedableRng};
    use test_case::test_case;

    use jagua_rs::entities::problems::problem_generic::LayoutIndex;
    use jagua_rs::entities::problems::problem_generic::ProblemGeneric;
    use jagua_rs::io::parser::Parser;
    use jagua_rs::util::polygon_simplification::PolySimplConfig;
    use lbf::io;
    use lbf::lbf_config::LBFConfig;
    use lbf::lbf_optimizer::LBFOptimizer;

    const N_ITEMS_TO_REMOVE: usize = 5;

    #[test_case("../assets/swim.json"; "swim")]
    #[test_case("../assets/shirts.json"; "shirts")]
    #[test_case("../assets/trousers.json"; "trousers")]
    #[test_case("../assets/mao.json"; "mao.json")]
    #[test_case("../assets/baldacci1.json"; "baldacci1")]
    #[test_case("../assets/baldacci2.json"; "baldacci2")]
    #[test_case("../assets/baldacci3.json"; "baldacci3")]
    #[test_case("../assets/baldacci4.json"; "baldacci4")]
    #[test_case("../assets/baldacci5.json"; "baldacci5")]
    #[test_case("../assets/baldacci6.json"; "baldacci6")]
    fn test_instance(instance_path: &str) {
        let instance = Path::new(instance_path);
        // parse the instance
        let mut config = LBFConfig::default();
        config.n_samples = 100;
        let json_instance = io::read_json_instance(&instance);
        let poly_simpl_config = match config.poly_simpl_tolerance {
            Some(tolerance) => PolySimplConfig::Enabled { tolerance },
            None => PolySimplConfig::Disabled,
        };

        let parser = Parser::new(poly_simpl_config, config.cde_config, true);
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
                let layout_index = LayoutIndex::Real(rng.gen_range(0..problem.layouts().len()));
                let random_placed_item = match problem
                    .get_layout(&layout_index)
                    .placed_items()
                    .iter()
                    .choose(&mut rng)
                {
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
