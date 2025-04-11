use crate::entities::layout::LayKey;
use crate::entities::placed_item::PlacedItem;
use crate::geometry::d_transformation::DTransformation;

#[derive(Clone, Debug, Copy)]
/// Encapsulates all required information to place an `Item` in a `Problem`
pub struct PlacingOption {
    /// Which layout to place the item in
    pub layout: LayoutType,
    /// The id of the item to be placed
    pub item_id: usize,
    /// The decomposition of the transformation
    pub d_transf: DTransformation,
}

impl PlacingOption {
    pub fn from_placed_item(layout: LayoutType, placed_item: &PlacedItem) -> Self {
        PlacingOption {
            layout,
            item_id: placed_item.item_id,
            d_transf: placed_item.d_transf,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType{
    /// An existing layout (currently open)
    Open(LayKey),
    /// A new layout (currently closed)
    Closed{
        bin_id: usize
    }
}
