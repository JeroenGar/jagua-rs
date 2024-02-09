use crate::entities::bin::Bin;
use crate::entities::item::Item;
use crate::geometry::geo_traits::Shape;
use crate::util::assertions;

/// Static representation of a problem instance.
#[derive(Debug)]
pub struct Instance {
    items: Vec<(Item, usize)>,
    item_area: f64,
    packing_type: PackingType,
}

impl Instance {
    pub fn new(items: Vec<(Item, usize)>, packing_type: PackingType) -> Instance {
        assert!(assertions::instance_item_bin_ids_correct(&items, &packing_type));

        let item_area = items.iter().map(|(item, qty)| item.shape().area() * *qty as f64).sum();

        Instance { items, item_area, packing_type }
    }

    pub fn bin(&self, id: usize) -> &Bin {
        match &self.packing_type {
            PackingType::BinPacking(bins) => &bins[id].0,
            PackingType::StripPacking { .. } => panic!("Instance is not a bin packing instance"),
        }
    }

    pub fn items(&self) -> &Vec<(Item, usize)> {
        &self.items
    }

    pub fn item_qty(&self, id: usize) -> usize {
        self.items[id].1
    }

    pub fn item(&self, id: usize) -> &Item {
        &self.items[id].0
    }

    pub fn total_item_qty(&self) -> usize {
        self.items.iter().map(|(_, qty)| qty).sum()
    }

    pub fn packing_type(&self) -> &PackingType {
        &self.packing_type
    }

    pub fn item_area(&self) -> f64 {
        self.item_area
    }
}


//TODO: clean this up
#[derive(Debug, Clone)]
pub enum PackingType {
    BinPacking(Vec<(Bin, usize)>),
    StripPacking { height: f64 },
}
