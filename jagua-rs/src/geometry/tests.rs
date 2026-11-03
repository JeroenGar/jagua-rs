//! Tests of the geometry module

mod original_shape {
    use crate::geometry::OriginalShape;
    use crate::geometry::geo_traits::Transformable;
    use crate::geometry::primitives::SPolygon;
    use crate::geometry::shape_modification::{
        ShapeModifyConfig, ShapeModifyMode, shape_modification_valid,
    };
    use crate::io::ext_repr::ExtSPolygon;
    use crate::io::import::{centering_transformation, import_simple_polygon};

    #[test]
    fn short_edge_does_not_collapse_when_centering() {
        // Polygon from issue #133. The vertices at index 8 and 9 are only 1e-6 apart: distinct in f32,
        // but identical once the shape is translated to its centroid, which made simplify_shape panic.
        let points = vec![
            (10.5, 6.204_517),
            (3.930_766e-16, 6.419_428),
            (-33.442_33, 5.734_94),
            (-47.662_89, 4.766_93),
            (-59.808_86, 3.290_652),
            (-69.298_87, 1.320_259),
            (-75.391_26, -0.889_303),
            (-77.600_12, -2.461_642),
            (-77.686_46, -2.777_069),
            (-77.686_46, -2.777_068),
            (-80.021_02, -11.306_129),
            (-88.827_56, -10.450_826),
            (-203.632_35, 0.699_174),
            (-220.234_46, 2.311_593),
            (-211.601_77, 16.584_175),
            (-194.721_86, 44.492_01),
            (-187.869_04, 57.095_108),
            (-183.597_82, 67.951_93),
            (-182.840_06, 71.706_25),
            (-182.800_17, 74.573_4),
            (-183.2407, 76.380_84),
            (-184.061_2, 77.732_344),
            (-185.847_34, 79.293_46),
            (-189.443_63, 81.091_91),
            (-195.307_63, 82.745_61),
            (-204.078_86, 83.978_09),
            (-226.052_1, 87.065_65),
            (-226.783_56, 90.060_72),
            (-208.706_9, 102.929_69),
            (-182.091_23, 121.877_63),
            (-180.688_03, 127.232_44),
            (-179.573_62, 135.095_55),
            (-179.523_21, 142.374_56),
            (-180.492_94, 149.230_05),
            (-182.492_9, 155.858_63),
            (-185.596_57, 162.443),
            (-189.911_15, 169.110_44),
            (-195.857_58, 176.317_67),
            (-199.083_3, 180.227_33),
            (-196.657_06, 191.631_46),
            (-190.255_38, 193.198_78),
            (-26.876_544, 233.198_78),
            (-18.457_486, 235.260_02),
            (-13.992_686, 225.543_02),
            (-13.892_675, 223.5243),
            (-13.421_246, 214.008_47),
            (-12.161_206, 205.7295),
            (-10.219_268, 198.415_95),
            (-7.773_85, 192.343_95),
            (-5.091_749, 187.800_5),
            (-2.598_881, 185.016_36),
            (-0.863_646, 183.905_51),
            (1.124_994_5e-14, 183.725_54),
            (10.5, 185.913_53),
        ];
        let shape = import_simple_polygon(&ExtSPolygon(points)).unwrap();
        let original = OriginalShape {
            pre_transform: centering_transformation(&shape),
            shape,
            modify_mode: ShapeModifyMode::Inflate,
            modify_config: ShapeModifyConfig {
                simplify_tolerance: Some(0.001),
                offset: None,
                narrow_concavity_cutoff: Some((0.01, 0.01)),
            },
        };

        let internal = original.convert_to_internal().unwrap();

        let min_edge_length = SPolygon::MIN_EDGE_LENGTH_RATIO * internal.diameter;
        assert!(internal.edge_iter().all(|e| e.length() >= min_edge_length));

        let centered = original
            .shape
            .transform_clone(&original.pre_transform.compose());
        assert!(shape_modification_valid(
            &centered,
            &internal,
            ShapeModifyMode::Inflate
        ));
    }
}

mod shape_modification {
    use crate::geometry::primitives::{Point, SPolygon};
    use crate::geometry::shape_modification::{
        ShapeModifyMode, close_narrow_concavities, offset_shape, shape_modification_valid,
        simplify_shape,
    };

    #[test]
    fn closing_concavity_measures_the_region_it_removes() {
        // Square with a notch of area 18 (2 wide, 9 deep) at the top, in a shape of area 82.
        let points = [
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 10.0),
            (6.0, 10.0),
            (6.0, 1.0),
            (4.0, 1.0),
            (4.0, 10.0),
            (0.0, 10.0),
        ];
        let shape = SPolygon::new(points.map(|(x, y)| Point(x, y)).to_vec()).unwrap();

        // 18 exceeds 15% of the area, the notch stays open
        let kept = close_narrow_concavities(&shape, ShapeModifyMode::Inflate, (0.2, 0.15));
        assert_eq!(kept.n_vertices(), shape.n_vertices());

        // 18 is below 25% of the area, the notch is closed
        let closed = close_narrow_concavities(&shape, ShapeModifyMode::Inflate, (0.2, 0.25));
        assert_eq!(closed.n_vertices(), shape.n_vertices() - 2);
        assert!(shape_modification_valid(
            &shape,
            &closed,
            ShapeModifyMode::Inflate
        ));
    }

    #[test]
    fn closing_concavity_does_not_keep_the_concavity_instead_of_the_shape() {
        // Leaving out the end vertex of the removed region gave it the wrong sign, which closed the edge
        // from (0,0) to (-10,0) on the wrong side and replaced the shape (area 7) by its notch (area 2.5).
        let points = [
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 2.0),
            (-10.0, 0.0),
            (-5.0, 0.5),
        ];
        let shape = SPolygon::new(points.map(|(x, y)| Point(x, y)).to_vec()).unwrap();

        let closed = close_narrow_concavities(&shape, ShapeModifyMode::Inflate, (1.0, 0.1));

        assert!(shape_modification_valid(
            &shape,
            &closed,
            ShapeModifyMode::Inflate
        ));
    }

    #[test]
    fn simplification_does_not_grow_bounding_box() {
        // Rectangle with a V-notch in its bottom edge, see sparrow#176.
        // Replacing the corners at (0,0) and (15,0) by the intersection of their adjacent edges
        // would add a vertex at (0,-30): a small area increase, but 30 units of extra height.
        let points = [
            (0.0, 0.0),
            (15.0, 0.0),
            (30.0, 30.0),
            (45.0, 0.0),
            (557.3, 0.0),
            (557.3, 970.0),
            (0.0, 970.0),
        ];
        let shape = SPolygon::new(points.map(|(x, y)| Point(x, y)).to_vec()).unwrap();

        let simplified = simplify_shape(&shape, ShapeModifyMode::Inflate, 0.001);

        assert!(simplified.n_vertices() < shape.n_vertices());
        assert!(shape_modification_valid(
            &shape,
            &simplified,
            ShapeModifyMode::Inflate
        ));
        assert_eq!(simplified.bbox, shape.bbox);
    }

    #[test]
    fn simplification_keeps_short_edges_when_diameter_grows() {
        // Replacing a chamfer on the right by a square corner grows the diameter,
        // and with it the minimum edge length, past the length of the short edge on the left.
        // SPolygon::new would then remove (0,5) from the simplified shape, cutting into the original,
        // so the replacement must be rejected.
        let points = [
            (0.0, 5.000_115),
            (0.0, 5.0),
            (4.0, 2.0),
            (4.0, 0.0),
            (9.0, 0.0),
            (10.0, 1.0),
            (10.0, 9.0),
            (9.0, 10.0),
            (4.0, 10.0),
            (4.0, 8.0),
        ];
        let shape = SPolygon::new(points.map(|(x, y)| Point(x, y)).to_vec()).unwrap();

        let simplified = simplify_shape(&shape, ShapeModifyMode::Inflate, 0.01);

        assert!(shape_modification_valid(
            &shape,
            &simplified,
            ShapeModifyMode::Inflate
        ));
    }

    #[test]
    fn offset_survives_f32_rounding_of_closed_slit() {
        // Rectangle with a 2.0 wide slit at 20 degrees, see issue #130.
        // Inflating by exactly half the slit width closes it into a zero-width spike,
        // whose vertices cross each other when the f64 buffer is rounded to f32.
        let points = [
            (0.0, 0.0),
            (400.0, 0.0),
            (400.0, 300.0),
            (202.128_36, 300.0),
            (222.400_59, 244.302_48),
            (220.521_21, 243.618_44),
            (200.0, 300.0),
            (0.0, 300.0),
        ];
        let shape = SPolygon::new(points.map(|(x, y)| Point(x, y)).to_vec()).unwrap();

        let offset = offset_shape(&shape, ShapeModifyMode::Inflate, 1.0).unwrap();

        assert!(shape_modification_valid(
            &shape,
            &offset,
            ShapeModifyMode::Inflate
        ));
    }
}

mod simple_polygon {
    use crate::geometry::fail_fast::SPSurrogateConfig;
    use crate::geometry::primitives::{Point, Rect, SPolygon};

    fn points(coords: &[(f32, f32)]) -> Vec<Point> {
        coords.iter().map(|&(x, y)| Point(x, y)).collect()
    }

    fn assert_min_edge_length(sp: &SPolygon) {
        let min_edge_length = SPolygon::MIN_EDGE_LENGTH_RATIO * sp.diameter;
        assert!(sp.edge_iter().all(|e| e.length() >= min_edge_length));
    }

    #[test]
    fn new_removes_short_edges() {
        let square = [(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];

        // exact consecutive duplicate
        let sp = SPolygon::new(points(&[
            square[0], square[1], square[1], square[2], square[3],
        ]));
        assert_eq!(sp.unwrap().n_vertices(), 4);

        // vertex closer than the minimum edge length to its predecessor
        let sp = SPolygon::new(points(&[
            square[0],
            square[1],
            (10.0, 1e-5),
            square[2],
            square[3],
        ]));
        assert_eq!(sp.unwrap().n_vertices(), 4);

        // last vertex too close to the first one (closing edge)
        let sp = SPolygon::new(points(&[
            square[0],
            square[1],
            square[2],
            square[3],
            (0.0, 1e-5),
        ]));
        let sp = sp.unwrap();
        assert_eq!(sp.n_vertices(), 4);
        assert_min_edge_length(&sp);

        // a vertex at a safe distance is kept
        let sp = SPolygon::new(points(&[
            square[0],
            square[1],
            (10.0, 1e-3),
            square[2],
            square[3],
        ]));
        assert_eq!(sp.unwrap().n_vertices(), 5);
    }

    #[test]
    fn rectangle_conversion() {
        let rect = Rect::try_new(0.0, 0.0, 10.0, 5.0).unwrap();
        assert_eq!(SPolygon::try_from(rect).unwrap().n_vertices(), 4);

        // the short sides are below the minimum edge length, so they are removed
        let sliver = Rect::try_new(0.0, 0.0, 10.0, 1e-6).unwrap();
        assert!(SPolygon::try_from(&sliver).is_err());
    }

    #[test]
    fn new_fails_when_too_few_vertices_remain() {
        let sliver = points(&[(0.0, 0.0), (10.0, 0.0), (10.0, 1e-6), (0.0, 1e-6)]);
        assert!(SPolygon::new(sliver).is_err());
    }

    #[test]
    fn centroid_far_from_origin() {
        // The cross products of absolute coordinates cancel out, which put this centroid at (1365.3, 1365.3)
        let sp = SPolygon::new(points(&[
            (4096.0, 4096.0),
            (4097.0, 4096.0),
            (4097.0, 4097.0),
            (4096.0, 4097.0),
        ]))
        .unwrap();
        assert_eq!(sp.centroid(), Point(4096.5, 4096.5));
    }

    #[test]
    fn diameter_with_vertices_of_equal_x() {
        // The convex hull sorted the vertices by x only and lost (0,0), giving a diameter of sqrt(181)
        let sp = SPolygon::new(points(&[
            (0.0, 1.0),
            (0.0, 0.0),
            (1.0, 0.0),
            (10.0, 10.0),
            (0.0, 2.0),
        ]))
        .unwrap();
        assert!((sp.diameter - 200.0_f32.sqrt()).abs() < 1e-4);
    }

    #[test]
    fn pole_far_from_origin() {
        // At this x coordinate, the midpoint of a quadrant rounds onto its side, which panicked in Rect::quadrants.
        // The f32 spacing is 2 here, so no point inside the square can be represented: an error, not a panic.
        let x = 16_777_216.0;
        let sp = SPolygon::new(points(&[
            (x, 0.0),
            (x + 2.0, 0.0),
            (x + 2.0, 2.0),
            (x, 2.0),
        ]));
        assert!(sp.is_err());
    }

    #[test]
    fn pole_of_thin_shape() {
        // The arms are thinner than the nodes of the default pole search tree, so no pole was found
        let sp = SPolygon::new(points(&[
            (0.0, 0.0),
            (100.0, 0.0),
            (100.0, 0.01),
            (0.01, 0.01),
            (0.01, 100.0),
            (0.0, 100.0),
        ]));
        assert!(sp.is_ok());
    }

    #[test]
    fn surrogate_of_thin_shape() {
        // The pole search runs out of room before the pole limits are reached, which used to fail
        let mut sp = SPolygon::new(points(&[
            (0.0, 0.0),
            (3000.0, 0.0),
            (3000.0, 1.0),
            (0.0, 1.0),
        ]))
        .unwrap();
        let config = SPSurrogateConfig {
            n_pole_limits: [(100, 0.0), (20, 0.75), (10, 0.90)],
            ff_pole_area_ratio: 0.5,
        };
        sp.generate_surrogate(config).unwrap();
        assert!(!sp.surrogate().poles.is_empty());
    }
}

mod short_edges {
    use crate::geometry::geo_traits::DistanceTo;
    use crate::geometry::primitives::{Point, SPolygon};
    use crate::geometry::short_edges::remove_short_edges;
    use itertools::Itertools;

    fn points(coords: &[(f32, f32)]) -> Vec<Point> {
        coords.iter().map(|&(x, y)| Point(x, y)).collect()
    }

    #[test]
    fn removed_vertices_are_reported() {
        let mut pts = points(&[
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 1e-5),
            (10.0, 10.0),
            (0.0, 10.0),
            (0.0, 1e-5),
        ]);
        let removal = remove_short_edges(&mut pts);

        let reported = removal
            .removed
            .iter()
            .map(|r| (r.idx, r.kept_idx))
            .collect_vec();
        // vertex 2 is too close to vertex 1, vertex 5 too close to vertex 0 across the closing edge
        assert_eq!(reported, [(2, 1), (5, 0)]);
        assert_eq!(pts.len(), 4);
    }

    #[test]
    fn dense_run_of_short_edges_is_thinned_not_removed() {
        // A 10x10 square whose bottom edge starts with a run sampled every 1e-5,
        // far below the minimum edge length (~1.4e-4). Comparing against the previous kept vertex
        // should keep a vertex roughly every 1.4e-4 instead of removing the whole run.
        let mut pts = (0..=10_000u16)
            .map(|i| Point(f32::from(i) * 1e-5, 0.0))
            .collect_vec();
        pts.extend(points(&[(10.0, 0.0), (10.0, 10.0), (0.0, 10.0)]));

        let n_removed = remove_short_edges(&mut pts).removed.len();

        let min_edge_length =
            SPolygon::MIN_EDGE_LENGTH_RATIO * SPolygon::calculate_diameter(pts.clone());
        assert!(n_removed > 9_000);
        assert!(
            pts.len() > 500,
            "the sampled edge should be thinned, not removed"
        );
        assert!(
            (0..pts.len())
                .all(|i| pts[i].distance_to(&pts[(i + 1) % pts.len()]) >= min_edge_length)
        );
    }
}
