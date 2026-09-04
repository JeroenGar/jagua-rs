use super::qt_traits::QTQueryable;
use super::{QTHazPresence, QTHazard, QTNode};
use crate::collision_detection::hazards::{Hazard, HazardEntity, filter::NoFilter};
use crate::geometry::Transformation;
use crate::geometry::geo_traits::Transformable;
use crate::geometry::primitives::{Edge, Point, Rect, SPolygon};
use slotmap::SlotMap;
use std::sync::Arc;

#[test]
fn constriction_without_boundary_edges_resolves_presence() {
    let root = Rect::try_new(0.0, 0.0, 10.0, 10.0).unwrap();
    for (shape_bbox, expected_entire) in [
        (Rect::try_new(11.0, 0.0, 21.0, 10.0).unwrap(), false),
        (Rect::try_new(-1.0, -1.0, 11.0, 11.0).unwrap(), true),
    ] {
        let shape = SPolygon::new(shape_bbox.corners().to_vec()).unwrap();
        let mut hazards = SlotMap::with_key();
        let key = hazards.insert(Hazard::new(
            HazardEntity::Hole { idx: 0 },
            Arc::new(shape),
            true,
        ));
        let hazard = QTHazard::from_root(root, &hazards[key], key);
        let children = hazard.constrict(root.quadrants(), &hazards);
        for child in children {
            assert!(match child.presence {
                QTHazPresence::Entire => expected_entire,
                QTHazPresence::None => !expected_entire,
                QTHazPresence::Partial(_) => false,
            });
        }
    }
}

#[test]
#[ignore = "known query false negative; see issue-86-investigation.md"]
fn rotated_square_collision_survives_quadtree_subdivision() {
    let root = Rect::try_new(-100.0, 6.1875, 100.0, 206.1875).unwrap();
    let hazard_shape = SPolygon::new(
        Rect::try_new(0.0, 46.1875, 10.0, 56.1875)
            .unwrap()
            .corners()
            .to_vec(),
    )
    .unwrap();
    let start = Point(-1.996_518_5, 54.710_38);
    let end = Point(6.042_488_6, 60.658_016);
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let query = SPolygon::new(vec![
        Point(0.0, 0.0),
        Point(10.0, 0.0),
        Point(10.0, 10.0),
        Point(0.0, 10.0),
    ])
    .unwrap()
    .transform_clone(&Transformation::from_rotation(dy.atan2(dx)).translate((start.0, start.1)));
    assert_eq!(query.edge(0), Edge { start, end });
    let mut hazards = SlotMap::with_key();
    let key = hazards.insert(Hazard::new(
        HazardEntity::Hole { idx: 0 },
        Arc::new(hazard_shape),
        true,
    ));

    for depth in [0, 3] {
        let mut tree = QTNode::new(depth, root, 16);
        tree.register_hazard(QTHazard::from_root(root, &hazards[key], key), &hazards);
        assert!(
            query
                .edge_iter()
                .any(|edge| tree.collides(&edge, &NoFilter).is_some()),
            "missed square collision at depth {depth}"
        );
    }
}

#[test]
fn edge_quadrants_allow_f32_false_positive_at_bisector() {
    let rect = Rect::try_new(0.0, 6.1875, 100.0, 106.1875).unwrap();
    let quadrants = rect.quadrants();
    let edge = Edge {
        start: Point(-29.439_144, -32.354_836),
        end: Point(51.902_332, 212.291_02),
    };
    // The old debug assertion panicked on a conservative f32 false positive here.
    assert!(edge.collides_with_quadrants(&rect, quadrants.each_ref())[1]);
}
