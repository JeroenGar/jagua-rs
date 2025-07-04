use crate::entities::placed_item::PlacedItem;
use crate::probs::bpp::entities::LayKey;
use crate::geometry::DTransformation;

#[derive(Clone, Debug, Copy)]
/// Encapsulates all required information to place an `Item` in a `Problem`
pub struct PlacingOption {
    /// Which layout to place the item in
    pub lay_key: LayKey,
    /// The id of the item to be placed
    pub item_id: usize,
    /// The decomposition of the transformation
    pub d_transf: DTransformation,
}

impl PlacingOption {
    pub fn from_placed_item(lay_key: LayKey, placed_item: &PlacedItem) -> Self {
        PlacingOption {
            lay_key,
            item_id: placed_item.item_id,
            d_transf: placed_item.d_transf,
        }
    }
}
