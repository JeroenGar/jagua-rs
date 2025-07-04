use crate::entities::Item;
use crate::io::import::Importer;
use crate::probs::bpp::entities::{BPInstance, Bin};
use crate::probs::bpp::io::ext_repr::ExtBPInstance;
use itertools::Itertools;
use rayon::prelude::*;

use anyhow::{Result, ensure};

/// Imports an instance into the library
pub fn import(importer: &Importer, ext_instance: &ExtBPInstance) -> Result<BPInstance> {
    let items = {
        let mut items = ext_instance
            .items
            .par_iter()
            .map(|ext_item| {
                let item = importer.import_item(&ext_item.base)?;
                let demand = ext_item.demand as usize;
                Ok((item, demand))
            })
            .collect::<Result<Vec<(Item, usize)>>>()?;

        items.sort_by_key(|(item, _)| item.id);
        ensure!(
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
                let container = importer.import_container(&ext_bin.base)?;
                Ok(Bin::new(container, ext_bin.stock, ext_bin.cost))
            })
            .collect::<Result<Vec<Bin>>>()?;

        bins.sort_by_key(|bin| bin.id);
        ensure!(
            bins.iter().enumerate().all(|(i, bin)| bin.id == i),
            "All bins should have consecutive IDs starting from 0. IDs: {:?}",
            bins.iter().map(|bin| bin.id).sorted().collect_vec()
        );
        bins
    };

    Ok(BPInstance::new(items, bins))
}
