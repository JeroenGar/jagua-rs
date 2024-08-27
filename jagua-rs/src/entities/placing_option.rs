use crate::entities::problems::problem_generic::LayoutIndex;
use crate::geometry::d_transformation::DTransformation;

#[derive(Clone, Debug, Copy)]
/// Encapsulates all required information to place an `Item` in a `Problem`
pub struct PlacingOption {
    /// Which layout to place the item in
    pub layout_index: LayoutIndex,
    /// The id of the item to be placed
    pub item_id: usize,
    /// The decomposition of the transformation
    pub d_transf: DTransformation,
}
