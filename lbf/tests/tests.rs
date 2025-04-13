#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::path::Path;

    use jagua_rs::entities::bin_packing::BPInstance;
    use jagua_rs::entities::strip_packing::SPInstance;
    use jagua_rs::io::parser::Parser;
    use jagua_rs::util::PolySimplConfig;
    use lbf::config::LBFConfig;
    use lbf::io;
    use lbf::opt::lbf_opt_bpp::LBFOptimizerBP;
    use lbf::opt::lbf_opt_spp::LBFOptimizerSP;
    use rand::SeedableRng;
    use rand::prelude::IteratorRandom;
    use rand::prelude::SmallRng;
    use test_case::test_case;

    const N_ITEMS_TO_REMOVE: usize = 5;

    #[test_case("../assets/swim.json"; "swim")]
    #[test_case("../assets/shirts.json"; "shirts")]
    #[test_case("../assets/trousers.json"; "trousers")]
    #[test_case("../assets/mao.json"; "mao")]
    #[test_case("../assets/albano.json"; "albano")]
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
        let any_instance = instance.as_ref() as &dyn Any;
        if let Some(sp_instance) = any_instance.downcast_ref::<SPInstance>() {
            test_strip_packing(sp_instance.clone(), config);
        } else if let Some(bp_instance) = any_instance.downcast_ref::<BPInstance>() {
            test_bin_packing(bp_instance.clone(), config);
        } else {
            panic!("Unknown instance type");
        }
    }

    fn test_strip_packing(instance: SPInstance, config: LBFConfig) {
        let mut opt = LBFOptimizerSP::new(instance, config, SmallRng::seed_from_u64(0));

        let mut rng = SmallRng::seed_from_u64(0);

        // a first lbf run
        opt.solve();
        {
            // remove some items
            let problem = &mut opt.problem;
            for _ in 0..N_ITEMS_TO_REMOVE {
                //pick random existing layout
                let random_placed_item = problem
                    .layout
                    .placed_items()
                    .iter()
                    .choose(&mut rng)
                    .map(|(key, _)| key);

                if let Some(random_placed_item) = random_placed_item {
                    // remove the item
                    problem.remove_item(random_placed_item, false);
                } else {
                    // no items to remove
                    break;
                }
            }
            problem.flush_changes();

            let solution = opt.problem.save();
            // second optimization run
            opt.solve();
            // restore the solution
            opt.problem.restore(&solution);
            // third optimization run
            opt.solve();
        }
    }

    fn test_bin_packing(instance: BPInstance, config: LBFConfig) {
        let mut opt = LBFOptimizerBP::new(instance, config, SmallRng::seed_from_u64(0));

        let mut rng = SmallRng::seed_from_u64(0);

        // a first optimization run
        opt.solve();

        {
            // remove some items
            let problem = &mut opt.problem;
            for _ in 0..N_ITEMS_TO_REMOVE {
                //pick random existing layout
                let lkey = problem.layouts.keys().choose(&mut rng).unwrap();
                let random_placed_item = problem.layouts[lkey]
                    .placed_items()
                    .iter()
                    .choose(&mut rng)
                    .map(|(key, _)| key);

                if let Some(random_placed_item) = random_placed_item {
                    // remove the item
                    problem.remove_item(lkey, random_placed_item, false);
                } else {
                    // no items to remove
                    break;
                }
            }
            // flush changes
            problem.flush_changes();

            let solution = opt.problem.save();
            // second optimization run
            opt.solve();
            // restore the solution
            opt.problem.restore(&solution);
            // third optimization run
            opt.solve();
        }
    }
}
