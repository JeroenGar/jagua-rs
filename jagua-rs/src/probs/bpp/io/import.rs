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
        let mut entries = ext_instance.bins.iter().collect_vec();
        entries.sort_by_key(|bin| bin.base.id);
        ensure!(
            entries.windows(2).all(|w| w[0].base.id != w[1].base.id),
            "bin IDs must be unique"
        );
        entries.retain(|bin| bin.stock > 0);
        ensure!(!entries.is_empty(), "instance must have positive bin stock");
        entries
            .par_iter()
            .enumerate()
            .map(|(idx, bin)| {
                Ok(Bin::new(
                    idx,
                    bin.base.id,
                    importer.import_container(&bin.base)?,
                    bin.stock,
                    bin.cost,
                ))
            })
            .collect::<Result<Vec<_>>>()?
    };

    Ok(BPInstance::new(items, bins))
}

/// Imports a solution into the library.
#[must_use]
pub fn import_solution(_instance: &BPInstance, _ext_solution: &ExtBPInstance) -> BPSolution {
    unimplemented!("not yet implemented")
}
