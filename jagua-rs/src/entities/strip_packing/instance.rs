use crate::entities::general::Bin;
use crate::entities::general::Instance;
use crate::entities::general::Item;
use crate::fsize;
use crate::geometry::geo_traits::Shape;
use crate::util::assertions;

#[derive(Debug, Clone)]
/// Instance of the Strip Packing Problem: a set of items to be packed into a single strip with a fixed height and variable width.
pub struct SPInstance {
    /// The items to be packed and their quantities
    pub items: Vec<(Item, usize)>,
    /// The total area of the items
    pub item_area: fsize,
    /// The (fixed) height of the strip
    pub strip_height: fsize,
}

impl SPInstance {
    pub fn new(items: Vec<(Item, usize)>, strip_height: fsize) -> Self {
        assert!(assertions::instance_item_bin_ids_correct(&items, &[]));

        let item_area = items
            .iter()
            .map(|(item, qty)| item.shape.area() * *qty as fsize)
            .sum();

        Self {
            items,
            item_area,
            strip_height,
        }
    }
}

impl Instance for SPInstance {
    fn items(&self) -> &[(Item, usize)] {
        &self.items
    }

    fn bins(&self) -> &[(Bin, usize)] {
        &[]
    }
}
