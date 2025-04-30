use jagua_rs_base::collision_detection::CDEConfig;
use jagua_rs_base::entities::{Container, OriginalShape};
use jagua_rs_base::geometry::DTransformation;
use jagua_rs_base::geometry::primitives::{Rect, SPolygon};
use jagua_rs_base::geometry::shape_modification::{ShapeModifyConfig, ShapeModifyMode};




/// Create a new `Bin` for a strip-packing problem. Instead of a shape, the bin is always rectangular.
pub fn from_strip(
    id: usize,
    rect: Rect,
    cde_config: CDEConfig,
    shape_modify_config: ShapeModifyConfig,
) -> Self {
    assert_eq!(rect.x_min, 0.0, "Strip x_min must be 0.0");
    assert_eq!(rect.y_min, 0.0, "Strip y_min must be 0.0");

    let original = OriginalShape {
        shape: SPolygon::from(rect),
        pre_transform: DTransformation::empty(),
        modify_mode: ShapeModifyMode::Deflate,
        modify_config: shape_modify_config,
    };

    Container::new(id, original, vec![], cde_config)
}