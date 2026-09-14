use crate::entities::Item;
use crate::probs::spp::entities::Strip;
use crate::probs::spp::util::assertions;

#[derive(Debug, Clone)]
/// Instance of the Strip Packing Problem.
pub struct SPInstance {
    /// The items to be packed and their demands
    pub items: Vec<(Item, usize)>,
    /// The strip in which to pack the items
    pub base_strip: Strip,
    /// Sorted external IDs indexed by internal item ID; not serialized separately.
    external_item_ids: Vec<u64>,
}

impl SPInstance {
    #[must_use]
    pub fn new(items: Vec<(Item, usize)>, base_strip: Strip, external_item_ids: Vec<u64>) -> Self {
        assert!(
            assertions::instance_item_ids_correct(&items),
            "All items should have consecutive IDs starting from 0"
        );

        assert_eq!(items.len(), external_item_ids.len());
        assert!(external_item_ids.windows(2).all(|w| w[0] < w[1]));
        Self {
            items,
            base_strip,
            external_item_ids,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn item_area(&self) -> f32 {
        self.items
            .iter()
            .map(|(item, qty)| item.shape_orig.area() * *qty as f32)
            .sum()
    }

    #[must_use]
    pub fn item_qty(&self, id: usize) -> usize {
        self.items[id].1
    }

    #[must_use]
    pub fn total_item_qty(&self) -> usize {
        self.items.iter().map(|(_, qty)| *qty).sum()
    }

    /// Retrieve an item by its internal index.
    #[must_use]
    pub fn item(&self, id: usize) -> &Item {
        &self.items[id].0
    }

    /// Resolve an internal item index to the caller's original ID.
    #[must_use]
    pub fn external_item_id(&self, id: usize) -> u64 {
        self.external_item_ids[id]
    }

    /// Resolve an external item ID, returning None for unknown or zero-demand items.
    #[must_use]
    pub fn internal_item_id(&self, id: u64) -> Option<usize> {
        self.external_item_ids.binary_search(&id).ok()
    }
}
