use crate::entities::problems::problem::LayoutIndex;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::transformation::Transformation;

#[derive(Clone, Debug)]
/// Contains information about how an `Item` with a `Transformation` applied can be placed in a `Layout`
pub struct PlacingOption {
    pub layout_index: LayoutIndex,
    pub item_id: usize,
    pub transf: Transformation,
    pub d_transf: DTransformation,
}