use crate::entities::LayoutSnapshot;
use crate::geometry::{DTransformation, Transformation};
use crate::io::ext_repr::{ExtLayout, ExtPlacedItem};

/// Exports a layout to an external representation.
/// The caller supplies the external bin/sheet ID; geometry carries no identity.
#[must_use]
pub fn export_layout_snapshot(layout: &LayoutSnapshot, container_id: u64) -> ExtLayout {
    let ext_placed_items = layout
        .placed_items
        .values()
        .map(|pi| {
            let item = &pi.item;

            let abs_transf =
                int_to_ext_transformation(&pi.d_transf, &item.shape_orig.pre_transform);

            ExtPlacedItem {
                item_id: item.external_id,
                transformation: abs_transf.into(),
            }
        })
        .collect();

    ExtLayout {
        container_id,
        placed_items: ext_placed_items,
        density: layout.density(),
    }
}

/// Converts an internal (used within `jagua-rs`) transformation to an external transformation (applicable to the original shapes).
///
/// * `int_transf` - The internal transformation.
/// * `pre_transf` - The transformation that was applied to the original shape to derive the internal representation.
#[must_use]
pub fn int_to_ext_transformation(
    int_transf: &DTransformation,
    pre_transf: &DTransformation,
) -> DTransformation {
    //1. apply the pre-transform
    //2. apply the internal transformation

    Transformation::empty()
        .transform_from_decomposed(pre_transf)
        .transform_from_decomposed(int_transf)
        .decompose()
}
