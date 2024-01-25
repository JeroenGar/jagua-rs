use std::sync::Arc;

use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::collision_detection::hazards::qz_haz_filter::QZHazardFilter;
use crate::entities::item::Item;
use crate::entities::placed_item_uid::PlacedItemUID;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::geo_traits::Transformable;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

#[derive(Clone, Debug)]
pub struct PlacedItem {
    pi_uid: PlacedItemUID,
    qz_haz_filter: Option<QZHazardFilter>,
    shape: Arc<SimplePolygon>,
}

impl PlacedItem {
    pub fn new(item: &Item, d_transformation: DTransformation) -> Self {
        let transformation = d_transformation.compose();
        let shape = Arc::new(item.shape().transform_clone(&transformation));
        let qz_haz_filter = item.hazard_filter().cloned();
        let pi_uid = PlacedItemUID::new(item.id(), d_transformation);
        PlacedItem {
            pi_uid,
            shape,
            qz_haz_filter,
        }
    }

    pub fn item_id(&self) -> usize {
        self.pi_uid.item_id()
    }

    pub fn d_transformation(&self) -> &DTransformation {
        self.pi_uid.d_transformation()
    }

    pub fn shape(&self) -> &Arc<SimplePolygon> {
        &self.shape
    }

    pub fn uid(&self) -> &PlacedItemUID {
        &self.pi_uid
    }


    pub fn haz_filter(&self) -> &Option<QZHazardFilter> {
        &self.qz_haz_filter
    }
}

impl Into<Hazard> for &PlacedItem {
    fn into(self) -> Hazard {
        Hazard::new(self.into(), self.shape.clone())
    }
}

impl Into<HazardEntity> for &PlacedItem {
    fn into(self) -> HazardEntity {
        HazardEntity::Item(self.pi_uid.clone())
    }
}
