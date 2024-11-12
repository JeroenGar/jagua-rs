use std::fmt::{Display, Formatter};
use std::iter;

use itertools::Itertools;

use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;
use crate::collision_detection::hpg::boundary_fill::BoundaryFillHPG;
use crate::collision_detection::hpg::grid::Grid;
use crate::collision_detection::hpg::grid_generator;
use crate::collision_detection::hpg::hpg_cell::{HPGCell, HPGCellUpdate};
use crate::fsize;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::util::assertions;

/// Grid of cells which store information about hazards in their vicinity.
/// The grid is a part of the CDE and is thus automatically updated when hazards are registered or deregistered.
#[derive(Debug, Clone)]
pub struct HazardProximityGrid {
    pub bbox: AARectangle,
    pub grid: Grid<HPGCell>,
    pub cell_radius: fsize,
    uncommitted_deregisters: Vec<HazardEntity>,
}

impl HazardProximityGrid {
    pub fn new(bbox: AARectangle, static_hazards: &[Hazard], n_cells: usize) -> Self {
        assert!(n_cells > 0);

        let cells = {
            //all universal hazards are applicable for the grid generator
            let uni_hazards = static_hazards
                .iter()
                .filter(|h| h.entity.is_universal())
                .cloned()
                .collect_vec();
            grid_generator::generate(bbox.clone(), &uni_hazards, n_cells)
        };
        let cell_radius = cells[0].diameter() / 2.0;

        let grid = {
            let elements = cells
                .into_iter()
                .map(|bbox| HPGCell::new(bbox, static_hazards))
                .map(|cell| {
                    let pos = cell.centroid;
                    (cell, pos)
                })
                .collect_vec();
            Grid::new(elements)
        };

        HazardProximityGrid {
            bbox,
            grid,
            uncommitted_deregisters: vec![],
            cell_radius,
        }
    }

    pub fn restore(&mut self, grid: Grid<HPGCell>) {
        assert_eq!(self.grid.cells.len(), grid.cells.len());
        self.grid = grid;
        self.uncommitted_deregisters.clear();
    }

    pub fn register_hazard(&mut self, to_register: &Hazard) {
        let shape = &to_register.shape;
        let poles = &shape.surrogate().poles;

        //To update the grid efficiently, we use a boundary fill algorithm to propagate the effect of each pole through the grid
        let mut b_fill = BoundaryFillHPG::new(&self.grid, &shape.bbox());

        for pole in poles {
            let seed_box = AARectangle::new(
                pole.bbox().x_min - 2.0 * self.cell_radius,
                pole.bbox().y_min - 2.0 * self.cell_radius,
                pole.bbox().x_max + 2.0 * self.cell_radius,
                pole.bbox().y_max + 2.0 * self.cell_radius,
            );

            b_fill = b_fill.reset(&self.grid, &seed_box);

            //As long as the boundary fill keeps finding new cells, keep updating the grid
            while let Some(next_cell) = b_fill.pop() {
                let cell = self.grid.cells[next_cell].as_mut();
                if let Some(cell) = cell {
                    let cell_update_result = cell.register_hazard_pole(to_register, pole);
                    let position_in_bf = match cell_update_result {
                        //Cell was directly affected, inside the boundary
                        HPGCellUpdate::Affected => GeoPosition::Interior,
                        //Cell was not affected, but its neighbors might be, so it is considered inside the boundary
                        HPGCellUpdate::NotAffected => GeoPosition::Interior,
                        //Cell was not affected and its neighbors are not affected, so it is considered outside the boundary
                        HPGCellUpdate::NeighborsNotAffected => GeoPosition::Exterior,
                    };
                    b_fill.report_position(next_cell, position_in_bf, &self.grid);
                } else {
                    //cell does not exist, mark as exterior
                    b_fill.report_position(next_cell, GeoPosition::Exterior, &self.grid);
                }
            }
        }
        debug_assert!(assertions::hpg_update_no_affected_cells_remain(
            to_register,
            self,
        ));
    }

    pub fn deregister_hazard<'a, I>(
        &mut self,
        to_deregister: HazardEntity,
        remaining: I,
        process_now: bool,
    ) where
        I: Iterator<Item = &'a Hazard> + Clone,
    {
        if process_now {
            for cell in self.grid.cells.iter_mut().flatten() {
                cell.deregister_hazards(iter::once(to_deregister), remaining.clone());
            }
        } else {
            self.uncommitted_deregisters.push(to_deregister);
        }
    }

    pub fn flush_deregisters<'a, I>(&mut self, remaining: I)
    where
        I: Iterator<Item = &'a Hazard> + Clone,
    {
        if self.is_dirty() {
            //deregister all pending hazards at once
            let to_deregister = self.uncommitted_deregisters.iter().cloned();
            for cell in self.grid.cells.iter_mut().flatten() {
                cell.deregister_hazards(to_deregister.clone(), remaining.clone());
            }

            self.uncommitted_deregisters.clear();
        }
    }

    pub fn is_dirty(&self) -> bool {
        !self.uncommitted_deregisters.is_empty()
    }
}

/// Error type for when the `HazardProximityGrid` is in a dirty state.
/// This can happen when the grid is accessed after a hazard has been deregistered but with "process_now" set to false.
/// The grid should be flushed to ensure all changes are processed.
#[derive(Debug)]
pub struct DirtyState;

impl Display for DirtyState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dirty state detected. Make sure all changes are flushed before accessing the grid."
        )
    }
}
