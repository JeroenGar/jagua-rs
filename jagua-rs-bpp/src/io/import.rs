use crate::entities::{BPInstance, Bin};
use crate::io::ext_repr::ExtBPInstance;
use itertools::Itertools;
use jagua_rs_base::entities::Item;
use jagua_rs_base::io::import::Importer;
use rayon::prelude::*;

/// Imports an instance into the library
pub fn import(importer: &Importer, ext_instance: &ExtBPInstance) -> BPInstance {
    let items = {
        let mut items: Vec<(Item, usize)> = ext_instance
            .items
            .par_iter()
            .map(|ext_item| {
                let item = importer.import_item(&ext_item.base);
                let demand = ext_item.demand as usize;
                (item, demand)
            })
            .collect();

        items.sort_by_key(|(item, _)| item.id);
        assert!(
            items.iter().enumerate().all(|(i, (item, _))| item.id == i),
            "All items should have consecutive IDs starting from 0. IDs: {:?}",
            items.iter().map(|(item, _)| item.id).sorted().collect_vec()
        );
        items
    };

    let bins = {
        let mut bins: Vec<Bin> = ext_instance
            .bins
            .par_iter()
            .map(|ext_bin| {
                let container = importer.import_container(&ext_bin.base);
                Bin::new(container, ext_bin.stock, ext_bin.cost)
            })
            .collect();

        bins.sort_by_key(|bin| bin.id);
        assert!(
            bins.iter().enumerate().all(|(i, bin)| bin.id == i),
            "All bins should have consecutive IDs starting from 0. IDs: {:?}",
            bins.iter().map(|bin| bin.id).sorted().collect_vec()
        );
        bins
    };

    BPInstance::new(items, bins)
}
