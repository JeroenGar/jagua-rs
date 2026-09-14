use crate::entities::{Container, Item};
use crate::probs::bpp::entities::bin::Bin;
use crate::probs::bpp::util::assertions::instance_item_bin_ids_correct;

#[derive(Debug, Clone)]
/// Instance of the Bin Packing Problem.
pub struct BPInstance {
    /// The items to be packed and their demands
    pub items: Vec<(Item, usize)>,
    /// Set of bins available to pack the items
    pub bins: Vec<Bin>,
}

impl BPInstance {
    #[must_use]
    pub fn new(items: Vec<(Item, usize)>, bins: Vec<Bin>) -> Self {
        assert!(instance_item_bin_ids_correct(&items, &bins));

        assert!(
            items
                .windows(2)
                .all(|w| w[0].0.external_id < w[1].0.external_id)
        );
        Self { items, bins }
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

    pub fn bins(&self) -> impl Iterator<Item = &Bin> {
        self.bins.iter()
    }

    #[must_use]
    pub fn bin_qty(&self, id: usize) -> usize {
        self.bins[id].stock
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

    /// Retrieve a bin's container by its index.
    #[must_use]
    pub fn container(&self, id: usize) -> &Container {
        &self.bins[id].container
    }

    /// Resolve an external item ID, returning None for unknown or zero-demand items.
    #[must_use]
    pub fn internal_item_id(&self, id: u64) -> Option<usize> {
        self.items
            .binary_search_by_key(&id, |(item, _)| item.external_id)
            .ok()
    }
}
