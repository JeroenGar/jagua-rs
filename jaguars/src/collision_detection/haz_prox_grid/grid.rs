use std::cmp::Ordering;
use std::ops::RangeInclusive;

use itertools::Itertools;
use ordered_float::NotNan;

//abstract representation of a grid of elements at specific coordinates
//divided into rows and columns

#[derive(Clone, Debug)]
pub struct Grid<T> {
    cells: Vec<Option<T>>,
    rows: Vec<NotNan<f64>>,
    cols: Vec<NotNan<f64>>,
    n_rows: usize,
    n_cols: usize,
}

impl<T> Grid<T> {
    pub fn new(elements: Vec<(T, (f64, f64))>) -> Self {
        //find all unique rows and columns from the element's coordinates
        let rows = elements.iter()
            .map(|(_e, (_x, y))| NotNan::new(*y).unwrap())
            .unique().sorted().collect::<Vec<NotNan<f64>>>();

        let cols = elements.iter()
            .map(|(_e, (x, _y))| NotNan::new(*x).unwrap())
            .unique().sorted().collect::<Vec<NotNan<f64>>>();

        let n_rows = rows.len();
        let n_cols = cols.len();

        //create a vector of cells, with the correct size
        let mut cells = (0..n_rows * n_cols).map(|_| None).collect_vec();

        for (element, (x, y)) in elements {
            //search correct row and col for the cell
            let row = match rows.binary_search(&NotNan::new(y).unwrap()) {
                Ok(row) => row,
                Err(_) => unreachable!()
            };
            let col = match cols.binary_search(&NotNan::new(x).unwrap()) {
                Ok(col) => col,
                Err(_) => unreachable!()
            };
            //convert to index
            let index = Self::calculate_index(row, col, n_rows, n_cols).expect("index out of bounds");
            cells[index] = Some(element);
        }

        Self {
            cells,
            rows,
            cols,
            n_rows,
            n_cols,
        }
    }

    //returns the range if row indices to completely cover the coordinate range
    pub fn rows_in_range(&self, y_range: RangeInclusive<f64>) -> RangeInclusive<usize> {
        let start_range = NotNan::new(*y_range.start()).expect("start is NaN");
        let end_range = NotNan::new(*y_range.end()).expect("end is NaN");

        let start_row = match self.rows.binary_search(&start_range) {
            Ok(start) => start,
            Err(start_insertion) => start_insertion.saturating_sub(1)
        };
        let end_row = match self.rows.binary_search(&end_range) {
            Ok(end) => end,
            Err(end_insertion) => usize::min(end_insertion, self.n_rows - 1)
        };

        start_row..=end_row
    }

    //returns the range if column indices to completely cover the coordinate range
    pub fn cols_in_range(&self, x_range: RangeInclusive<f64>) -> RangeInclusive<usize> {
        let start_range = NotNan::new(*x_range.start()).expect("start is NaN");
        let end_range = NotNan::new(*x_range.end()).expect("end is NaN");

        let start_col = match self.cols.binary_search(&start_range) {
            Ok(start) => start,
            Err(start_insertion) => start_insertion.saturating_sub(1)
        };
        let end_col = match self.cols.binary_search(&end_range) {
            Ok(end) => end,
            Err(end_insertion) => usize::min(end_insertion, self.n_cols - 1)
        };

        start_col..=end_col
    }

    pub fn get_neighbors(&self, idx: usize) -> [usize; 8] {
        //returns the indices of the 8 directly neighboring cells. If the cell is on the edge, the index of the cell itself is returned
        let mut neighbors = [0; 8];
        let (row, col) = (idx / self.n_cols, idx % self.n_cols);
        let (n_cols, n_rows) = (self.n_cols, self.n_rows);

        //ugly but fast
        neighbors[0] = if row > 0 && col > 0 { idx - n_cols - 1 } else { idx };
        neighbors[1] = if row > 0 { idx - n_cols } else { idx };
        neighbors[2] = if row > 0 && col < n_cols - 1 { idx - n_cols + 1 } else { idx };
        neighbors[3] = if col > 0 { idx - 1 } else { idx };
        neighbors[4] = if col < n_cols - 1 { idx + 1 } else { idx };
        neighbors[5] = if row < n_rows - 1 && col > 0 { idx + n_cols - 1 } else { idx };
        neighbors[6] = if row < n_rows - 1 { idx + n_cols } else { idx };
        neighbors[7] = if row < n_rows - 1 && col < n_cols - 1 { idx + n_cols + 1 } else { idx };

        neighbors
    }

    pub fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        Self::calculate_index(row, col, self.n_rows, self.n_cols)
    }

    fn calculate_index(row: usize, col: usize, n_rows: usize, n_cols: usize) -> Option<usize> {
        match (row.cmp(&n_rows), col.cmp(&n_cols)) {
            (Ordering::Less, Ordering::Less) => Some(row * n_cols + col), //out of bounds
            _ => None //out of bounds
        }
    }

    pub fn get_row_col(&self, index: usize) -> Option<(usize, usize)> {
        match index.cmp(&(self.n_rows * self.n_cols)) {
            Ordering::Less => {
                let row = index / self.n_cols;
                let col = index % self.n_cols;
                Some((row, col))
            }
            _ => None //out of bounds
        }
    }

    pub fn n_rows(&self) -> usize {
        self.n_rows
    }

    pub fn n_cols(&self) -> usize {
        self.n_cols
    }

    pub fn elements(&self) -> &Vec<Option<T>> {
        &self.cells
    }

    pub fn elements_mut(&mut self) -> &mut Vec<Option<T>> {
        &mut self.cells
    }
}