use crate::io::import::{Importer, import_demand_items};
use crate::probs::bpp::entities::{BPInstance, BPSolution, Bin};
use crate::probs::bpp::io::ext_repr::ExtBPInstance;
use itertools::Itertools;
use rayon::prelude::*;

use anyhow::{Result, ensure};

/// Imports an instance into the library
pub fn import_instance(importer: &Importer, ext_instance: &ExtBPInstance) -> Result<BPInstance> {
    let items = import_demand_items(
        importer,
        ext_instance
            .items
            .iter()
            .map(|item| (&item.base, item.demand)),
    )?;

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
        bins.retain(|bin| bin.stock > 0);
        ensure!(
            bins.iter().enumerate().all(|(i, bin)| bin.id == i),
            "All bins should have consecutive IDs starting from 0. IDs: {:?}",
            bins.iter().map(|bin| bin.id).sorted().collect_vec()
        );
        ensure!(
            !bins.is_empty(),
            "ExtBPInstance must have at least one bin with positive stock"
        );

        bins
    };

    Ok(BPInstance::new(items, bins))
}

/// Imports a solution into the library.
#[must_use]
pub fn import_solution(_instance: &BPInstance, _ext_solution: &ExtBPInstance) -> BPSolution {
    unimplemented!("not yet implemented")
}
