use crate::collision_detection::haz_prox_grid::grid::Grid;
use crate::collision_detection::haz_prox_grid::hpg_cell::HPGCell;
use crate::collision_detection::hazards::hazard::Hazard;

//Snapshot of the CDE state at a given time.
//Can be used to restore the CDE to a previous state.
#[derive(Clone, Debug)]
pub struct CDESnapshot {
    dynamic_hazards: Vec<Hazard>,
    grid: Grid<HPGCell>
}

impl CDESnapshot {
    pub fn new(dynamic_hazards: Vec<Hazard>, grid: Grid<HPGCell>) -> Self {
        Self {
            dynamic_hazards,
            grid
        }
    }
    pub fn dynamic_hazards(&self) -> &Vec<Hazard> {
        &self.dynamic_hazards
    }

    pub fn grid(&self) -> &Grid<HPGCell> {
        &self.grid
    }

}
