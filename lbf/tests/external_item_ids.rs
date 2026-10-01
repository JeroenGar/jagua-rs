use jagua_rs::Instant;
use jagua_rs::io::import::Importer;
use jagua_rs::io::svg::{SvgDrawOptions, s_layout_to_svg};
use jagua_rs::probs::spp::io::ext_repr::{ExtSPInstance, ExtSPSolution};
use jagua_rs::probs::spp::io::{export, import_instance, import_solution};
use serde_json::json;

#[test]
fn sparse_item_ids_survive_json_svg_and_warm_start() -> anyhow::Result<()> {
    let item = |id, demand| {
        json!({
            "id": id, "demand": demand, "allowed_orientations": [0],
            "shape": {"type": "rectangle", "data": {
                "x_min": 0, "y_min": 0, "width": 2, "height": 2
            }}
        })
    };
    let mut input: ExtSPInstance = serde_json::from_value(json!({
        "name": "sparse IDs", "strip_height": 10,
        "items": [item(u64::MAX, 1), item(4, 0), item(42, 1)]
    }))?;
    let importer = Importer::new(lbf::config::LBFConfig::default().cde_config, None, None);
    let instance = import_instance(&importer, &input)?;
    assert_eq!(instance.item(0).idx, 0);
    assert_eq!(instance.item(0).external_id, u64::MAX);
    assert_eq!(instance.item(1).external_id, 42);
    assert_eq!(instance.item_idx(4), None);

    let placement = |id, x| {
        json!({
            "item_id": id,
            "transformation": {"rotation": 0, "translation": [x, 1]}
        })
    };
    let mut external: ExtSPSolution = serde_json::from_value(json!({
        "strip_width": 10, "density": 0.08, "run_time_sec": 0,
        "layout": {"container_id": 0, "density": 0.08,
            "placed_items": [placement(42, 1), placement(u64::MAX, 5)]}
    }))?;
    let epoch = Instant::now();
    let solution = import_solution(&instance, &external)?;
    let output = export(&solution, epoch);
    assert_eq!(
        serde_json::to_value(&output.layout.placed_items)?,
        serde_json::to_value(&external.layout.placed_items)?
    );
    let svg = s_layout_to_svg(&solution.layout_snapshot, SvgDrawOptions::default(), "").to_string();
    assert!(svg.contains("item_42"));
    assert!(svg.contains(&format!("item_{}", u64::MAX)));

    external.layout.placed_items[0].item_id = 4;
    assert!(import_solution(&instance, &external).is_err());
    input.items.push(input.items[1].clone());
    assert!(import_instance(&importer, &input).is_err());
    for pi in solution.layout_snapshot.placed_items.values() {
        assert!(std::sync::Arc::ptr_eq(&pi.item, instance.item(pi.item.idx)));
    }
    drop(instance);
    let restored = jagua_rs::entities::Layout::from_snapshot(&solution.layout_snapshot);
    assert_eq!(restored.items().count(), 2);
    assert_eq!(restored.density(), output.density);
    assert!(
        s_layout_to_svg(&restored.save(), SvgDrawOptions::default(), "")
            .to_string()
            .contains("item_42")
    );
    Ok(())
}
