use jagua_rs::Instant;
use jagua_rs::entities::{ContainerMismatch, Layout};
use jagua_rs::geometry::DTransformation;
use jagua_rs::io::ext_repr::{ExtContainer, ExtItem};
use jagua_rs::io::import::Importer;
use jagua_rs::probs::bpp::entities::{BPLayoutType, BPPlacement, BPProblem};
use jagua_rs::probs::bpp::io::ext_repr::ExtBPInstance;
use jagua_rs::probs::bpp::io::{export, import_instance};
use lbf::config::LBFConfig;
use serde_json::json;
use std::sync::Arc;

#[test]
fn restore_uses_static_geometry_and_bin_stock_uses_bin_identity() -> anyhow::Result<()> {
    let importer = Importer::new(LBFConfig::default().cde_config, None, None, None);
    let rectangle = |width| {
        json!({"type": "rectangle", "data": {
            "x_min": 0, "y_min": 0, "width": width, "height": 10
        }})
    };
    let item_json = json!({
        "id": 42, "demand": 2,
        "allowed_orientations": [0],
        "shape": {"type": "rectangle", "data": {
            "x_min": 0, "y_min": 0, "width": 2, "height": 2
        }}
    });
    let item: ExtItem = serde_json::from_value(item_json.clone())?;
    let item = importer.import_item(&item, 0)?;
    let container: ExtContainer = serde_json::from_value(json!({"id": 0, "shape": rectangle(10)}))?;
    let mut layout = Layout::new(importer.import_container(&container)?);
    layout.place_item(&item, DTransformation::new(0.0, (5.0, 5.0)));
    let saved = layout.save();
    assert!(layout.is_feasible());
    for changed in [
        json!({"id": 0, "shape": rectangle(10)}),
        json!({"id": 0, "shape": rectangle(3)}),
        json!({"id": 0, "shape": rectangle(10), "zones": [{
            "quality": 0, "shape": {"type": "rectangle", "data": {
                "x_min": 4, "y_min": 4, "width": 2, "height": 2
            }}
        }]}),
    ] {
        layout.swap_container(importer.import_container(&serde_json::from_value(changed)?)?);
        let before = layout.save();
        let feasible_before = layout.is_feasible();
        assert_eq!(layout.restore(&saved), Err(ContainerMismatch));
        assert!(Arc::ptr_eq(
            &layout.container.base_cde,
            &before.container.base_cde
        ));
        assert_eq!(layout.is_feasible(), feasible_before);
        assert!(jagua_rs::util::assertions::snapshot_matches_layout(
            &layout, &before
        ));
        assert!(jagua_rs::util::assertions::layout_qt_matches_fresh_qt(
            &layout
        ));

        layout.swap_container(saved.container.clone());
        layout.restore(&saved)?;
        assert!(layout.is_feasible());
        assert!(Arc::ptr_eq(
            &layout.container.base_cde,
            &saved.container.base_cde
        ));
        layout.restore(&saved)?;
        assert!(layout.is_feasible());
    }

    let input: ExtBPInstance = serde_json::from_value(json!({
        "name": "bin identities", "items": [item_json],
        "bins": [
            {"id": 900, "stock": 2, "cost": 11, "shape": rectangle(10)},
            {"id": 70, "stock": 2, "cost": 7, "shape": rectangle(10)}
        ]
    }))?;
    let instance = import_instance(&importer, &input)?;
    let epoch = Instant::now();
    let mut problem = BPProblem::new(instance.clone());
    let mut placements = vec![];
    for bin_id in 0..2 {
        placements.push(problem.place_item(BPPlacement {
            layout_id: BPLayoutType::Closed { bin_id },
            item_idx: 0,
            d_transf: DTransformation::new(0.0, (5.0, 5.0)),
        }));
    }
    let saved = problem.save();
    for (key, item) in placements {
        problem.remove_item(key, item);
    }
    problem.restore(&saved);
    assert_eq!(problem.bin_stock_qtys, vec![1, 1]);
    assert_eq!(problem.bin_cost(), 18);
    let exported = export(&instance, &problem.save(), epoch);
    let mut ids: Vec<_> = exported.layouts.iter().map(|l| l.container_id).collect();
    ids.sort_unstable();
    assert_eq!(ids, vec![70, 900]);
    Ok(())
}
