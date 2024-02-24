use crate::entities::problems::problem_generic::LayoutIndex;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::transformation::Transformation;

#[derive(Clone, Debug)]
/// Encapsulates all required information to place an `Item` in a `Problem`
pub struct PlacingOption {
    pub layout_index: LayoutIndex,
    pub item_id: usize,
    pub transf: Transformation,
    pub d_transf: DTransformation,
}
