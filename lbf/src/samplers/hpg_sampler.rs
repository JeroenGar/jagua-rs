use itertools::Itertools;
use log::{debug, info};

use rand::distributions::uniform::UniformSampler;
use rand::prelude::{SliceRandom, SmallRng};

use jaguars::entities::item::Item;
use jaguars::entities::layout::Layout;


use jaguars::geometry::geo_traits::{Shape};


use jaguars::geometry::transformation::Transformation;
use crate::lbf_cost::LBFCost;

use crate::samplers::uniform_rect_sampler::UniformAARectSampler;

pub struct HPGSampler<'a> {
    item: &'a Item,
    cell_samplers: Vec<UniformAARectSampler>,
    x_bound: f64,
    pretransform: Transformation,
    coverage_area: f64,
}

impl<'a> HPGSampler<'a> {
    pub fn new(item: &'a Item, layout: &Layout) -> Option<HPGSampler<'a>> {
        let poi = item.shape().poi();

        let hpg = layout.cde().haz_prox_grid().expect("grid changes present");

        let cell_radius = hpg.cell_radius();

        let hpg_cells = hpg.cells();

        //center the shape's POI to the origin
        let pretransform = Transformation::from_translation((-poi.center().0, -poi.center().1));

        //collect all eligible cells from the Hazard Proximity Grid
        let cell_samplers = hpg_cells.iter()
            .flatten()
            .filter(|cell| {
                let prox : f64 = (&cell.hazard_proximity(item.base_quality())).into();
                poi.radius() < prox + cell_radius
            })
            .map(|cell| UniformAARectSampler::new(cell.bbox().clone(), item))
            .collect_vec();

        let coverage_area = cell_samplers.iter()
            .map(|s| s.bbox.area())
            .sum();

        let x_bound = layout.bin().bbox().x_max();

        match cell_samplers.is_empty() {
            true => None,
            false => Some(HPGSampler { item, cell_samplers, x_bound, pretransform, coverage_area })
        }
    }

    pub fn coverage_area(&self) -> f64 {
        self.coverage_area
    }
    pub fn sample(&self, rng: &mut SmallRng) -> Transformation {
        //first step: sample a cell
        let cell_sampler = self.cell_samplers.choose(rng).expect("no active samplers");

        let sample = cell_sampler.sample_x_bounded(rng, self.x_bound);

        //apply the sample to the pretransform
        self.pretransform.clone().transform_from_decomposed(&sample)
    }

    pub fn tighten_x_bound(&mut self, best: &LBFCost){
        let poi_rad = self.item.shape().poi().radius();
        let new_x_bound = *best.x_max - poi_rad; //we need at least one POI radius of space to the left of the best solution

        if new_x_bound < self.x_bound {
            debug!("tightening x bound from {} to {}", self.x_bound, new_x_bound);
            //remove all cells that are out of bounds, update the coverage area
            self.cell_samplers
                .retain(|cell_sampler| {
                    let in_bounds = cell_sampler.bbox.x_min() < new_x_bound;

                    if !in_bounds {
                        self.coverage_area -= cell_sampler.bbox.area();
                    }
                    in_bounds
                });

            self.x_bound = new_x_bound
        }
    }
}