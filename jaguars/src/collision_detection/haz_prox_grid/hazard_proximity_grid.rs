use std::iter;

use itertools::Itertools;

use crate::collision_detection::haz_prox_grid::{grid_generator, hp_cell};
use crate::collision_detection::haz_prox_grid::boundary_fill::BoundaryFillGrid;
use crate::collision_detection::haz_prox_grid::grid::Grid;
use crate::collision_detection::haz_prox_grid::hp_cell::{HPCell, HPCellUpdate};
use crate::collision_detection::hazards::hazard::Hazard;
use crate::collision_detection::hazards::hazard_entity::HazardEntity;
use crate::geometry::primitives::aa_rectangle::AARectangle;
use crate::geometry::geo_traits::Shape;
use crate::geometry::primitives::point::Point;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::util::config::HazProxConfig;

#[derive(Debug, Clone)]
pub struct HazardProximityGrid {
    config: HazProxConfig,
    grid: Grid<HPCell>,
    cell_radius: f64,
    value: f64,
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

        let value = grid.elements().iter().flatten().map(|d| d.value()).sum();

        HazardProximityGrid {
            config,
            grid,
            value,
            cell_radius,
            pending_deregisters: vec![],
        }
    }

    pub fn restore(&mut self, grid: Grid<HPCell>, value: f64) {
        assert_eq!(self.grid.elements().len(), grid.elements().len());
        self.grid = grid;
        self.value = value;
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
                let old_value = cell.value();
                match cell.register_hazard(to_register) {
                    HPCellUpdate::Affected => {
                        self.value += cell.value() - old_value;
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
                    .filter(|(i, res)| res == &HPCellUpdate::Affected)
                    .map(|(i, res)| i)
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
                    let old_value = cell.value();
                    let result = cell.deregister_hazards(iter::once(to_deregister), remaining.clone());
                    match result {
                        HPCellUpdate::Affected => self.value += cell.value() - old_value,
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
                let old_value = cell.value();
                cell.deregister_hazards(to_deregister.clone(), remaining.clone());
                self.value += cell.value() - old_value;
            }

            self.pending_deregisters.clear();
        }
    }

    pub fn has_pending_deregisters(&self) -> bool {
        !self.pending_deregisters.is_empty()
    }

    pub fn cells(&self) -> Result<&Vec<Option<HPCell>>, PendingChangesErr> {
        match self.has_pending_deregisters() {
            true => Err(PendingChangesErr),
            false => Ok(self.grid.elements())
        }
    }

    pub fn value(&self) -> Result<f64, PendingChangesErr> {
        match self.has_pending_deregisters() {
            true => Err(PendingChangesErr),
            false => {
                debug_assert!({
                    let recalc = self.grid.elements().iter().flatten().map(|d| d.value()).sum();
                    almost::equal(self.value, recalc)
                });
                Ok(self.value)
            }
        }
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

    pub fn value_loss(&self, shape: &SimplePolygon, upperbound: Option<f64>) -> Result<f64, PendingChangesErr> {
        debug_assert!({
            let value_loss_bf = self.value_loss_bf(shape, None)?;
            let inefficient_value_loss = self.value_loss_full(shape)?;
            let ratio = value_loss_bf / inefficient_value_loss;
            if ratio < 0.999 || ratio > 1.001 {
                panic!("full value loss calculation is not equal to boundary fill calculation: {} != {}", value_loss_bf, inefficient_value_loss);
            }
            true
        });

        self.value_loss_bf(shape, upperbound)
    }

    fn value_loss_bf(&self, shape: &SimplePolygon, upperbound: Option<f64>) -> Result<f64, PendingChangesErr> {
        //TODO: make clear in return when value_loss is exact versus just higher than upperbound

        //Calculates the loss in value of the grid if the item were to be placed
        //Unlike the full calculation, this uses a boundary fill algorithm to calculate the value loss
        //This is much faster, and should yield the same result

        //Value loss only occurs when a the new hazard is closer than the current closest
        //Furthermore, when for a cell, the new hazard is further away than the current closest hazard + cell radius,
        //All of its neighbors cannot possibly have any value loss, therefore creating a boundary
        //Once we have a full boundary around the seed point, we can stop visiting cells

        //Calculates the loss in value of the dot grid if the item were to be placed at the given transformation
        let transformed_poles = shape.surrogate().poles();
        let transformed_bounding_pole = shape.surrogate().poles_bounding_circle();
        //start off with all dots inside the bounding circle
        let seed_bbox = {
            let Point(x, y) = transformed_bounding_pole.center();
            let padding = transformed_bounding_pole.radius() + self.cell_radius();
            AARectangle::new(x - padding, y - padding, x + padding, y + padding)
        };

        let mut boundary_fill = BoundaryFillGrid::new(seed_bbox, self.grid());

        let mut estimated_value_loss = 0.0;

        while let Some(next_cell_index) = boundary_fill.pop(&self.grid) {
            let hp_cell = self.grid().elements()[next_cell_index].as_ref();
            if let Some(hp_cell) = hp_cell {
                let pole_proximity = hp_cell::distance_to_surrogate_poles_border(hp_cell, &transformed_poles);

                match hp_cell.value_loss(pole_proximity) {
                    (Some(value_loss), HPCellUpdate::Affected) => {
                        //There would be a loss in value for this cell if the item was placed here
                        estimated_value_loss += value_loss;
                        boundary_fill.queue_neighbors(next_cell_index, self.grid());
                    }
                    (None, HPCellUpdate::Unaffected) => {
                        //No value loss occurs, but neighbors might be affected
                        boundary_fill.queue_neighbors(next_cell_index, self.grid());
                    }
                    (None, HPCellUpdate::Boundary) => (),
                    _ => unreachable!()
                }
            }
            if estimated_value_loss > upperbound.unwrap_or(f64::INFINITY) {
                return Ok(estimated_value_loss);
            }
        }
        Ok(estimated_value_loss)
    }

    fn value_loss_full(&self, shape: &SimplePolygon) -> Result<f64, PendingChangesErr> {
        //Does a full recalculation of the value loss for all dots
        //This is inefficient, but is used to verify the boundary fill calculation

        let cells = self.cells()?;
        let transformed_poles = shape.surrogate().poles();

        let relevant_dots = cells.iter().flatten();

        let estimated_value_loss = relevant_dots.map(
            |cell| {
                let pole_prox = hp_cell::distance_to_surrogate_poles_border(cell, &transformed_poles);
                let (value_loss, _) = cell.value_loss(pole_prox);

                value_loss.unwrap_or(0.0)
            }
        ).sum();
        Ok(estimated_value_loss)
    }
}

#[derive(Debug)]
pub struct PendingChangesErr;
