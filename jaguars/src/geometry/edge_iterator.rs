use crate::geometry::primitives::edge::Edge;
use crate::geometry::primitives::simple_polygon::SimplePolygon;

#[derive(Clone, Debug)]
pub struct EdgeIterator<'a> {
    i: usize,
    shape: &'a SimplePolygon,
}

impl<'a> EdgeIterator<'a> {
    pub fn new(shape: &'a SimplePolygon) -> Self {
        Self {
            i: 0,
            shape,
        }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i < self.shape.number_of_points() {
            true => {
                let j = (self.i + 1) % self.shape.number_of_points();
                let line = self.shape.get_edge(self.i, j);

                self.i += 1;

                Some(line)
            }
            false => None
        }
    }
}