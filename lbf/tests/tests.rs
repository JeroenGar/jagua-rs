#[cfg(test)]
mod tests {
    use std::path::Path;
    use jagua_rs::io::import::Importer;
    use lbf::config::LBFConfig;
    use lbf::io::{read_bpp_instance, read_spp_instance};
    use lbf::opt::lbf_bpp::LBFOptimizerBP;
    use lbf::opt::lbf_spp::LBFOptimizerSP;
    use rand::prelude::IteratorRandom;
    use rand::prelude::SmallRng;
    use rand::SeedableRng;
    use test_case::test_case;
    use jagua_rs::prob_variants::{bpp, spp};
    use anyhow::Result;

    const N_ITEMS_TO_REMOVE: usize = 5;
    
    #[test_case("../assets/swim.json"; "swim")]
    #[test_case("../assets/shirts.json"; "shirts")]
    #[test_case("../assets/trousers.json"; "trousers")]
    #[test_case("../assets/mao.json"; "mao")]
    #[test_case("../assets/albano.json"; "albano")]
    fn test_strip_packing(instance_path: &str) -> Result<()> {
        let ext_instance = read_spp_instance(Path::new(instance_path))?;
        let instance = spp::io::import(&importer(), &ext_instance)?;

        let mut opt = LBFOptimizerSP::new(instance, config(), SmallRng::seed_from_u64(0));

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

            let solution = opt.problem.save();
            // second optimization run
            opt.solve();
            // restore the solution
            opt.problem.restore(&solution);
            // third optimization run
            opt.solve();
        }
        Ok(())
    }

    #[test_case("../assets/baldacci1.json"; "baldacci1")]
    #[test_case("../assets/baldacci2.json"; "baldacci2")]
    #[test_case("../assets/baldacci3.json"; "baldacci3")]
    #[test_case("../assets/baldacci4.json"; "baldacci4")]
    #[test_case("../assets/baldacci5.json"; "baldacci5")]
    #[test_case("../assets/baldacci6.json"; "baldacci6")]
    fn test_bin_packing(instance_path: &str) -> Result<()> {
        let ext_instance = read_bpp_instance(Path::new(instance_path))?;
        let instance = bpp::io::import(&importer(), &ext_instance)?;

        let mut opt = LBFOptimizerBP::new(instance, config(), SmallRng::seed_from_u64(0));

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

            let solution = opt.problem.save();
            // second optimization run
            opt.solve();
            // restore the solution
            opt.problem.restore(&solution);
            // third optimization run
            opt.solve();
        }
        Ok(())
    }


    fn config() -> LBFConfig {
        LBFConfig {
            n_samples: 100,
            ..LBFConfig::default()
        }
    }

    fn importer() -> Importer {
        Importer::new(
            config().cde_config,
            config().poly_simpl_tolerance,
            config().min_item_separation,
        )
    }
}
