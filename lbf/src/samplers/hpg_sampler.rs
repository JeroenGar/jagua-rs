use itertools::Itertools;
use log::debug;
use rand::Rng;
use rand::prelude::IndexedRandom;

use jagua_rs::entities::item::Item;
use jagua_rs::entities::layout::Layout;
use jagua_rs::fsize;
use jagua_rs::geometry::geo_traits::Shape;
use jagua_rs::geometry::primitives::aa_rectangle::AARectangle;
use jagua_rs::geometry::transformation::Transformation;

use crate::lbf_cost::LBFPlacingCost;
use crate::samplers::uniform_rect_sampler::UniformAARectSampler;

/// Creates `Transformation` samples for a given item.
/// Samples from the Hazard Proximity Grid uniformly, but only cells which could accommodate the item.
/// Cells were a collision is guaranteed are discarded.
pub struct HPGSampler<'a> {
    pub item: &'a Item,
    pub cell_samplers: Vec<UniformAARectSampler>,
    pub cost_bound: LBFPlacingCost,
    pub pretransform: Transformation,
    pub coverage_area: fsize,
    pub bin_bbox_area: fsize,
    pub n_samples: usize,
}

impl<'a> HPGSampler<'a> {
    pub fn new(item: &'a Item, layout: &Layout) -> Option<HPGSampler<'a>> {
        let poi = &item.shape.poi;
        let bin_bbox = layout.bin.bbox();

        //create a pre-transformation which centers the shape around its Pole of Inaccessibility.
        let pretransform = Transformation::from_translation((-poi.center.0, -poi.center.1));

        let hpg = layout.cde().haz_prox_grid().unwrap();
        let all_cells = hpg.grid.cells.iter().flatten();
        let eligible_cells = all_cells.filter(|c| c.could_accommodate_item(item));

        //create samplers for all eligible cells
        let cell_samplers = eligible_cells
            .filter_map(|c| {
                //map each eligible cell to a rectangle sampler, bounded by the layout's bbox.
                //(at low densities, the cells could extend significantly beyond the layout's bbox)
                AARectangle::from_intersection(&c.bbox, &bin_bbox)
            })
            .map(|bbox| UniformAARectSampler::new(bbox, item))
            .collect_vec();

        let coverage_area = cell_samplers.iter().map(|s| s.bbox.area()).sum();

        let cost_bound = LBFPlacingCost::new(bin_bbox.x_max, bin_bbox.y_max);

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
                    cost_bound,
                    pretransform,
                    coverage_area,
                    bin_bbox_area: bin_bbox.area(),
                    n_samples: 0,
                })
            }
        }
    }

    /// Samples a `Transformation`
    pub fn sample(&mut self, rng: &mut impl Rng) -> Transformation {
        self.n_samples += 1;

        //sample one of the eligible cells
        let cell_sampler = self.cell_samplers.choose(rng).expect("no active samplers");

        //from that cell, sample a transformation
        let sample = cell_sampler.sample(rng);

        //combine the pretransform with the sampled transformation
        self.pretransform.clone().transform_from_decomposed(&sample)
    }

    /// Removes all cells that cannot possibly generate a `Transformation` which would be better than the current best solution.
    /// LBF specific
    pub fn tighten(&mut self, best: LBFPlacingCost) {
        let poi_rad = self.item.shape.poi.radius;

        if best < self.cost_bound {
            //remove all cells that are out of bounds, update the coverage area
            self.cell_samplers.retain(|cell_sampler| {
                //minimum cost that could be achieved by a cell
                let min_cost = LBFPlacingCost::new(
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

            self.cost_bound = best;
            debug!(
                "[HPGS] tightened sampler to {} cells, coverage: {:.3}%",
                self.cell_samplers.len(),
                self.coverage_area / self.bin_bbox_area * 100.0
            );
        }
    }
}
