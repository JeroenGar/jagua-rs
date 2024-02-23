use itertools::Itertools;
use log::debug;
use rand::prelude::SliceRandom;
use rand::Rng;

use jagua_rs::entities::item::Item;
use jagua_rs::entities::layout::Layout;
use jagua_rs::geometry::geo_traits::Shape;
use jagua_rs::geometry::primitives::aa_rectangle::AARectangle;
use jagua_rs::geometry::transformation::Transformation;

use crate::lbf_cost::LBFPlacingCost;
use crate::samplers::uniform_rect_sampler::UniformAARectSampler;

pub struct HPGSampler<'a> {
    pub item: &'a Item,
    pub cell_samplers: Vec<UniformAARectSampler>,
    pub x_bound: f64,
    pub pretransform: Transformation,
    pub coverage_area: f64,
}

impl<'a> HPGSampler<'a> {
    pub fn new(item: &'a Item, layout: &Layout) -> Option<HPGSampler<'a>> {
        let poi = &item.shape.poi;
        let layout_bbox = layout.bin().bbox();

        let hpg = layout.cde().haz_prox_grid().expect("grid changes present");

        let hpg_cells = &hpg.grid.cells;

        //center the shape's POI to the origin
        let pretransform = Transformation::from_translation((-poi.center.0, -poi.center.1));

        //collect all eligible cells from the Hazard Proximity Grid
        let cell_samplers = hpg_cells.iter()
            .flatten()
            .filter(|cell| cell.could_accommodate_item(item))
            .map(|cell| {
                //in cases of low cell count, the cell might extend beyond the layout's bbox
                let bbox = AARectangle::new(
                    f64::max(cell.bbox().x_min, layout_bbox.x_min),
                    f64::max(cell.bbox().y_min, layout_bbox.y_min),
                    f64::min(cell.bbox().x_max, layout_bbox.x_max),
                    f64::min(cell.bbox().y_max, layout_bbox.y_max),
                );
                UniformAARectSampler::new(bbox, item)
            }).collect_vec();

        let coverage_area = cell_samplers.iter()
            .map(|s| s.bbox.area())
            .sum();

        let x_bound = layout.bin().bbox().x_max;

        match cell_samplers.is_empty() {
            true => None,
            false => Some(HPGSampler { item, cell_samplers, x_bound, pretransform, coverage_area })
        }
    }
    pub fn sample(&self, rng: &mut impl Rng) -> Transformation {
        //first step: sample a cell
        let cell_sampler = self.cell_samplers.choose(rng).expect("no active samplers");

        let sample = cell_sampler.sample_x_bounded(rng, self.x_bound);

        //apply the sample to the pretransform
        self.pretransform.clone().transform_from_decomposed(&sample)
    }

    pub fn tighten_x_bound(&mut self, x_max: f64) {
        let poi_rad = self.item.shape.poi.radius;
        let new_x_bound = x_max - poi_rad; //we need at least one POI radius of space to the left of the best solution

        if new_x_bound < self.x_bound {
            debug!("tightening x bound from {} to {}", self.x_bound, new_x_bound);
            //remove all cells that are out of bounds, update the coverage area
            self.cell_samplers
                .retain(|cell_sampler| {
                    let in_bounds = cell_sampler.bbox.x_min < new_x_bound;

                    if !in_bounds {
                        self.coverage_area -= cell_sampler.bbox.area();
                    }
                    in_bounds
                });

            self.x_bound = new_x_bound
        }
    }
}