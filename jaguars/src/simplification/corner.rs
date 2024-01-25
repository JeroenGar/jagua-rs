use crate::geometry::primitives::point::Point;

//Corner is defined as the left hand side of the line i_prev-i-i_next
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corner {
    i_prev: usize,
    i: usize,
    i_next: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CornerType {
    Concave,
    Convex,
    Collinear,
}

impl Corner {
    pub fn new(i_prev: usize, i: usize, i_next: usize) -> Self {
        Self {
            i_prev,
            i,
            i_next,
        }
    }

    pub fn flip(&mut self) {
        std::mem::swap(&mut self.i_prev, &mut self.i_next);
    }

    pub fn determine_type(&self, points: &[Point]) -> CornerType {
        //returns the corner type on the left-hand side p1-p2-p3

        //From: https://algorithmtutor.com/Computational-Geometry/Determining-if-two-consecutive-segments-turn-left-or-right/

        let (p1, p2, p3) = (points[self.i_prev], points[self.i], points[self.i_next]);

        let p1p2 = (p2.0 - p1.0, p2.1 - p1.1);
        let p1p3 = (p3.0 - p1.0, p3.1 - p1.1);

        //if the vector from p1 to p3 is more to the left then p1 to p2, their cross product will be positive

        let cross_prod = p1p2.0 * p1p3.1 - p1p2.1 * p1p3.0;

        //if p2p3 turns to the left with respect to p1p2, the cross product is positive
        //Thus a positive cross product means a convex corner, while a negative cross product means a concave corner

        match cross_prod {
            x if x < 0.0 => CornerType::Concave,
            x if x == 0.0 => CornerType::Collinear,
            x if x > 0.0 => CornerType::Convex,
            _ => panic!("Cross product is NaN"),
        }
    }


    pub fn i_prev(&self) -> usize {
        self.i_prev
    }
    pub fn i(&self) -> usize {
        self.i
    }
    pub fn i_next(&self) -> usize {
        self.i_next
    }
}