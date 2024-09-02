use crate::collision_detection::hazard_filter::QZHazardFilter;
use crate::entities::item::Item;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::geo_traits::Transformable;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use slotmap::new_key_type;
use std::sync::Arc;

new_key_type! {
    /// Unique key for each `PlacedItem` in a layout.
    pub struct PItemKey;
}

/// Represents an `Item` that has been placed in a `Layout`
#[derive(Clone, Debug)]
pub struct PlacedItem {
    /// ID of the type of `Item` that was placed
    pub item_id: usize,
    /// The transformation that was applied to the `Item` before it was placed
    pub d_transf: DTransformation,
    /// The filter for hazards that the `Item` is unaffected by
    pub hazard_filter: Option<QZHazardFilter>,
    /// The shape of the `Item` after it has been transformed and placed in a `Layout`
    pub shape: Arc<SimplePolygon>,
}

impl PlacedItem {
    pub fn new(item: &Item, d_transf: DTransformation) -> Self {
        let transf = d_transf.compose();
        let shape = Arc::new(item.shape.transform_clone(&transf));
        let qz_haz_filter = item.hazard_filter.clone();

        PlacedItem {
            item_id: item.id,
            d_transf,
            shape,
            hazard_filter: qz_haz_filter,
        }
    }
}
