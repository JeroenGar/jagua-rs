use crate::geometry::d_transformation::DTransformation;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Unique identifier for a placed item
pub struct PlacedItemUID {
    item_id: usize,
    d_transf: DTransformation,
}

impl PlacedItemUID {
    pub fn new(item_id: usize, dt: DTransformation) -> Self {
        Self { d_transf: dt, item_id }
    }

    pub fn d_transformation(&self) -> &DTransformation {
        &self.d_transf
    }

    pub fn item_id(&self) -> usize {
        self.item_id
    }
}
