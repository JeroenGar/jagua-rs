pub struct EdgeIntervalIterator {
    interval: (usize, usize),
    n_points: usize,
    i: usize,
    done: bool,
}

impl EdgeIntervalIterator {
    pub fn new(interval: (usize, usize), n_points: usize) -> Self {
        let i = interval.0;
        let done = false;

        Self {
            interval,
            n_points,
            i,
            done,
        }
    }
}

impl Iterator for EdgeIntervalIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.done {
            true => {
                None
            }
            false => {
                let j = if self.i == self.n_points - 1 { 0 } else { self.i + 1 };
                if j == self.interval.1 {
                    self.done = true;
                }
                let pair = Some((self.i, j));
                self.i = j;
                pair
            }
        }
    }
}