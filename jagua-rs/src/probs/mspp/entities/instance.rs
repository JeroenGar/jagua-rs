use crate::entities::Item;
use crate::probs::mspp::entities::strip::Strip;
use crate::probs::mspp::util::assertions;
use itertools::Itertools;
use std::sync::Arc;

#[derive(Debug, Clone)]
/// Instance of the "Multi Strip Packing Problem".
/// The goal is to pack a set of items into strips of fixed height and variable width, minimizing the total width used.
/// The strips have a maximum width and multiple strips can be used to pack all items.
pub struct MSPInstance {
    /// The items to be packed and their demands
    pub items: Vec<(Arc<Item>, usize)>,
    /// The strip in which to pack the items
    pub base_strip: Strip,
}

impl MSPInstance {
    #[must_use]
    pub fn new(items: Vec<(Arc<Item>, usize)>, base_strip: Strip) -> Self {
        assert!(
            assertions::instance_item_ids_correct(&items),
            "All items should have consecutive IDs starting from 0"
        );

        assert!(items.iter().map(|(item, _)| item.external_id).all_unique());
        Self { items, base_strip }
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
    pub fn item(&self, id: usize) -> &Arc<Item> {
        &self.items[id].0
    }

    /// Resolve an external item ID, returning None for unknown or zero-demand items.
    #[must_use]
    pub fn internal_item_id(&self, id: u64) -> Option<usize> {
        self.items
            .iter()
            .find(|(item, _)| item.external_id == id)
            .map(|(item, _)| item.idx)
    }
}
