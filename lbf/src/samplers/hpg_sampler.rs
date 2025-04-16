use crate::opt::loss::LBFLoss;
use crate::samplers::uniform_rect_sampler::UniformAARectSampler;
use itertools::Itertools;
use jagua_rs::collision_detection::CDEngine;
use jagua_rs::entities::general::Item;
use jagua_rs::fsize;
use jagua_rs::geometry::DTransformation;
use jagua_rs::geometry::Transformation;
use jagua_rs::geometry::geo_traits::Shape;
use jagua_rs::geometry::primitives::AARectangle;
use log::debug;
use rand::Rng;
use rand::prelude::IndexedRandom;

/// Creates `Transformation` samples for a given item.
/// Samples from the Hazard Proximity Grid uniformly, but only cells which could accommodate the item.
/// Cells were a collision is guaranteed are discarded.
pub struct HPGSampler<'a> {
    pub item: &'a Item,
    pub cell_samplers: Vec<UniformAARectSampler>,
    pub loss_bound: LBFLoss,
    pub pretransform: Transformation,
    pub coverage_area: fsize,
    pub bin_bbox_area: fsize,
    pub n_samples: usize,
}

impl<'a> HPGSampler<'a> {
    pub fn new(item: &'a Item, cde: &CDEngine) -> Option<HPGSampler<'a>> {
        let poi = &item.shape_cd.poi;
        let bin_bbox = &cde.bbox;

        //create a pre-transformation which centers the shape around its Pole of Inaccessibility.
        let pretransform = Transformation::from_translation((-poi.center.0, -poi.center.1));

        let hpg = cde.haz_prox_grid().unwrap();
        let all_cells = hpg.grid.cells.iter().flatten();
        let eligible_cells = all_cells.filter(|c| c.could_accommodate_item(item));

        //create samplers for all eligible cells
        let cell_samplers = eligible_cells
            .filter_map(|c| {
                //map each eligible cell to a rectangle sampler, bounded by the CDE's bbox.
                //(at low densities, the cells could extend significantly beyond the CDE's bbox)
                AARectangle::from_intersection(&c.bbox, &bin_bbox)
            })
            .map(|bbox| UniformAARectSampler::new(bbox, item))
            .collect_vec();

        let coverage_area = cell_samplers.iter().map(|s| s.bbox.area()).sum();

        let loss_bound = LBFLoss::new(bin_bbox.x_max, bin_bbox.y_max);

        match cell_samplers.is_empty() {
            true => {
                debug!("[HPG] no eligible cells to sample from");
                None
            }
            false => {
                debug!(
                    "[HPGS] created sampler with {} eligible cells, coverage: {:.3}%",
                    cell_samplers.len(),
                    coverage_area / bin_bbox.area() * 100.0
                );
                Some(HPGSampler {
                    item,
                    cell_samplers,
                    loss_bound,
                    pretransform,
                    coverage_area,
                    bin_bbox_area: bin_bbox.area(),
                    n_samples: 0,
                })
            }
        }
    }

    /// Samples a `Transformation`
    pub fn sample(&mut self, rng: &mut impl Rng) -> DTransformation {
        self.n_samples += 1;

        //sample one of the eligible cells
        let cell_sampler = self.cell_samplers.choose(rng).expect("no active samplers");

        //from that cell, sample a transformation
        let sample = cell_sampler.sample(rng);

        //combine the pretransform with the sampled transformation
        let t = self.pretransform.clone().transform_from_decomposed(&sample);

        t.decompose()
    }

    /// Removes all cells that cannot possibly generate a `Transformation` which would be better than the current best solution.
    /// LBF specific
    pub fn tighten(&mut self, best: LBFLoss) {
        let poi_rad = self.item.shape_cd.poi.radius;

        if best < self.loss_bound {
            //remove all cells that are out of bounds, update the coverage area
            self.cell_samplers.retain(|cell_sampler| {
                //minimum cost that could be achieved by a cell
                let min_cost = LBFLoss::new(
                    cell_sampler.bbox.x_min + poi_rad,
                    cell_sampler.bbox.y_min + poi_rad,
                );

                match min_cost < best {
                    true => true,
                    false => {
                        self.coverage_area -= cell_sampler.bbox.area();
                        false
                    }
                }
            });

            self.loss_bound = best;
            debug!(
                "[HPGS] tightened sampler to {} cells, coverage: {:.3}%",
                self.cell_samplers.len(),
                self.coverage_area / self.bin_bbox_area * 100.0
            );
        }
    }
}
