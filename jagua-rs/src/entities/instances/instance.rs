use std::any::Any;
use crate::entities::bin::Bin;
use crate::entities::item::Item;
use crate::fsize;

/// An `Instance` is the static (unmodifiable) representation of a problem instance.
/// This trait defines shared functionality of all instance variants.
pub trait Instance: Any {
    fn items(&self) -> &[(Item, usize)];
    fn item_qty(&self, id: usize) -> usize {
        self.items()[id].1
    }
    fn item(&self, id: usize) -> &Item {
        &self.items()[id].0
    }

    fn bins(&self) -> &[(Bin, usize)];

    fn bin_qty(&self, id: usize) -> usize {
        self.bins()[id].1
    }
    fn bin(&self, id: usize) -> &Bin {
        &self.bins()[id].0
    }
    fn total_item_qty(&self) -> usize {
        self.items().iter().map(|(_, qty)| qty).sum()
    }
    fn item_area(&self) -> fsize;
}
