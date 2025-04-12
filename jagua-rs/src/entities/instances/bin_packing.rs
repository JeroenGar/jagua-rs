use crate::entities::bin::Bin;
use crate::entities::instances::instance::Instance;
use crate::entities::item::Item;
use crate::fsize;
use crate::geometry::geo_traits::Shape;
use crate::util::assertions;

/// Bin-packing problem instance: a set of items to be packed into a set of bins.
/// The items are to be packed in such a way that the total cost of the bins used is minimized.
#[derive(Debug, Clone)]
pub struct BPInstance {
    /// Items to be packed in the instance, along with their requested quantities
    pub items: Vec<(Item, usize)>,
    /// Total area of all items in the instance
    pub item_area: fsize,
    /// Set of bins available to pack the items, along with their quantities
    pub bins: Vec<(Bin, usize)>,
}

impl BPInstance {
    pub fn new(items: Vec<(Item, usize)>, bins: Vec<(Bin, usize)>) -> Self {
        assert!(assertions::instance_item_bin_ids_correct(&items, &bins));

        let item_area = items
            .iter()
            .map(|(item, qty)| item.shape.area() * *qty as fsize)
            .sum();

        Self {
            items,
            item_area,
            bins,
        }
    }
}

impl Instance for BPInstance {
    fn items(&self) -> &[(Item, usize)] {
        &self.items
    }

    fn bins(&self) -> &[(Bin, usize)] {
        &self.bins
    }

    fn bin_qty(&self, id: usize) -> usize {
        self.bins[id].1
    }

    fn bin(&self, id: usize) -> &Bin {
        &self.bins[id].0
    }

    fn item_area(&self) -> fsize {
        self.item_area
    }
}
