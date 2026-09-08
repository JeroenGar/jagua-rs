mod geometry {
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

    use jagua_rs::geometry::convex_hull::{convex_hull_from_points, convex_hull_from_surrogate};
    use jagua_rs::geometry::fail_fast::SPSurrogateConfig;
    use jagua_rs::geometry::geo_enums::RotationRange;
    use jagua_rs::geometry::geo_traits::{
        CollidesWith, DistanceTo, Transformable, TransformableFrom,
    };
    use jagua_rs::geometry::primitives::{Point, SPolygon};
    use jagua_rs::geometry::{
        AllowedOrientations, DTransformation, Transformation, normalize_rotation,
    };

    fn assert_point(actual: Point, expected: Point) {
        assert!(
            actual.distance_to(&expected) < 0.0001,
            "{actual:?} != {expected:?}"
        );
    }

    #[test]
    fn reflected_transform_composition_and_inverse() {
        let point = Point(2.0, 3.0);
        let mirror = DTransformation::empty().with_reflection(true).compose();
        assert_point(point.transform_clone(&mirror), Point(2.0, -3.0));
        assert!(mirror.clone().transform(&mirror).is_empty());
        assert!(!DTransformation::default().reflected);
        assert_ne!(
            DTransformation::empty(),
            DTransformation::empty().with_reflection(true)
        );

        for reflected in [false, true] {
            for r in [0.0, FRAC_PI_2, PI, -0.73] {
                let dt = DTransformation::new(r, (7.0, -11.0)).with_reflection(reflected);
                let t = dt.compose();
                let y = if reflected { -point.1 } else { point.1 };
                let expected = Point(
                    r.cos() * point.0 - r.sin() * y + 7.0,
                    r.sin() * point.0 + r.cos() * y - 11.0,
                );
                assert_point(point.transform_clone(&t), expected);
                assert_eq!(t.is_reflected(), reflected);
                assert_eq!(t.decompose().reflected, reflected);
                assert_point(point.transform_clone(&t.decompose().compose()), expected);
                assert_point(expected.transform_clone(&t.clone().inverse()), point);
                for preceding in [Transformation::from_translation((5.0, 9.0)), mirror.clone()] {
                    let sequential = point.transform_clone(&preceding).transform_clone(&t);
                    let composed = preceding.transform_from_decomposed(&dt);
                    assert_point(point.transform_clone(&composed), sequential);
                    assert_point(
                        point.transform_clone(&composed.decompose().compose()),
                        sequential,
                    );
                }
            }
        }
    }

    #[test]
    fn permitted_axes_compile_to_canonical_rotations() {
        let fixed_y = AllowedOrientations::new(RotationRange::None, vec![FRAC_PI_2]).unwrap();
        assert!(fixed_y.allows(&DTransformation::empty()));
        assert!(!fixed_y.allows(&DTransformation::new(PI, (0.0, 0.0))));
        assert!(!fixed_y.allows(&DTransformation::empty().with_reflection(true)));
        let y_pose = DTransformation::new(PI, (0.0, 0.0)).with_reflection(true);
        assert!(fixed_y.allows(&y_pose));
        assert!(fixed_y.allows(&y_pose.compose().decompose()));
        assert_point(
            Point(2.0, 3.0).transform_clone(&y_pose.compose()),
            Point(-2.0, 3.0),
        );

        let both = AllowedOrientations::new(RotationRange::None, vec![0.0, FRAC_PI_2]).unwrap();
        assert!(both.allows(&DTransformation::empty().with_reflection(true)));
        assert!(both.allows(&y_pose));
        assert!(!both.allows(&DTransformation::new(PI, (0.0, 0.0))));
        let diagonal = AllowedOrientations::new(RotationRange::None, vec![FRAC_PI_4]).unwrap();
        let pose = DTransformation::new(FRAC_PI_2, (0.0, 0.0)).with_reflection(true);
        assert!(diagonal.allows(&pose));
        assert_point(
            Point(2.0, 3.0).transform_clone(&pose.compose()),
            Point(3.0, 2.0),
        );

        let wrapped = AllowedOrientations::new(
            RotationRange::Discrete(vec![0.0, TAU, -TAU]),
            vec![0.0, PI, -PI],
        )
        .unwrap();
        assert_eq!(wrapped.rotations(false), Some(&RotationRange::None));
        assert_eq!(wrapped.rotations(true), Some(&RotationRange::None));
        let continuous =
            AllowedOrientations::new(RotationRange::Continuous, vec![0.1, 0.2]).unwrap();
        assert_eq!(continuous.rotations(true), Some(&RotationRange::Continuous));
        assert!(continuous.allows(&DTransformation::new(0.73, (0.0, 0.0)).with_reflection(true)));
        let disabled = AllowedOrientations::new(RotationRange::Discrete(vec![]), vec![]).unwrap();
        assert!(disabled.allows(&DTransformation::empty()));
        assert!(!disabled.allows(&y_pose));

        // Input deduplication must not erase nearby, distinct rotations.
        let close =
            AllowedOrientations::new(RotationRange::Discrete(vec![0.0, 1e-7]), vec![]).unwrap();
        assert_eq!(
            close.rotations(false),
            Some(&RotationRange::Discrete(vec![0.0, 1e-7]))
        );
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(
                AllowedOrientations::new(RotationRange::Discrete(vec![invalid]), vec![]).is_err()
            );
            assert!(AllowedOrientations::new(RotationRange::None, vec![invalid]).is_err());
        }
        assert!(normalize_rotation(-f32::EPSILON) < TAU);
    }

    #[test]
    fn reflected_polygon_buffers_preserve_ccw_geometry_and_surrogates() {
        let original = SPolygon::new(vec![
            Point(2.0, 3.0),
            Point(8.0, 3.0),
            Point(8.0, 5.0),
            Point(5.0, 5.0),
            Point(5.0, 9.0),
            Point(2.0, 9.0),
        ])
        .unwrap();
        for with_surrogate in [false, true] {
            let mut reference = original.clone();
            if with_surrogate {
                reference
                    .generate_surrogate(SPSurrogateConfig {
                        n_pole_limits: [(4, 0.0); 3],
                        ff_pole_area_ratio: 0.5,
                        n_ff_piers: 2,
                    })
                    .unwrap();
            }
            for reflected_reference in [false, true] {
                let reference = reference.transform_clone(
                    &DTransformation::new(0.0, (1.0, 2.0))
                        .with_reflection(reflected_reference)
                        .compose(),
                );
                let mut buffer = reference.clone();
                let vertex_allocation = buffer.vertices.as_ptr();
                let hull_allocation = buffer
                    .surrogate
                    .as_ref()
                    .map(|s| s.convex_hull_indices.as_ptr());
                for reflected in [true, false, true, true, false] {
                    let t = DTransformation::new(0.37, (13.0, 7.0))
                        .with_reflection(reflected)
                        .compose();
                    buffer.transform_from(&reference, &t);
                    let in_place = reference.transform_clone(&t);
                    assert_eq!(buffer.vertices, in_place.vertices);
                    assert_eq!(buffer.vertices.as_ptr(), vertex_allocation);
                    assert!(
                        (SPolygon::calculate_area(&buffer.vertices) - original.area).abs() < 0.0001
                    );
                    assert_eq!(buffer.area, original.area);
                    assert_point(buffer.centroid(), reference.centroid().transform_clone(&t));
                    assert_eq!(
                        buffer.bbox,
                        SPolygon::generate_bounding_box(&buffer.vertices)
                    );
                    assert_point(buffer.poi.center, reference.poi.center.transform_clone(&t));
                    assert_eq!(buffer.poi.radius, reference.poi.radius);

                    if with_surrogate {
                        let surrogate = buffer.surrogate();
                        assert_eq!(
                            Some(surrogate.convex_hull_indices.as_ptr()),
                            hull_allocation
                        );
                        assert_eq!(
                            surrogate.convex_hull_indices,
                            in_place.surrogate().convex_hull_indices
                        );
                        let hull = convex_hull_from_surrogate(&buffer).unwrap();
                        let fresh_hull = convex_hull_from_points(buffer.vertices.clone());
                        assert!(SPolygon::calculate_area(&hull) > 0.0);
                        assert_eq!(hull.len(), fresh_hull.len());
                        assert!(hull.iter().all(|p| fresh_hull.contains(p)));
                        assert!(
                            (SPolygon::calculate_area(&hull) - surrogate.convex_hull_area).abs()
                                < 0.0001
                        );
                        for (pole, ref_pole) in
                            surrogate.poles.iter().zip(&reference.surrogate().poles)
                        {
                            assert_eq!(*pole, ref_pole.transform_clone(&t));
                        }
                        for (pier, ref_pier) in
                            surrogate.piers.iter().zip(&reference.surrogate().piers)
                        {
                            assert_eq!(*pier, ref_pier.transform_clone(&t));
                        }
                    }
                    for x in 0..11_u16 {
                        for y in 0..11_u16 {
                            let p = Point(f32::from(x) + 0.17, f32::from(y) + 0.31);
                            let transformed = p.transform_clone(&t);
                            assert_eq!(
                                buffer.collides_with(&transformed),
                                reference.collides_with(&p)
                            );
                            assert!(
                                (buffer.distance_to(&transformed) - reference.distance_to(&p))
                                    .abs()
                                    < 0.0001
                            );
                        }
                    }
                }
            }
        }
    }
}

mod io {
    use std::f32::consts::PI;

    use serde_json::json;

    use jagua_rs::collision_detection::CDEConfig;
    use jagua_rs::geometry::DTransformation;
    use jagua_rs::geometry::fail_fast::SPSurrogateConfig;
    use jagua_rs::geometry::geo_enums::RotationRange;
    use jagua_rs::io::ext_repr::{ExtItem, ExtTransformation};
    use jagua_rs::io::import::Importer;

    fn importer() -> Importer {
        Importer::new(
            CDEConfig {
                quadtree_depth: 3,
                cd_threshold: 16,
                item_surrogate_config: SPSurrogateConfig {
                    n_pole_limits: [(4, 0.0); 3],
                    ff_pole_area_ratio: 0.5,
                    n_ff_piers: 2,
                },
            },
            None,
            None,
        )
    }

    fn item_json() -> serde_json::Value {
        json!({
            "id": 0,
            "shape": {"type": "simple_polygon", "data": [[2,3],[8,3],[8,5],[5,5],[5,9],[2,9]]}
        })
    }

    #[test]
    fn orientation_modes_normalize_and_roundtrip() {
        let importer = importer();
        for (rotation, expected) in [
            (json!({"mode":"continuous"}), RotationRange::Continuous),
            (json!({"mode":"discrete","angles":[0]}), RotationRange::None),
            (json!({"mode":"stepped","step":360}), RotationRange::None),
            (
                json!({"mode":"discrete","angles":[450,-270,90]}),
                RotationRange::Discrete(vec![PI / 2.0]),
            ),
            (
                json!({"mode":"stepped","step":180}),
                RotationRange::Discrete(vec![0.0, PI]),
            ),
            (
                json!({"mode":"stepped","step":90}),
                RotationRange::Discrete(vec![0.0, PI / 2.0, PI, 3.0 * PI / 2.0]),
            ),
        ] {
            for axes in [json!([]), json!([90, -90, 270])] {
                let mut value = item_json();
                value["orientation"] = json!({"rotation":rotation,"reflection_axes":axes});
                let ext: ExtItem = serde_json::from_value(value).unwrap();
                let encoded = serde_json::to_value(&ext).unwrap();
                assert_eq!(encoded["orientation"]["rotation"]["mode"], rotation["mode"]);
                assert!(encoded.get("allowed_rotations").is_none());
                let restored: ExtItem = serde_json::from_value(encoded).unwrap();
                assert_eq!(ext.orientation, restored.orientation);
                for ext in [&ext, &restored] {
                    let item = importer.import_item(ext, 0).unwrap();
                    assert_eq!(item.allowed_orientations.rotations(false), Some(&expected));
                    for degrees in [0.0_f32, 45.0, 90.0, 180.0, 270.0] {
                        let allowed = match &expected {
                            RotationRange::Continuous => true,
                            RotationRange::None => degrees == 0.0,
                            RotationRange::Discrete(angles) => {
                                angles.contains(&degrees.to_radians())
                            }
                        };
                        assert_eq!(
                            item.allowed_orientations
                                .allows(&DTransformation::new(degrees.to_radians(), (0.0, 0.0))),
                            allowed
                        );
                        // Y-axis reflection compiles to canonical X-reflection plus 180 degrees.
                        assert_eq!(
                            item.allowed_orientations.allows(
                                &DTransformation::new((degrees + 180.0).to_radians(), (0.0, 0.0))
                                    .with_reflection(true)
                            ),
                            allowed && !axes.as_array().unwrap().is_empty()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn orientation_schema_rejects_missing_null_legacy_and_wrong_mode_fields() {
        fn check<T: serde::de::DeserializeOwned + serde::Serialize>(mut value: serde_json::Value) {
            assert!(serde_json::from_value::<T>(value.clone()).is_err());
            for orientation in [
                json!(null),
                json!({}),
                json!({"rotation":null}),
                json!({"rotation":[]}),
                json!({"rotation":{"mode":"unknown"}}),
                json!({"rotation":{"mode":"discrete"}}),
                json!({"rotation":{"mode":"stepped"}}),
                json!({"rotation":{"mode":"continuous","angles":[0]}}),
                json!({"rotation":{"mode":"continuous","step":90}}),
                json!({"rotation":{"mode":"discrete","angles":[0],"step":90}}),
                json!({"rotation":{"mode":"stepped","step":90,"angles":[0]}}),
                json!({"rotation":{"mode":"continuous"},"reflection_axes":null}),
                json!({"rotation":{"mode":"continuous"},"offset":30}),
            ] {
                value["orientation"] = orientation;
                assert!(
                    serde_json::from_value::<T>(value.clone()).is_err(),
                    "{value}"
                );
            }
            value["orientation"] = json!({"rotation":{"mode":"continuous"}});
            let parsed: T = serde_json::from_value(value.clone()).unwrap();
            let encoded = serde_json::to_value(parsed).unwrap();
            assert_eq!(encoded["orientation"], value["orientation"]);
            assert!(serde_json::from_value::<T>(encoded).is_ok());
            for key in [
                "allowed_rotations",
                "allowed_orientations",
                "allowed_reflection_axes",
            ] {
                for legacy in [json!([0]), json!(null)] {
                    let mut mixed = value.clone();
                    mixed[key] = legacy;
                    assert!(
                        serde_json::from_value::<T>(mixed.clone()).is_err(),
                        "{mixed}"
                    );
                    mixed.as_object_mut().unwrap().remove("orientation");
                    assert!(serde_json::from_value::<T>(mixed).is_err());
                }
            }
        }
        check::<ExtItem>(item_json());
        {
            let mut value = item_json();
            value["demand"] = json!(1);
            check::<jagua_rs::probs::spp::io::ext_repr::ExtItem>(value.clone());
            check::<jagua_rs::probs::bpp::io::ext_repr::ExtItem>(value.clone());
            check::<jagua_rs::probs::mspp::io::ext_repr::ExtItem>(value);
        }
    }

    #[test]
    fn orientation_import_validates_numbers_and_bounds_expansion() {
        use jagua_rs::geometry::AllowedOrientations;
        use jagua_rs::io::ext_repr::ExtRotation;
        let importer = importer();
        let mut value = item_json();
        value["orientation"] = json!({"rotation":{"mode":"continuous"}});
        let mut ext: ExtItem = serde_json::from_value(value).unwrap();
        for angles in [
            vec![],
            vec![f32::NAN],
            vec![f32::INFINITY],
            vec![0.0; AllowedOrientations::MAX_ANGLES + 1],
        ] {
            ext.orientation.rotation = ExtRotation::Discrete { angles };
            assert!(importer.import_item(&ext, 0).is_err());
        }
        for step in [
            0.0,
            -90.0,
            361.0,
            7.0,
            90.001,
            f32::from_bits(90.0_f32.to_bits() + 2),
            0.10001,
            f32::NAN,
            f32::INFINITY,
            f32::MIN_POSITIVE,
            360.0 / (AllowedOrientations::MAX_ANGLES + 1) as f32,
        ] {
            ext.orientation.rotation = ExtRotation::Stepped { step };
            assert!(importer.import_item(&ext, 0).is_err(), "step {step}");
        }
        for (step, count) in [
            (0.1, 3600),
            (2.5, 144),
            (f32::from_bits(90.0_f32.to_bits() + 1), 4),
            (
                360.0 / AllowedOrientations::MAX_ANGLES as f32,
                AllowedOrientations::MAX_ANGLES,
            ),
        ] {
            ext.orientation.rotation = ExtRotation::Stepped { step };
            let item = importer.import_item(&ext, 0).unwrap();
            let Some(RotationRange::Discrete(angles)) = item.allowed_orientations.rotations(false)
            else {
                panic!("expected discrete rotations")
            };
            assert_eq!(angles.len(), count);
            assert_eq!(angles[0], 0.0);
            assert!(angles.windows(2).all(|pair| pair[0] < pair[1]));
            assert!(angles[count - 1] < std::f32::consts::TAU);
            assert!(item.allowed_orientations.allows(&DTransformation::new(
                (360.0 - step).to_radians(),
                (0.0, 0.0)
            )));
        }
        ext.orientation.reflection_axes = vec![0.0, 0.001];
        assert!(importer.import_item(&ext, 0).is_err()); // Reject Cartesian expansion before allocation.
        ext.orientation.rotation = ExtRotation::Continuous {};
        for axes in [
            vec![f32::NAN],
            vec![f32::NEG_INFINITY],
            vec![0.0; AllowedOrientations::MAX_ANGLES + 1],
        ] {
            ext.orientation.reflection_axes = axes;
            assert!(importer.import_item(&ext, 0).is_err());
        }
    }

    #[test]
    fn transformation_json_preserves_reflection_and_old_output() {
        let old = json!({"rotation": 0.0, "translation": [1.0, 2.0]});
        let ext: ExtTransformation = serde_json::from_value(old.clone()).unwrap();
        assert!(!ext.reflected);
        assert_eq!(serde_json::to_value(ext).unwrap(), old);
        let pose = DTransformation::new(PI, (1.0, 2.0)).with_reflection(true);
        let value = serde_json::to_value(ExtTransformation::from(pose)).unwrap();
        assert_eq!(value["reflected"], true);
        assert_eq!(
            DTransformation::from(serde_json::from_value::<ExtTransformation>(value).unwrap()),
            pose
        );
    }

    #[test]
    fn reflected_layout_roundtrip_collision_restore_and_svg() {
        use jagua_rs::collision_detection::hazards::filter::NoFilter;
        use jagua_rs::geometry::geo_traits::{DistanceTo, Transformable};
        use jagua_rs::geometry::primitives::Point;
        use jagua_rs::io::export::int_to_ext_transformation;
        use jagua_rs::io::import::ext_to_int_transformation;
        use jagua_rs::io::svg::{SvgDrawOptions, layout_to_svg};
        use jagua_rs::probs::spp::entities::{SPPlacement, SPProblem};
        use jagua_rs::probs::spp::io::ext_repr::{ExtSPInstance, ExtSPSolution};
        use jagua_rs::probs::spp::io::{export, import_instance, import_solution};

        let mut item = item_json();
        item["orientation"] =
            json!({"rotation":{"mode":"discrete","angles":[0]},"reflection_axes":[90]});
        item["demand"] = json!(2);
        let ext: ExtSPInstance = serde_json::from_value(json!({
            "name": "reflection", "strip_height": 30, "items": [item]
        }))
        .unwrap();
        let mut conflicting = serde_json::to_value(&ext).unwrap();
        conflicting["items"][0]["allowed_orientations"] = json!([]);
        assert!(serde_json::from_value::<ExtSPInstance>(conflicting).is_err());
        let instance = import_instance(&importer(), &ext).unwrap();
        let mut problem = SPProblem::new(instance.clone()).unwrap();
        problem.change_strip_width(40.0).unwrap();
        let item = instance.item(0);
        // External Y reflection: (x,y) -> (20-x, 5+y), independently of centering.
        let external = DTransformation::new(PI, (20.0, 5.0)).with_reflection(true);
        let internal = ext_to_int_transformation(&external, &item.shape_orig.pre_transform);
        assert!(item.allowed_orientations.allows(&internal));
        assert!(!problem.layout.cde().detect_surrogate_collision(
            item.shape_cd.surrogate(),
            &internal.compose(),
            &NoFilter,
        ));
        let exported = int_to_ext_transformation(&internal, &item.shape_orig.pre_transform);
        for p in &item.shape_orig.shape.vertices {
            let expected = Point(20.0 - p.0, 5.0 + p.1);
            assert!(
                p.transform_clone(&exported.compose())
                    .distance_to(&expected)
                    < 0.0001
            );
        }
        let pk = problem.place_item(SPPlacement {
            item_idx: 0,
            d_transf: internal,
        });
        assert!(problem.layout.is_feasible());
        let saved = problem.save();
        assert!(problem.layout.cde().detect_surrogate_collision(
            item.shape_cd.surrogate(),
            &internal.compose(),
            &NoFilter,
        ));
        let overlap = problem.place_item(SPPlacement {
            item_idx: 0,
            d_transf: internal,
        });
        assert!(!problem.layout.is_feasible());
        problem.remove_item(overlap);
        problem.remove_item(pk);
        problem.restore(&saved);
        assert!(problem.layout.is_feasible());
        assert_eq!(problem.layout.placed_items[pk].d_transf, internal);

        let output = export(&saved, saved.time_stamp);
        let encoded = serde_json::to_string(&output).unwrap();
        let decoded: ExtSPSolution = serde_json::from_str(&encoded).unwrap();
        assert!(decoded.layout.placed_items[0].transformation.reflected);
        let restored = import_solution(&instance, &decoded).unwrap();
        let restored_item = restored
            .layout_snapshot
            .placed_items
            .values()
            .next()
            .unwrap();
        assert!(restored_item.d_transf.reflected);
        let original_item = &problem.layout.placed_items[pk];
        for (a, b) in restored_item
            .shape
            .vertices
            .iter()
            .zip(&original_item.shape.vertices)
        {
            assert!(a.distance_to(b) < 0.0001);
        }
        let svg =
            layout_to_svg(&problem.layout, SvgDrawOptions::default(), "reflection").to_string();
        assert!(svg.contains(", scale(1 -1)"));
        assert!(svg.contains("rotate("));
    }
}

mod sampling {
    use std::f32::consts::PI;

    use jagua_rs::geometry::DTransformation;
    use jagua_rs::geometry::primitives::Rect;
    use jagua_rs::io::ext_repr::ExtItem;
    use jagua_rs::io::import::Importer;
    use rand::{SeedableRng, rngs::SmallRng};
    use serde_json::json;

    use lbf::config::LBFConfig;
    use lbf::samplers::ls_sampler::LSSampler;
    use lbf::samplers::uniform_rect_sampler::UniformRectSampler;

    #[test]
    fn samplers_explore_and_preserve_permitted_reflections() {
        let importer = Importer::new(LBFConfig::default().cde_config, None, None);
        let bbox = Rect::try_new(0.0, 0.0, 20.0, 20.0).unwrap();
        let mut rng = SmallRng::seed_from_u64(42);
        for rotation in [
            json!({"mode":"discrete","angles":[0]}),
            json!({"mode":"discrete","angles":[30,90]}),
            json!({"mode":"stepped","step":90}),
            json!({"mode":"continuous"}),
        ] {
            let ext: ExtItem = serde_json::from_value(json!({
                "id": 0, "orientation": {"rotation": rotation, "reflection_axes": [90]},
                "shape": {"type": "simple_polygon", "data": [[0,0],[5,0],[1,3]]}
            }))
            .unwrap();
            let item = importer.import_item(&ext, 0).unwrap();
            let sampler = UniformRectSampler::new(bbox, &item);
            let mut seen = [false; 2];
            for _ in 0..128 {
                let pose = sampler.sample(&mut rng);
                seen[usize::from(pose.reflected)] = true;
                assert!(item.allowed_orientations.allows(&pose));
                let mut local = LSSampler::from_defaults(&item, pose, bbox);
                for _ in 0..8 {
                    let nearby = local.sample(&mut rng);
                    assert_eq!(nearby.reflected, pose.reflected);
                    assert!(item.allowed_orientations.allows(&nearby));
                }
                let next = sampler.sample(&mut rng);
                local.shift_mean(next);
                let shifted = local.sample(&mut rng);
                assert_eq!(shifted.reflected, next.reflected);
                assert!(item.allowed_orientations.allows(&shifted));
            }
            assert_eq!(seen, [true, true]);
            if rotation == json!({"mode":"discrete","angles":[0]}) {
                let pose = DTransformation::new(PI, (10.0, 10.0)).with_reflection(true);
                let mut local = LSSampler::from_defaults(&item, pose, bbox);
                assert_eq!(local.sample(&mut rng).rotation(), PI);
            }
        }
    }
}
