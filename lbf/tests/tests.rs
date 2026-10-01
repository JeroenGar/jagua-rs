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
    #[test_case("../assets/reflection.json"; "reflection")]
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
                        .layout()
                        .placed_items()
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
                    let lkey = problem.layouts().keys().choose(&mut rng).unwrap();
                    let random_placed_item = problem.layouts()[lkey]
                        .placed_items()
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
    fn quality_zones_have_surrogates_and_filtering_shares_shapes() -> Result<()> {
        use jagua_rs::io::ext_repr::{ExtContainer, ExtQualityZone, ExtShape};
        use std::sync::Arc;

        let rect = |x_min| ExtShape::Rectangle {
            x_min,
            y_min: 2.0,
            width: 1.0,
            height: 1.0,
        };
        let container = importer().import_container(&ExtContainer {
            id: 0,
            shape: ExtShape::Rectangle {
                x_min: 0.0,
                y_min: 0.0,
                width: 10.0,
                height: 10.0,
            },
            zones: [2.0, 6.0]
                .map(|x_min| ExtQualityZone {
                    quality: 0,
                    shape: rect(x_min),
                })
                .into(),
        })?;
        let zone = container.quality_zones()[0].as_ref().unwrap();
        assert!(
            zone.shapes_cd()
                .iter()
                .all(|s| !s.surrogate().poles.is_empty())
        );

        let filtered = zone.filtered(|shape| shape.bbox().x_min > 5.0);
        assert_eq!(filtered.shapes_cd().len(), 1);
        assert!(Arc::ptr_eq(&filtered.shapes_cd()[0], &zone.shapes_cd()[1]));
        assert!(Arc::ptr_eq(
            &filtered.shapes_orig()[0],
            &zone.shapes_orig()[1]
        ));
        Ok(())
    }

    #[test]
    fn item_holes_are_rejected_but_container_holes_are_preserved() -> Result<()> {
        use jagua_rs::entities::{InferiorQualityZone, N_QUALITIES};
        use jagua_rs::geometry::fail_fast::SPSurrogateConfig;
        use jagua_rs::io::ext_repr::{
            ExtContainer, ExtPolygon, ExtQualityZone, ExtSPolygon, ExtShape,
        };

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
            container.quality_zones()[0]
                .as_ref()
                .unwrap()
                .shapes_cd()
                .len(),
            1
        );
        let mut external_container = ExtContainer {
            id: 0,
            shape: ExtShape::Polygon(polygon.clone()),
            zones: vec![ExtQualityZone {
                quality: 1,
                shape: ExtShape::SimplePolygon(polygon.inner[0].clone()),
            }],
        };
        assert!(importer().import_container(&external_container).is_ok());
        for quality in [N_QUALITIES, usize::MAX] {
            external_container.zones[0].quality = quality;
            assert!(importer().import_container(&external_container).is_err());
            assert!(InferiorQualityZone::new(quality, vec![], SPSurrogateConfig::none()).is_err());
        }
        external_container.zones[0].quality = 1;
        for shape in [
            ExtShape::Polygon(polygon.clone()),
            ExtShape::MultiPolygon(vec![polygon.clone()]),
        ] {
            external_container.zones[0].shape = shape;
            assert!(importer().import_container(&external_container).is_err());
        }
        external_container.zones.clear();
        external_container.shape = ExtShape::MultiPolygon(vec![polygon.clone()]);
        assert!(importer().import_container(&external_container).is_err());

        input.items[0].base.shape = ExtShape::Polygon(ExtPolygon {
            inner: vec![],
            ..polygon
        });
        assert!(spp::io::import_instance(&importer(), &input).is_ok());
        for quality in [0, N_QUALITIES - 1] {
            input.items[0].base.min_quality = Some(quality);
            assert!(importer().import_item(&input.items[0].base, 0).is_ok());
        }
        for quality in [N_QUALITIES, usize::MAX] {
            input.items[0].base.min_quality = Some(quality);
            assert!(importer().import_item(&input.items[0].base, 0).is_err());
        }
        Ok(())
    }

    #[test]
    fn quality_zones_filter_queries_and_layouts_consistently() -> Result<()> {
        use jagua_rs::collision_detection::hazards::filter::{HazKeyFilter, NoFilter};
        use jagua_rs::entities::Layout;
        use jagua_rs::geometry::DTransformation;
        use jagua_rs::geometry::geo_traits::Transformable;
        use jagua_rs::io::ext_repr::{ExtContainer, ExtItem};
        use serde_json::json;

        let rectangle = |x, y, width, height| {
            json!({
                "type": "rectangle",
                "data": {"x_min": x, "y_min": y, "width": width, "height": height}
            })
        };
        let external: ExtContainer = serde_json::from_value(json!({
            "id": 0, "shape": rectangle(0, 0, 30, 12),
            "zones": ([0, 2, 3, 4].into_iter().enumerate().map(|(i, q)| json!({
                "quality": q, "shape": rectangle(2 + 6 * i, 2, 4, 6)
            })).collect::<Vec<_>>())
        }))?;
        let importer = importer();
        let container = importer.import_container(&external)?;
        // This zone covers complete quadtree nodes, including the query's virtual root.
        let large_zone: ExtContainer = serde_json::from_value(json!({
            "id": 0, "shape": rectangle(0, 0, 100, 100),
            "zones": [{"quality": 5, "shape": rectangle(0, 0, 60, 60)}]
        }))?;
        for required in [Some(3), None] {
            let input: ExtItem = serde_json::from_value(json!({
                "id": 0, "min_quality": required,
                "orientation": {"rotation": {"mode": "discrete", "angles": [0]}},
                "shape": rectangle(0, 0, 2, 2)
            }))?;
            let item = importer.import_item(&input, 0)?;
            let mut layout = Layout::new(container.clone());
            for (quality, x) in [(0, 2.0), (2, 8.0), (3, 14.0), (4, 20.0)] {
                // Exercise containment and crossing a zone boundary.
                for dx in [2.0, 0.5] {
                    let pose = DTransformation::new(0.0, (x + dx, 5.0));
                    let shape = item.shape_cd().transform_clone(&pose.compose());
                    let expected_collision = required.is_none() || quality < 3;
                    let cde = layout.cde();
                    let collision = match required {
                        Some(q) => cde.detect_poly_collision(
                            &shape,
                            &HazKeyFilter::from_irrelevant_qzones(q, cde.hazards_map()),
                        ),
                        None => cde.detect_poly_collision(&shape, &NoFilter),
                    };
                    assert_eq!(
                        collision, expected_collision,
                        "quality {quality}, required {required:?}"
                    );
                    let key = layout.place_item(&item, pose);
                    assert_eq!(layout.is_collision_free(), !expected_collision);
                    layout.remove_item(key);
                }
            }
            // Quality filtering must not hide other items or the exterior.
            let pose = DTransformation::new(0.0, (27.0, 5.0));
            let first = layout.place_item(&item, pose);
            assert!(layout.is_collision_free());
            let second = layout.place_item(&item, pose);
            assert!(!layout.is_collision_free());
            layout.remove_item(second);
            layout.remove_item(first);
            layout.place_item(&item, DTransformation::new(0.0, (-1.0, 5.0)));
            assert!(!layout.is_collision_free());
            let mut covered = Layout::new(importer.import_container(&large_zone)?);
            let mut small_input = input.clone();
            small_input.shape = jagua_rs::io::ext_repr::ExtShape::Rectangle {
                x_min: 0.0,
                y_min: 0.0,
                width: 0.5,
                height: 0.5,
            };
            let small_item = importer.import_item(&small_input, 0)?;
            let pose = DTransformation::new(0.0, (4.0, 4.0));
            let shape = small_item.shape_cd().transform_clone(&pose.compose());
            let filter = HazKeyFilter::from_irrelevant_qzones(
                required.unwrap_or(jagua_rs::entities::N_QUALITIES),
                covered.cde().hazards_map(),
            );
            assert_eq!(
                covered.cde().detect_poly_collision(&shape, &filter),
                required.is_none()
            );
            covered.place_item(&small_item, pose);
            assert_eq!(covered.is_collision_free(), required.is_some());
        }
        Ok(())
    }

    #[test]
    fn instance_separation_controls_item_and_container_geometry() -> Result<()> {
        let mut small: spp::io::ext_repr::ExtSPInstance =
            serde_json::from_value(serde_json::json!({
                "name": "small separated items", "strip_height": 2.0,
                "min_item_separation": 0.5,
                "items": [{"id": 0, "demand": 2,
                    "orientation": {"rotation": {"mode": "continuous"}},
                    "shape": {"type": "rectangle",
                    "data": {"x_min": 0, "y_min": 0, "width": 0.1, "height": 0.1}}}]
            }))?;
        let small_instance = spp::io::import_instance(&importer(), &small)?;
        let mut small_problem = spp::entities::SPProblem::new(small_instance)?;
        let width_before = small_problem.strip_width();
        assert!(small_problem.fit_strip().is_err());
        assert_eq!(small_problem.strip_width(), width_before);
        let before = small_problem.save();
        assert!(small_problem.change_strip_width(0.02).is_err());
        assert_eq!(small_problem.strip(), before.strip());
        assert!(jagua_rs::util::assertions::snapshot_matches_layout(
            small_problem.layout(),
            before.layout_snapshot()
        ));
        let mut optimizer = LBFOptimizerSP::new(
            small_problem.instance().clone(),
            config(),
            SmallRng::seed_from_u64(0),
        )?;
        optimizer.solve()?;
        assert!(optimizer.problem.layout().is_collision_free());
        small.min_item_separation = 2.0;
        assert!(spp::io::import_instance(&importer(), &small).is_err());

        let mut input = read_spp_instance(Path::new("../assets/fu.json"))?;
        assert_eq!(input.min_item_separation, 0.0);
        let plain = spp::io::import_instance(&importer(), &input)?;
        assert_eq!(plain.item(0).shape_orig().modify_config.offset, None);

        input.min_item_separation = 2.0;
        let spaced = spp::io::import_instance(&importer(), &input)?;
        assert_eq!(spaced.item(0).shape_orig().modify_config.offset, Some(1.0));
        let problem = jagua_rs::probs::spp::entities::SPProblem::new(spaced)?;
        assert_eq!(
            problem
                .layout()
                .container()
                .outer_orig()
                .modify_config
                .offset,
            Some(1.0)
        );

        for invalid in [-1.0, f32::INFINITY, f32::NAN] {
            input.min_item_separation = invalid;
            assert!(spp::io::import_instance(&importer(), &input).is_err());
        }
        Ok(())
    }

    #[test]
    fn explicit_rotations_import_solve_and_round_trip() -> Result<()> {
        use jagua_rs::geometry::geo_enums::RotationRange;
        use jagua_rs::probs::spp::io::ext_repr::ExtSPInstance;
        use serde_json::json;

        let input = |rotation| {
            json!({
                "name": "rotations", "strip_height": 10,
                "items": [{
                    "id": 42, "demand": 2, "orientation": {"rotation": rotation},
                    "shape": {"type": "rectangle", "data": {
                        "x_min": 0, "y_min": 0, "width": 2, "height": 3
                    }}
                }]
            })
        };
        for (rotation, expected) in [
            (
                json!({"mode": "discrete", "angles": [360, 0, -180, 180]}),
                RotationRange::Discrete(vec![0.0, std::f32::consts::PI]),
            ),
            (
                json!({"mode": "stepped", "step": 180}),
                RotationRange::Discrete(vec![0.0, std::f32::consts::PI]),
            ),
            (json!({"mode": "stepped", "step": 360}), RotationRange::None),
            (json!({"mode": "continuous"}), RotationRange::Continuous),
        ] {
            let external: ExtSPInstance = serde_json::from_value(input(rotation))?;
            let instance = spp::io::import_instance(&importer(), &external)?;
            assert_eq!(
                instance.item(0).allowed_orientations().rotations(),
                &expected
            );
            let round_trip: ExtSPInstance =
                serde_json::from_value(serde_json::to_value(&external)?)?;
            assert_eq!(
                round_trip.items[0].base.orientation,
                external.items[0].base.orientation
            );
            let epoch = jagua_rs::Instant::now();
            let solution =
                LBFOptimizerSP::new(instance.clone(), config(), SmallRng::seed_from_u64(0))?
                    .solve()?;
            let exported = spp::io::export(&solution, epoch);
            let restored = spp::io::import_solution(&instance, &exported)?;
            assert_eq!(restored.layout_snapshot().placed_items().len(), 2);
        }
        for rotation in [
            json!(null),
            json!({"mode": "discrete", "angles": []}),
            json!({"mode": "stepped", "step": 0}),
            json!({"mode": "stepped", "step": 7}),
            json!({"mode": "stepped", "step": 0.0001}),
            json!({"mode": "continuous", "angles": [0]}),
        ] {
            let parsed = serde_json::from_value::<ExtSPInstance>(input(rotation));
            assert!(parsed.is_err() || spp::io::import_instance(&importer(), &parsed?).is_err());
        }
        let mut legacy = input(json!({"mode": "continuous"}));
        legacy["items"][0]["allowed_orientations"] = json!(null);
        assert!(serde_json::from_value::<ExtSPInstance>(legacy).is_err());
        let mut missing = input(json!({"mode": "continuous"}));
        missing["items"][0]
            .as_object_mut()
            .unwrap()
            .remove("orientation");
        assert!(serde_json::from_value::<ExtSPInstance>(missing).is_err());
        let decimal: ExtSPInstance =
            serde_json::from_value(input(json!({"mode": "stepped", "step": 0.1})))?;
        let instance = spp::io::import_instance(&importer(), &decimal)?;
        assert!(
            matches!(instance.item(0).allowed_orientations().rotations(), RotationRange::Discrete(a) if a.len() == 3600)
        );
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
