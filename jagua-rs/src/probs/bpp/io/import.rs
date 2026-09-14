use crate::io::import::Importer;
use crate::probs::bpp::entities::{BPInstance, BPSolution, Bin};
use crate::probs::bpp::io::ext_repr::ExtBPInstance;
use itertools::Itertools;
use rayon::prelude::*;

use anyhow::{Result, ensure};

/// Imports an instance into the library
pub fn import_instance(importer: &Importer, ext_instance: &ExtBPInstance) -> Result<BPInstance> {
    ensure!(
        ext_instance
            .items
            .iter()
            .map(|item| item.base.id)
            .all_unique(),
        "item IDs must be unique"
    );
    let items = ext_instance
        .items
        .iter()
        .filter(|item| item.demand > 0)
        .enumerate()
        .map(|(idx, item)| {
            Ok((
                importer.import_item(&item.base, idx)?,
                usize::try_from(item.demand)?,
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    ensure!(!items.is_empty(), "instance must have positive item demand");

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
