use crate::entities::Item;
use crate::geometry::DTransformation;
use crate::geometry::geo_traits::Transformable;
use crate::geometry::primitives::SPolygon;
use slotmap::new_key_type;
use std::sync::Arc;

#[cfg(doc)]
use crate::entities::Layout;

new_key_type! {
    /// Unique key for each [`PlacedItem`] in a layout.
    pub struct PItemKey;
}

/// Represents an [`Item`] that has been placed in a [`Layout`]
#[derive(Clone, Debug)]
pub struct PlacedItem {
    /// Shared immutable item definition, retained by layouts and snapshots.
    pub(crate) item: Arc<Item>,
    /// The transformation that was applied to the `Item` before it was placed
    pub(crate) d_transf: DTransformation,
    /// The shape of the `Item` after it has been transformed and placed in a `Layout`
    pub(crate) shape: Arc<SPolygon>,
}

impl PlacedItem {
    #[must_use]
    pub fn new(item: &Arc<Item>, d_transf: DTransformation) -> Self {
        let transf = d_transf.compose();
        let shape = item.shape_cd.transform_clone(&transf);

        PlacedItem {
            item: Arc::clone(item),
            d_transf,
            shape: Arc::new(shape),
        }
    }

    /// Shared immutable item definition, retained by layouts and snapshots.
    #[must_use]
    pub fn item(&self) -> &Arc<Item> {
        &self.item
    }

    /// The transformation that was applied to the `Item` before it was placed
    #[must_use]
    pub fn d_transf(&self) -> DTransformation {
        self.d_transf
    }

    /// The shape of the `Item` after it has been transformed and placed in a `Layout`
    #[must_use]
    pub fn shape(&self) -> &Arc<SPolygon> {
        &self.shape
    }
}
