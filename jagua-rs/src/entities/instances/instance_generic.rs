use crate::entities::item::Item;

/// Trait for shared functionality of all instance variants.
pub trait InstanceGeneric {
    fn items(&self) -> &[(Item, usize)];
    fn item_qty(&self, id: usize) -> usize{
        self.items()[id].1
    }
    fn item(&self, id: usize) -> &Item {
        &self.items()[id].0
    }
    fn total_item_qty(&self) -> usize{
        self.items().iter().map(|(_, qty)| qty).sum()
    }

    fn item_area(&self) -> f64;
}