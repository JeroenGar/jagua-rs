use std::iter;

use itertools::Itertools;

use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::collision_detection::hpg::boundary_fill::BoundaryFillGrid;
use crate::collision_detection::hpg::grid::Grid;
use crate::collision_detection::hpg::grid_generator;
use crate::collision_detection::hpg::hpg_cell::{HPGCell, HPGCellUpdate};
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::aa_rectangle::AARectangle;

/// Grid of cells which store information about hazards in their vicinity.
/// The grid is a part of the `CDEngine` and is thus automatically updated when hazards are registered or deregistered.
#[derive(Debug, Clone)]
pub struct HazardProximityGrid {
    pub grid: Grid<HPGCell>,
    pending_deregisters: Vec<HazardEntity>,
    pub cell_radius : f64,
}

impl HazardProximityGrid {
    pub fn new(bbox: AARectangle, static_hazards: &[Hazard], n_cells: usize) -> Self {
        assert!(n_cells > 0);
        let cells = grid_generator::generate(bbox, static_hazards, n_cells);
        let cell_radius = cells[0].diameter() / 2.0;

        let grid = {
            let elements = cells.into_iter()
                .map(|bbox| HPGCell::new(bbox, static_hazards))
                .map(|cell| {
                    let pos = cell.centroid();
                    (cell, pos)
                })
                .collect_vec();
            Grid::new(elements)
        };

        HazardProximityGrid {
            grid,
            pending_deregisters: vec![],
            cell_radius
        }
    }

    pub fn restore(&mut self, grid: Grid<HPGCell>) {
        assert_eq!(self.grid.cells.len(), grid.cells.len());
        self.grid = grid;
        self.pending_deregisters.clear();
    }

    pub fn register_hazard(&mut self, to_register: &Hazard) {
        let seed_bbox = {
            let shape_bbox = to_register.shape.bbox();
            AARectangle::new(
                shape_bbox.x_min - self.cell_radius,
                shape_bbox.y_min - self.cell_radius,
                shape_bbox.x_max + self.cell_radius,
                shape_bbox.y_max + self.cell_radius,
            )
        };

        let mut b_fill = BoundaryFillGrid::new(&self.grid, seed_bbox);

        while let Some(next_dot_index) = b_fill.pop(&self.grid) {
            let cell = self.grid.cells[next_dot_index].as_mut();
            if let Some(cell) = cell {
                match cell.register_hazard(to_register) {
                    HPGCellUpdate::Affected => {
                        b_fill.queue_neighbors(next_dot_index, &self.grid);
                    }
                    HPGCellUpdate::Unaffected => {
                        b_fill.queue_neighbors(next_dot_index, &self.grid);
                    }
                    HPGCellUpdate::Boundary => ()
                }
            }
        }

        //TODO: move this to an assertion check
        debug_assert!(
            {
                let old_cells = self.grid.cells.clone();

                //ensure no changes remain
                let undetected = self.grid.cells.iter_mut().enumerate()
                    .flat_map(|(i, cell)| cell.as_mut().map(|cell| (i, cell)))
                    .map(|(i, cell)| (i, cell.register_hazard(to_register)))
                    .filter(|(_i, res)| res == &HPGCellUpdate::Affected)
                    .map(|(i, _res)| i)
                    .collect_vec();

                let undetected_row_cols = undetected.iter().map(|i| self.grid.to_row_col(*i).unwrap()).collect_vec();

                if undetected.len() != 0 {
                    println!("{:?} undetected affected cells", undetected_row_cols);
                    for i in undetected {
                        println!("old {:?}", &old_cells[i]);
                        println!("new {:?}", &self.grid.cells[i]);
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
                for cell in self.grid.cells.iter_mut().flatten() {
                    let result = cell.deregister_hazards(iter::once(to_deregister), remaining.clone());
                    match result {
                        HPGCellUpdate::Affected => (),
                        HPGCellUpdate::Unaffected => (),
                        HPGCellUpdate::Boundary => unreachable!()
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

            for cell in self.grid.cells.iter_mut().flatten() {
                cell.deregister_hazards(to_deregister.clone(), remaining.clone());
            }

            self.pending_deregisters.clear();
        }
    }

    pub fn has_pending_deregisters(&self) -> bool {
        !self.pending_deregisters.is_empty()
    }

}


/// Error type for when the `HazardProximityGrid` cannot be accessed due to pending changes.
/// To avoid this error, ensure all changes are flushed before requesting the grid.
#[derive(Debug)]
pub struct PendingChangesErr;
