use std::collections::VecDeque;

use crate::collision_detection::hpg::circling_iterator::CirclingIterator;
use crate::collision_detection::hpg::grid::Grid;
use crate::geometry::primitives::aa_rectangle::AARectangle;

///Boundary Fill type of algorithm.
///Iteratively visits cells within a grid.
///While unseeded, the struct will visit cells from the seed_bbox, from the inside out.
///When a cell's neighbors are queued, the struct is considered seeded, and will from then on visit cells from the queue.
///The "Fill" finished when there are no more cells to visit, i.e. the seed_iterator runs out (unseeded) or the queue is empty (seeded)
#[derive(Debug, Clone)]
pub struct BoundaryFillGrid {
    state: Vec<CellState>,
    seed_iterator: CirclingIterator,
    queue: VecDeque<usize>,
    n_visited: usize,
    seeded: bool,
}

impl BoundaryFillGrid {

    /// Creates a new BoundaryFillGrid, add all cells inside the seed_bbox to the queue
    pub fn new<T>(grid: &Grid<T>, seed_bbox: AARectangle) -> Self {
        let state = vec![CellState::new(); grid.n_rows * grid.n_cols];

        //Find the range of rows and columns which reside inside the seed_bbox
        let row_range = grid.rows_in_range(seed_bbox.y_min..=seed_bbox.y_max);
        let col_range = grid.cols_in_range(seed_bbox.x_min..=seed_bbox.x_max);

        //FIFO queue to keep track of which cells to visit next
        let queue = VecDeque::with_capacity(state.len());

        //Queue all cells within the seed_box, from the inside out. (seed is more likely to be in the center)
        let seed_iterator = CirclingIterator::new(row_range, col_range);

        Self {
            state,
            seed_iterator,
            queue,
            n_visited: 0,
            seeded: false,
        }
    }

    /// Returns the next cell to visit and pops it from the queue,
    /// if there are no more cells to visit return None
    pub fn pop<T>(&mut self, grid: &Grid<T>) -> Option<usize> {
        match self.seeded {
            false => {
                match self.seed_iterator.next() {
                    Some((row, col)) => {
                        let cell = grid.to_index(row, col);
                        if let Some(cell) = cell {
                            self.state[cell].visit();
                            self.n_visited += 1;
                        }
                        cell
                    }
                    None => None
                }
            }
            true => {
                match self.queue.pop_front() {
                    Some(cell) => {
                        self.state[cell].visit();
                        self.n_visited += 1;
                        Some(cell)
                    }
                    None => None
                }
            }
        }
    }

    /// Adds all unvisited neighbors of the cell at index to the queue
    /// Also marks the grid as seeded
    pub fn queue_neighbors<T>(&mut self, index: usize, grid: &Grid<T>) {
        self.seeded = true;

        //Push all not-queued neighbor cells to the queue
        for neighbor in grid.get_neighbors(index) {
            if self.state[neighbor] == CellState::NotQueued {
                self.state[neighbor].queue();
                self.queue.push_back(neighbor);
            }
        }
    }
}

//state machine to keep track of each cells status
#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    NotQueued,
    Visited,
    Queued,
}

impl CellState {
    fn new() -> Self {
        CellState::NotQueued
    }
    fn visit(&mut self) {
        *self = match self {
            CellState::Queued | CellState::NotQueued => CellState::Visited,
            CellState::Visited => unreachable!("invalid state: cell already visited")
        }
    }

    fn queue(&mut self) {
        *self = match self {
            CellState::NotQueued => CellState::Queued,
            CellState::Visited => unreachable!("invalid state: cell already visited"),
            CellState::Queued => unreachable!("invalid state: cell already queued")
        }
    }

}

