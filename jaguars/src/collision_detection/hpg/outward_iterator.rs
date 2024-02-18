use std::iter::Rev;
use std::ops::RangeInclusive;

///Iterator that iterates over a range from the center outwards
/// eg. 0..=10 -> 5, 6, 4, 7, 3, 8, 2, 9, 1, 10, 0
#[derive(Debug, Clone)]
pub struct OutwardIterator {
    left_iterator: Rev<RangeInclusive<usize>>,
    right_iterator: RangeInclusive<usize>,
    switch: bool,
}

impl OutwardIterator {
    pub fn new(range: RangeInclusive<usize>) -> Self {
        let median = (range.start() + range.end()) / 2;
        let left_iterator = (*range.start()..=median).rev();
        let right_iterator = median + 1..=*range.end();

        let value = Self {
            left_iterator,
            right_iterator,
            switch: false,
        };

        value
    }
}

impl Iterator for OutwardIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        //depending on the switch advance one of the iterators, if both are empty return None
        let first_option = match self.switch {
            true => self.right_iterator.next(),
            false => self.left_iterator.next(),
        };
        self.switch = !self.switch;
        match first_option {
            Some(_) => first_option,
            None => {
                match self.switch {
                    true => self.right_iterator.next(),
                    false => self.left_iterator.next(),
                }
            }
        }
    }
}