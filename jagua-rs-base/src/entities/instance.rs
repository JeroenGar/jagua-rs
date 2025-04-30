use crate::entities::Container;
use crate::entities::Item;
use std::any::Any;

/// The static (unmodifiable) representation of a problem instance.
/// This trait defines shared functionality between any instance variant.
pub trait Instance: Any {
    /// Returns the items in the instance, along with their requested quantities.
    fn items(&self) -> &[(Item, usize)];

    /// Returns the bins in the instance, along with their stock quantities.
    fn bins(&self) -> &[(Container, usize)];

    fn item_qty(&self, id: usize) -> usize {
        self.items()[id].1
    }
    fn item(&self, id: usize) -> &Item {
        &self.items()[id].0
    }

    fn bin_qty(&self, id: usize) -> usize {
        self.bins()[id].1
    }
    fn bin(&self, id: usize) -> &Container {
        &self.bins()[id].0
    }
    fn total_item_qty(&self) -> usize {
        self.items().iter().map(|(_, qty)| qty).sum()
    }
}
