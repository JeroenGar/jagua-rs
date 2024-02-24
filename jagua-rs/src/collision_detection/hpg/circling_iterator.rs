use std::ops::RangeInclusive;

use crate::collision_detection::hpg::outward_iterator::OutwardIterator;

/// 2D version of OutwardIterator
/// Iterates over a 2D range from the center outwards
#[derive(Debug, Clone)]
pub struct CirclingIterator {
    original_col_iter: OutwardIterator,
    row_iter: OutwardIterator,
    col_iter: OutwardIterator,
    current_row: usize,
}

impl CirclingIterator {
    pub fn new(row_range: RangeInclusive<usize>, col_range: RangeInclusive<usize>) -> Self {
        let mut row_iter = OutwardIterator::new(row_range);
        let col_iter = OutwardIterator::new(col_range);

        let current_row = row_iter.next().unwrap();
        let original_col_iter = col_iter.clone();

        Self {
            original_col_iter,
            row_iter,
            col_iter,
            current_row,
        }
    }
}

impl Iterator for CirclingIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.col_iter.next() {
            Some(col) => Some((self.current_row, col)),
            None => {
                //advance row iterator
                match self.row_iter.next() {
                    Some(row) => {
                        //set current row and reset col iterator
                        self.current_row = row;
                        self.col_iter = self.original_col_iter.clone();
                        self.next()
                    }
                    None => None,
                }
            }
        }
    }
}
