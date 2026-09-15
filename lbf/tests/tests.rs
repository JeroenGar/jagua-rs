#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    use jagua_rs::io::import::Importer;
    use jagua_rs::probs::{bpp, spp};
    use lbf::config::LBFConfig;
    use lbf::io::{read_bpp_instance, read_spp_instance};
    use lbf::opt::lbf_bpp::LBFOptimizerBP;
    use lbf::opt::lbf_spp::LBFOptimizerSP;
    use rand::SeedableRng;
    use rand::prelude::IteratorRandom;
    use rand::prelude::SmallRng;
    use std::path::Path;
    use test_case::test_case;

    const N_ITEMS_TO_REMOVE: usize = 5;

    const QT_DEPTHS: [u8; 3] = [0, 3, 10];

    #[test_case("../assets/albano.json"; "albano")]
    #[test_case("../assets/blaz1.json"; "blaz1")]
    #[test_case("../assets/dagli.json"; "dagli")]
    #[test_case("../assets/fu.json"; "fu")]
    #[test_case("../assets/jakobs1.json"; "jakobs1")]
    #[test_case("../assets/jakobs2.json"; "jakobs2")]
    #[test_case("../assets/mao.json"; "mao")]
    #[test_case("../assets/marques.json"; "marques")]
    #[test_case("../assets/shapes0.json"; "shapes0")]
    #[test_case("../assets/shapes1.json"; "shapes1")]
    #[test_case("../assets/shirts.json"; "shirts")]
    #[test_case("../assets/swim.json"; "swim")]
    #[test_case("../assets/trousers.json"; "trousers")]
    fn test_strip_packing(instance_path: &str) -> Result<()> {
        let ext_instance = read_spp_instance(Path::new(instance_path))?;
        let instance = spp::io::import_instance(&importer(), &ext_instance)?;

        for qt_depth in QT_DEPTHS {
            let mut config = config();
            config.cde_config.quadtree_depth = qt_depth;

            let mut opt =
                LBFOptimizerSP::new(instance.clone(), config, SmallRng::seed_from_u64(0))?;

            let mut rng = SmallRng::seed_from_u64(0);

            // a first lbf run
            opt.solve()?;
            {
                // remove some items
                let problem = &mut opt.problem;
                for _ in 0..N_ITEMS_TO_REMOVE {
                    //pick random existing layout
                    let random_placed_item = problem
                        .layout
                        .placed_items
                        .iter()
                        .choose(&mut rng)
                        .map(|(key, _)| key);

                    if let Some(random_placed_item) = random_placed_item {
                        // remove the item
                        problem.remove_item(random_placed_item);
                    } else {
                        // no items to remove
                        break;
                    }
                }

                let solution = opt.problem.save();
                // second optimization run
                opt.solve()?;
                // restore the solution
                opt.problem.restore(&solution);
                // third optimization run
                opt.solve()?;
            }
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
        let instance = bpp::io::import_instance(&importer(), &ext_instance)?;

        for qt_depth in QT_DEPTHS {
            let mut config = config();
            config.cde_config.quadtree_depth = qt_depth;
            let mut opt = LBFOptimizerBP::new(instance.clone(), config, SmallRng::seed_from_u64(0));

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
                        .placed_items
                        .iter()
                        .choose(&mut rng)
                        .map(|(key, _)| key);

                    if let Some(random_placed_item) = random_placed_item {
                        // remove the item
                        problem.remove_item(lkey, random_placed_item);
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
        }
        Ok(())
    }

    #[test]
    fn test_rejects_self_intersecting_polygon() -> Result<()> {
        let ext_instance = read_spp_instance(Path::new("../assets/self_intersecting.json"))?;

        assert!(spp::io::import_instance(&importer(), &ext_instance).is_err());
        Ok(())
    }

    #[test]
    fn test_inflate_all_dataset_items() -> Result<()> {
        #[derive(serde::Deserialize)]
        struct Dataset {
            items: Vec<jagua_rs::io::ext_repr::ExtItem>,
        }

        let importer =
            Importer::new(config().cde_config, None, None).with_min_item_separation(2.0)?;
        let mut n_items = 0;
        for entry in std::fs::read_dir("../assets")? {
            let path = entry?.path();
            if path.extension().is_none_or(|ext| ext != "json")
                || matches!(
                    path.file_name().and_then(|name| name.to_str()),
                    Some("config_lbf.json" | "self_intersecting.json")
                )
            {
                // Configuration and the intentional invalid-input fixture aren't datasets.
                continue;
            }
            let dataset: Dataset = serde_json::from_reader(std::fs::File::open(&path)?)
                .with_context(|| path.display().to_string())?;
            for (idx, item) in dataset.items.into_iter().enumerate() {
                importer.import_item(&item, idx).with_context(|| {
                    format!("{}: failed to inflate item {}", path.display(), item.id)
                })?;
                n_items += 1;
            }
        }
        assert!(n_items > 0);
        println!("Inflated and imported {n_items} dataset items");
        Ok(())
    }

    #[test]
    fn item_holes_are_rejected_but_container_holes_are_preserved() -> Result<()> {
        use jagua_rs::io::ext_repr::{ExtContainer, ExtPolygon, ExtSPolygon, ExtShape};

        let mut input = read_spp_instance(Path::new("../assets/fu.json"))?;
        let polygon = ExtPolygon {
            outer: ExtSPolygon(vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)]),
            inner: vec![ExtSPolygon(vec![(2.0, 2.0), (3.0, 2.0), (2.0, 3.0)])],
        };
        input.items[0].base.shape = ExtShape::Polygon(polygon.clone());
        assert!(spp::io::import_instance(&importer(), &input).is_err());
        let container = importer().import_container(&ExtContainer {
            id: 0,
            shape: ExtShape::Polygon(polygon.clone()),
            zones: vec![],
        })?;
        assert_eq!(
            container.quality_zones[0].as_ref().unwrap().shapes_cd.len(),
            1
        );
        input.items[0].base.shape = ExtShape::Polygon(ExtPolygon {
            inner: vec![],
            ..polygon
        });
        assert!(spp::io::import_instance(&importer(), &input).is_ok());
        Ok(())
    }

    #[test]
    fn instance_separation_controls_item_and_container_geometry() -> Result<()> {
        let mut small: spp::io::ext_repr::ExtSPInstance =
            serde_json::from_value(serde_json::json!({
                "name": "small separated items", "strip_height": 2.0,
                "min_item_separation": 0.5,
                "items": [{"id": 0, "demand": 2, "shape": {"type": "rectangle",
                    "data": {"x_min": 0, "y_min": 0, "width": 0.1, "height": 0.1}}}]
            }))?;
        let small_instance = spp::io::import_instance(&importer(), &small)?;
        let mut small_problem = spp::entities::SPProblem::new(small_instance)?;
        let before = small_problem.save();
        assert!(small_problem.change_strip_width(0.02).is_err());
        assert_eq!(small_problem.strip, before.strip);
        assert!(jagua_rs::util::assertions::snapshot_matches_layout(
            &small_problem.layout,
            &before.layout_snapshot
        ));
        let mut optimizer = LBFOptimizerSP::new(
            small_problem.instance.clone(),
            config(),
            SmallRng::seed_from_u64(0),
        )?;
        optimizer.solve()?;
        assert!(optimizer.problem.layout.is_feasible());
        small.min_item_separation = 2.0;
        assert!(spp::io::import_instance(&importer(), &small).is_err());

        let mut input = read_spp_instance(Path::new("../assets/fu.json"))?;
        assert_eq!(input.min_item_separation, 0.0);
        let plain = spp::io::import_instance(&importer(), &input)?;
        assert_eq!(plain.item(0).shape_orig.modify_config.offset, None);

        input.min_item_separation = 2.0;
        let spaced = spp::io::import_instance(&importer(), &input)?;
        assert_eq!(spaced.item(0).shape_orig.modify_config.offset, Some(1.0));
        let problem = jagua_rs::probs::spp::entities::SPProblem::new(spaced)?;
        assert_eq!(
            problem.layout.container.outer_orig.modify_config.offset,
            Some(1.0)
        );

        for invalid in [-1.0, f32::INFINITY, f32::NAN] {
            input.min_item_separation = invalid;
            assert!(spp::io::import_instance(&importer(), &input).is_err());
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
            config().narrow_concavity_cutoff,
        )
    }
}
