use crate::collision_detection::haz_prox_grid::grid::Grid;
use crate::collision_detection::haz_prox_grid::hp_cell::HPCell;
use crate::collision_detection::hazards::hazard::Hazard;

//Snapshot of the CDE state at a given time.
//Can be used to restore the CDE to a previous state.
#[derive(Clone, Debug)]
pub struct CDESnapshot {
    dynamic_hazards: Vec<Hazard>,
    grid: Grid<HPCell>,
    grid_value: f64,
}

impl CDESnapshot {
    pub fn new(dynamic_hazards: Vec<Hazard>, grid: Grid<HPCell>, grid_value: f64) -> Self {
        Self {
            dynamic_hazards,
            grid,
            grid_value,
        }
    }
    pub fn dynamic_hazards(&self) -> &Vec<Hazard> {
        &self.dynamic_hazards
    }

    pub fn grid(&self) -> &Grid<HPCell> {
        &self.grid
    }
    pub fn grid_value(&self) -> f64 {
        self.grid_value
    }
}
