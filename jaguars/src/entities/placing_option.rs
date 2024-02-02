use crate::entities::problems::problem::LayoutIndex;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::transformation::Transformation;

#[derive(Clone, Debug)]
/// Represents a (valid) configuration of placing an item in a layout
pub struct PlacingOption {
    pub layout_index: LayoutIndex,
    pub item_id: usize,
    pub transf: Transformation,
    pub d_transf: DTransformation,
}