use std::collections::VecDeque;
use std::ops::RangeInclusive;

use crate::collision_detection::hpg::grid::Grid;
use crate::geometry::geo_enums::GeoPosition;
use crate::geometry::primitives::AARectangle;

// Boundary fill algorithm to propagate update through the hazard proximity grid.
// During the update of an HPGCell, it can often guarantee that the update will also not affect any of its direct neighbors.
// These "unaffected" cells can form a boundary around the update.
// By using the boundary fill algorithm, we can efficiently visit all cells that are affected by the update, without iterating over the entire grid.

// The `BoundaryFillHPG` operates in two states: "unseeded" and "seeded".
// In the "unseeded" state, we have not yet found any cell that is inside the "boundary" of the update.
// During this state, the algorithm will visit all the cells that fall within the seedbox.
// The seedbox is a rectangle that is guaranteed to contain at least one cell that is "within the boundary" of the update.

// From the moment we find a cell that is inside the boundary, we enter the "seeded" state.
// From this point on, the algorithm will only queue neighbors of cells within the boundary,

// When the queue runs out of cells to visit, all cells which could be affected should have been visited and updated.
#[derive(Debug, Clone)]
pub struct BoundaryFillHPG {
    state: Vec<CellState>,
    seedbox_rows: RangeInclusive<usize>,
    seedbox_cols: RangeInclusive<usize>,
    queue: VecDeque<usize>,
    pub n_visited: usize,
    pub seeded: bool,
}

impl BoundaryFillHPG {
    pub fn new<T>(grid: &Grid<T>, seed_bbox: &AARectangle) -> Self {
        let n_cells = grid.n_rows * grid.n_cols;
        //convert the seedbox from a rectangle to a range of rows and columns in the grid
        let seedbox_rows = grid.rows_in_range(seed_bbox.y_min..=seed_bbox.y_max);
        let seedbox_cols = grid.cols_in_range(seed_bbox.x_min..=seed_bbox.x_max);
        Self {
            state: vec![CellState::NotQueued; n_cells],
            seedbox_rows,
            seedbox_cols,
            queue: VecDeque::with_capacity(n_cells),
            n_visited: 0,
            seeded: false,
        }
        .init_queue(grid)
    }

    /// Initializes the queue with the middle cell of the seedbox
    fn init_queue<T>(mut self, grid: &Grid<T>) -> Self {
        debug_assert!(self.n_visited == 0 && self.queue.is_empty());
        let middle_row =
            self.seedbox_rows.start() + (self.seedbox_rows.end() - self.seedbox_rows.start()) / 2;
        let middle_col =
            self.seedbox_cols.start() + (self.seedbox_cols.end() - self.seedbox_cols.start()) / 2;
        let middle_cell = grid.to_index(middle_row, middle_col).unwrap();

        self.state[middle_cell].queue();
        self.queue.push_back(middle_cell);

        self
    }

    /// Returns the next cell to visit and pops it from the queue,
    /// if there are no more cells to visit return None
    pub fn pop(&mut self) -> Option<usize> {
        match self.queue.pop_front() {
            Some(cell) => {
                self.state[cell].visit(&mut self.n_visited);
                Some(cell)
            }
            None => None,
        }
    }

    /// Reports if the cell was inside or outside the boundary of the update
    pub fn report_position<T>(&mut self, index: usize, position: GeoPosition, grid: &Grid<T>) {
        let queue_neighbors = match (self.seeded, position) {
            (false, GeoPosition::Interior) => {
                //seed has been found, unqueue all cells and mark as seeded
                self.seeded = true;
                self.queue.drain(..).for_each(|i| self.state[i].dequeue());
                true
            }
            (false, GeoPosition::Exterior) => {
                //no seed found, continue queuing if the cell is within the seedbox
                let (row, col) = grid.to_row_col(index).expect("cell should exist");
                self.seedbox_rows.contains(&row) && self.seedbox_cols.contains(&col)
            }
            //seeded, only queue if the cell is within the boundary
            (true, GeoPosition::Interior) => true,
            (true, GeoPosition::Exterior) => false,
        };

        if queue_neighbors {
            for neighbor in grid.get_neighbors(index) {
                if self.state[neighbor] == CellState::NotQueued {
                    self.state[neighbor].queue();
                    self.queue.push_back(neighbor);
                }
            }
        }
    }

    /// Resets the state of the boundary fill algorithm, with a new seedbox
    pub fn reset<T>(mut self, grid: &Grid<T>, seed_bbox: &AARectangle) -> Self {
        debug_assert!(self.state.len() == grid.n_rows * grid.n_cols);

        //allows us to reuse the heap allocated state and queue vectors
        self.state
            .iter_mut()
            .for_each(|state| *state = CellState::NotQueued);
        self.queue.clear();

        Self {
            state: self.state,
            seedbox_rows: grid.rows_in_range(seed_bbox.y_min..=seed_bbox.y_max),
            seedbox_cols: grid.cols_in_range(seed_bbox.x_min..=seed_bbox.x_max),
            queue: self.queue,
            n_visited: 0,
            seeded: false,
        }
        .init_queue(grid)
    }
}

//State machine to keep track of each cell's status
#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    NotQueued,
    Visited,
    Queued,
}

impl CellState {
    fn visit(&mut self, visit_count: &mut usize) {
        *visit_count += 1;
        *self = match self {
            CellState::Queued | CellState::NotQueued => CellState::Visited,
            CellState::Visited => unreachable!("invalid state: cell already visited"),
        }
    }

    fn queue(&mut self) {
        *self = match self {
            CellState::NotQueued => CellState::Queued,
            CellState::Visited => unreachable!("invalid state: cell already visited"),
            CellState::Queued => unreachable!("invalid state: cell already queued"),
        }
    }

    fn dequeue(&mut self) {
        *self = match self {
            CellState::Queued => CellState::NotQueued,
            CellState::Visited => unreachable!("invalid state: cell already visited"),
            CellState::NotQueued => unreachable!("invalid state: cell already not queued"),
        }
    }
}
