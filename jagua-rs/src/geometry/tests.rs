use super::geo_traits::{DistanceTo, Transformable};
use super::primitives::{Point, Rect, SPolygon};
use super::shape_modification::{ShapeModifyMode, offset_shape, shape_modification_valid};
use crate::io::ext_repr::ExtSPolygon;
use crate::io::import::{centering_transformation, import_simple_polygon};

#[test]
fn clearance_buffer_handles_gardeyn2_item9() {
    // Studio's 2 mm clearance failed on this part with geo-buffer 0.2.0.
    let input: ExtSPolygon =
        serde_json::from_str(include_str!("fixtures/gardeyn2-item9.json")).unwrap();
    let original = import_simple_polygon(&input).unwrap();
    let centered = original.transform_clone(&centering_transformation(&original).compose());
    let buffered = offset_shape(&centered, ShapeModifyMode::Inflate, 1.0).unwrap();
    assert!(shape_modification_valid(
        &centered,
        &buffered,
        ShapeModifyMode::Inflate
    ));
}

#[test]
fn rounded_buffer_preserves_clearance_and_inward_offsets() {
    let original = SPolygon::from(Rect::try_new(0.0, 0.0, 10.0, 10.0).unwrap());
    let inflated = offset_shape(&original, ShapeModifyMode::Inflate, 1.0).unwrap();
    for (i, a) in inflated.vertices.iter().enumerate() {
        let b = inflated.vertices[(i + 1) % inflated.vertices.len()];
        let midpoint = Point(a.0.midpoint(b.0), a.1.midpoint(b.1));
        // A 0.1-radian chord falls short of its 1 mm arc by at most 0.00125 mm.
        for point in [*a, midpoint] {
            assert!((original.distance_to(&point) - 1.0).abs() < 0.0013);
        }
    }
    let deflated = offset_shape(&original, ShapeModifyMode::Deflate, 1.0).unwrap();
    assert!((deflated.area - 64.0).abs() < 1e-4);
    assert!(shape_modification_valid(
        &original,
        &deflated,
        ShapeModifyMode::Deflate
    ));
    assert!(offset_shape(&original, ShapeModifyMode::Deflate, 6.0).is_err());
}

#[test]
fn intersecting_polygon_error_reports_edges_without_dumping_vertices() {
    let error = SPolygon::new(vec![
        Point(0.0, 0.0),
        Point(2.0, 2.0),
        Point(0.0, 2.0),
        Point(2.0, 0.0),
    ])
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Simple polygon contains intersecting edges 0 and 2"
    );
}
