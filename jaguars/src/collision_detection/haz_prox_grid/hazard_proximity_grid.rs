use std::iter;

use itertools::Itertools;

use crate::collision_detection::haz_prox_grid::{grid_generator};
use crate::collision_detection::haz_prox_grid::boundary_fill::BoundaryFillGrid;
use crate::collision_detection::haz_prox_grid::grid::Grid;
use crate::collision_detection::haz_prox_grid::hp_cell::{HPCell, HPCellUpdate};
use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::geo_traits::Shape;


use crate::util::config::HazProxConfig;

#[derive(Debug, Clone)]
pub struct HazardProximityGrid {
    config: HazProxConfig,
    grid: Grid<HPCell>,
    cell_radius: f64,
    pending_deregisters: Vec<HazardEntity>,
}

impl HazardProximityGrid {
    pub fn new(bbox: AARectangle, static_hazards: &[Hazard], config: HazProxConfig) -> Self {
        let cells = match config {
            HazProxConfig::Number(n_dots_target) => grid_generator::generate(bbox, static_hazards, n_dots_target),
            HazProxConfig::Density => panic!("Not implemented yet"),
        };

        let cell_radius = {
            let half_width = cells[0].width() / 2.0;
            f64::sqrt(2.0 * half_width.powi(2))
        };


        let grid = {
            let elements = cells.into_iter()
                .map(|bbox| HPCell::new(bbox, static_hazards))
                .map(|cell| {
                    let centroid = cell.centroid();
                    (cell, centroid.into())
                }).collect_vec();
            Grid::new(elements)
        };

        HazardProximityGrid {
            config,
            grid,
            cell_radius,
            pending_deregisters: vec![],
        }
    }

    pub fn restore(&mut self, grid: Grid<HPCell>) {
        assert_eq!(self.grid.elements().len(), grid.elements().len());
        self.grid = grid;
        self.pending_deregisters.clear();
    }

    pub fn register_hazard(&mut self, to_register: &Hazard) {
        let seed_bbox = {
            let shape_bbox = to_register.shape().bbox();
            AARectangle::new(
                shape_bbox.x_min() - self.cell_radius,
                shape_bbox.y_min() - self.cell_radius,
                shape_bbox.x_max() + self.cell_radius,
                shape_bbox.y_max() + self.cell_radius,
            )
        };

        let mut b_fill = BoundaryFillGrid::new(seed_bbox, &self.grid);

        while let Some(next_dot_index) = b_fill.pop(&self.grid) {
            let cell = self.grid.elements_mut()[next_dot_index].as_mut();
            if let Some(cell) = cell {
                match cell.register_hazard(to_register) {
                    HPCellUpdate::Affected => {
                        b_fill.queue_neighbors(next_dot_index, &self.grid);
                    }
                    HPCellUpdate::Unaffected => {
                        b_fill.queue_neighbors(next_dot_index, &self.grid);
                    }
                    HPCellUpdate::Boundary => ()
                }
            }
        }

        debug_assert!(
            {
                let old_cells = self.grid.elements().clone();

                //ensure no changes remain
                let undetected = self.grid.elements_mut().iter_mut().enumerate()
                    .flat_map(|(i, cell)| cell.as_mut().map(|cell| (i, cell)))
                    .map(|(i, cell)| (i, cell.register_hazard(to_register)))
                    .filter(|(_i, res)| res == &HPCellUpdate::Affected)
                    .map(|(i, _res)| i)
                    .collect_vec();

                let undetected_row_cols = undetected.iter().map(|i| self.grid.get_row_col(*i).unwrap()).collect_vec();

                if undetected.len() != 0 {
                    println!("{:?} undetected affected cells", undetected_row_cols);
                    for i in undetected {
                        println!("old {:?}", &old_cells[i]);
                        println!("new {:?}", &self.grid.elements()[i]);
                    }
                    false
                } else {
                    true
                }
            }
        );
    }

    pub fn deregister_hazard<'a, I>(&mut self, to_deregister: &HazardEntity, remaining: I, process_now: bool)
        where I: Iterator<Item=&'a Hazard> + Clone
    {
        match process_now {
            true => {
                for cell in self.grid.elements_mut().iter_mut().flatten() {
                    let result = cell.deregister_hazards(iter::once(to_deregister), remaining.clone());
                    match result {
                        HPCellUpdate::Affected => (),
                        HPCellUpdate::Unaffected => (),
                        HPCellUpdate::Boundary => unreachable!()
                    }
                }
            }
            false => {
                self.pending_deregisters.push(to_deregister.clone());
            }
        }
    }

    pub fn flush_deregisters<'a, I>(&mut self, remaining: I)
        where I: Iterator<Item=&'a Hazard> + Clone
    {
        if self.has_pending_deregisters() {
            let to_deregister = self.pending_deregisters.iter();

            for cell in self.grid.elements_mut().iter_mut().flatten() {
                cell.deregister_hazards(to_deregister.clone(), remaining.clone());
            }

            self.pending_deregisters.clear();
        }
    }

    pub fn has_pending_deregisters(&self) -> bool {
        !self.pending_deregisters.is_empty()
    }

    pub fn cells(&self) -> &Vec<Option<HPCell>> {
        &self.grid.elements()
    }

    pub fn cell_radius(&self) -> f64 {
        self.cell_radius
    }

    pub fn grid(&self) -> &Grid<HPCell> {
        &self.grid
    }

    pub fn config(&self) -> HazProxConfig {
        self.config
    }

}

#[derive(Debug)]
pub struct PendingChangesErr;
