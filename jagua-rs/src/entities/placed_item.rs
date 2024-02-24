use std::sync::Arc;

use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::collision_detection::hazard_filter::QZHazardFilter;
use crate::entities::item::Item;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::geo_traits::Transformable;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

/// Represents an `Item` that has been placed in a `Layout`
#[derive(Clone, Debug)]
pub struct PlacedItem {
    /// Unique identifier for the placed item
    pub uid: PlacedItemUID,
    pub qz_haz_filter: Option<QZHazardFilter>,
    /// The shape of the `Item` after it has been transformed and placed in a `Layout`
    pub shape: Arc<SimplePolygon>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Unique identifier for a placed item
pub struct PlacedItemUID {
    pub item_id: usize,
    pub d_transf: DTransformation,
}

impl PlacedItem {
    pub fn new(item: &Item, d_transf: DTransformation) -> Self {
        let transf = d_transf.compose();
        let shape = Arc::new(item.shape.transform_clone(&transf));
        let qz_haz_filter = item.hazard_filter.clone();
        let pi_uid = PlacedItemUID {
            item_id: item.id,
            d_transf,
        };
        PlacedItem {
            uid: pi_uid,
            shape,
            qz_haz_filter,
        }
    }

    pub fn item_id(&self) -> usize {
        self.uid.item_id
    }

    pub fn d_transformation(&self) -> &DTransformation {
        &self.uid.d_transf
    }
}

impl Into<Hazard> for &PlacedItem {
    fn into(self) -> Hazard {
        Hazard::new(self.into(), self.shape.clone())
    }
}

impl Into<HazardEntity> for &PlacedItem {
    fn into(self) -> HazardEntity {
        HazardEntity::PlacedItem(self.uid.clone())
    }
}
