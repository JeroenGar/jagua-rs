use crate::entities::placed_item::PlacedItem;
use crate::entities::problems::problem_generic::LayoutIndex;
use crate::geometry::d_transformation::DTransformation;

#[derive(Clone, Debug, Copy)]
/// Encapsulates all required information to place an `Item` in a `Problem`
pub struct PlacingOption {
    /// Which layout to place the item in
    pub layout_idx: LayoutIndex,
    /// The id of the item to be placed
    pub item_id: usize,
    /// The decomposition of the transformation
    pub d_transf: DTransformation,
}

impl PlacingOption {
    pub fn from_placed_item(layout_idx: LayoutIndex, placed_item: &PlacedItem) -> Self {
        PlacingOption {
            layout_idx,
            item_id: placed_item.item_id,
            d_transf: placed_item.d_transf,
        }
    }
}
