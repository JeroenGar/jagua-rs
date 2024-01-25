use std::time::Instant;

use itertools::Itertools;

use crate::entities::instance::{Instance, PackingType};
use crate::entities::stored_layout::StoredLayout;
use crate::geometry::geo_traits::Shape;

#[derive(Debug, Clone)]
pub struct Solution {
    id: usize,
    stored_layouts: Vec<StoredLayout>,
    usage: f64,
    included_item_qtys: Vec<usize>,
    target_item_qtys: Vec<usize>,
    bin_qtys: Vec<usize>,
    time_stamp: Instant,
}

impl Solution {
    pub fn new(id: usize, stored_layouts: Vec<StoredLayout>, usage: f64, included_item_qtys: Vec<usize>, target_item_qtys: Vec<usize>, bin_qtys: Vec<usize>) -> Self {
        Solution {
            id,
            stored_layouts,
            usage,
            included_item_qtys,
            target_item_qtys,
            bin_qtys,
            time_stamp: Instant::now(),
        }
    }

    pub fn stored_layouts(&self) -> &Vec<StoredLayout> {
        &self.stored_layouts
    }

    pub fn is_complete(&self, instance: &Instance) -> bool {
        self.included_item_qtys.iter().enumerate().all(|(i, &qty)| qty >= instance.item_qty(i))
    }

    pub fn completeness(&self, instance: &Instance) -> f64 {
        //ratio of included item area vs total instance item area
        let total_item_area = instance.item_area();
        let included_item_area = self.included_item_qtys.iter().enumerate()
            .map(|(i, qty)| instance.item(i).shape().area() * *qty as f64)
            .sum::<f64>();
        let completeness = included_item_area / total_item_area;
        completeness
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn included_item_qtys(&self) -> &Vec<usize> {
        &self.included_item_qtys
    }

    pub fn missing_item_qtys(&self, instance: &Instance) -> Vec<isize> {
        debug_assert!(instance.items().len() == self.included_item_qtys.len());
        self.included_item_qtys.iter().enumerate()
            .map(|(i, &qty)| instance.item_qty(i) as isize - qty as isize)
            .collect_vec()
    }

    pub fn bin_qtys(&self) -> &Vec<usize> {
        &self.bin_qtys
    }

    pub fn usage(&self) -> f64 {
        self.usage
    }

    pub fn target_item_qtys(&self) -> &Vec<usize> {
        &self.target_item_qtys
    }

    pub fn is_best_possible(&self, instance: &Instance) -> bool {
        match instance.packing_type() {
            PackingType::StripPacking { .. } => false,
            PackingType::BinPacking(bins) => {
                match self.stored_layouts.len() {
                    0 => panic!("No stored layouts in solution"),
                    1 => {
                        let cheapest_bin = &bins.iter().min_by(|(b1, _), (b2, _)| b1.value().cmp(&b2.value())).unwrap().0;
                        self.stored_layouts[0].bin().id() == cheapest_bin.id()
                    }
                    _ => false
                }
            }
        }
    }

    pub fn time_stamp(&self) -> Instant {
        self.time_stamp
    }
}
