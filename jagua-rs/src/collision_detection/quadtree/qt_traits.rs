use crate::geometry::geo_traits::CollidesWith;
use crate::geometry::primitives::Rect;
use crate::geometry::primitives::{Circle, Edge, Point};
use std::cmp::Ordering;

/// Common trait for all geometric primitives that can be directly queried in the quadtree
/// for collisions with the edges of the registered hazards. These include: [Rect], [Edge] and [Circle].
pub trait QTQueryable: CollidesWith<Edge> + CollidesWith<Rect> {
    fn collides_with_quadrants(&self, _r: &Rect, qs: [&Rect; 4]) -> [bool; 4] {
        qs.map(|q| self.collides_with(q))
    }
}

impl QTQueryable for Circle {}
impl QTQueryable for Rect {}

impl QTQueryable for Edge {
    fn collides_with_quadrants(&self, r: &Rect, qs: [&Rect; 4]) -> [bool; 4] {
        let e_x_min = self.x_min();
        let e_x_max = self.x_max();
        let e_y_min = self.y_min();
        let e_y_max = self.y_max();

        let [mut c0, mut c1, mut c2, mut c3] = [0, 1, 2, 3].map(|idx| {
            let q = qs[idx];

            let x_no_overlap = e_x_min.max(q.x_min) > e_x_max.min(q.x_max);
            let y_no_overlap = e_y_min.max(q.y_min) > e_y_max.min(q.y_max);

            if x_no_overlap || y_no_overlap {
                Some(false)
            } else if q.collides_with(&self.start) || q.collides_with(&self.end) {
                Some(true)
            } else {
                None
            }
        });

        //if none of the quadrants are indeterminate, we can return early
        if [c0,c1,c2,c3].iter().all(|c| c.is_some()) {
            return [c0.unwrap(), c1.unwrap(), c2.unwrap(), c3.unwrap()];
        }

        //  1    0
        //  2    3

        let crns = r.corners();
        let c = r.centroid();

        let side_intersect = |e: &Edge, c_s: &mut Option<bool>, c_e: &mut Option<bool>| {
            if c_s.is_none() || c_e.is_none() {
                if let Some((t, u)) = edge_intersection_custom(e, self) {
                    //dbg!(t,u);
                    match t.partial_cmp(&0.5).unwrap() {
                        Ordering::Less => {
                            *c_e = Some(true);
                        }
                        Ordering::Greater => {
                            *c_s = Some(true);
                        }
                        Ordering::Equal => {
                            *c_s = Some(true);
                            *c_e = Some(true);
                        }
                    }
                }
            }
        };

        let bisect_intersect = |e: &Edge, c_s: [&mut Option<bool>; 2], c_e: [&mut Option<bool>; 2]| {
            if c_s.iter().any(|t| t.is_none())
                || c_e.iter().any(|t| t.is_none())
            {
                if let Some((t, u)) = edge_intersection_custom(e, self) {
                    //dbg!(t,u);
                    match t.partial_cmp(&0.5).unwrap() {
                        Ordering::Less => {
                            *c_e[0] = Some(true);
                            *c_e[1] = Some(true);
                        }
                        Ordering::Greater => {
                            *c_s[0] = Some(true);
                            *c_s[1] = Some(true);
                        }
                        Ordering::Equal => {
                            *c_s[0] = Some(true);
                            *c_s[1] = Some(true);
                            *c_e[0] = Some(true);
                            *c_e[1] = Some(true);
                        }
                    }
                }
            }
        };

        let h_bisect = Edge {
            start: Point(r.x_min, c.1),
            end: Point(r.x_max, c.1),
        };
        let v_bisect = Edge {
            start: Point(c.0, r.y_min),
            end: Point(c.0, r.y_max),
        };

        let top = Edge {
            start: crns[0],
            end: crns[1],
        };
        let left = Edge {
            start: crns[1],
            end: crns[2],
        };
        let bottom = Edge {
            start: crns[2],
            end: crns[3],
        };
        let right = Edge {
            start: crns[3],
            end: crns[0],
        };

        //  1    0
        //  2    3

        bisect_intersect(&h_bisect, [&mut c1, &mut c2], [&mut c0, &mut c3]);
        bisect_intersect(&v_bisect, [&mut c2, &mut c3], [&mut c0, &mut c1]);

        side_intersect(&top, &mut c0, &mut c1);
        side_intersect(&bottom, &mut c2, &mut c3);
        side_intersect(&left, &mut c1, &mut c2);
        side_intersect(&right, &mut c3, &mut c0);

        let quadrant_collisions = [c0, c1, c2, c3].map(|c| c.unwrap_or_else(|| false));
        // assert_eq!(
        //     quadrant_collisions,
        //     qs.map(|q| self.collides_with(q)),
        //     "{:?}, {:?}, {:?}, {:?}, {:?}",
        //     [c0,c1,c2,c3],
        //     self,
        //     r,
        //     edge_intersection_custom(&left, &self),
        //     edge_intersection_custom(&h_bisect, &self)
        // );
        debug_assert!({
            let old_cs = qs.map(|q| self.collides_with(q));
            for i in 0..4 {
                if old_cs[i] && !quadrant_collisions[i] {
                    panic!(
                        "undetected quadrant collision: {:?}, {:?}, {:?}, {:?}",
                        [c0, c1, c2, c3],
                        old_cs,
                        self,
                        r
                    );
                }
            }
            true
        });

        quadrant_collisions
    }
}

#[inline(always)]
fn edge_intersection_custom(e1: &Edge, e2: &Edge) -> Option<(f32, f32)> {
    let Point(x1, y1) = e1.start;
    let Point(x2, y2) = e1.end;
    let Point(x3, y3) = e2.start;
    let Point(x4, y4) = e2.end;

    // Early exit if bounding boxes do not overlap
    // {
    //     let x_min_e1 = x1.min(x2);
    //     let x_max_e1 = x1.max(x2);
    //     let y_min_e1 = y1.min(y2);
    //     let y_max_e1 = y1.max(y2);
    //
    //     let x_min_e2 = x3.min(x4);
    //     let x_max_e2 = x3.max(x4);
    //     let y_min_e2 = y3.min(y4);
    //     let y_max_e2 = y3.max(y4);
    //
    //     let x_axis_no_overlap = x_min_e1.max(x_min_e2) > x_max_e1.min(x_max_e2);
    //     let y_axis_no_overlap = y_min_e1.max(y_min_e2) > y_max_e1.min(y_max_e2);
    //
    //     if x_axis_no_overlap || y_axis_no_overlap {
    //         return None;
    //     }
    // }

    if !edge_intersect_fast_fail(e1, e2) {
        return None; //fast fail
    }

    let Point(x1, y1) = e1.start;
    let Point(x2, y2) = e1.end;
    let Point(x3, y3) = e2.start;
    let Point(x4, y4) = e2.end;


    //based on: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
    let t_nom = (x2 - x4) * (y4 - y3) - (y2 - y4) * (x4 - x3);
    let t_denom = (x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3);
    let u_nom = (x2 - x4) * (y2 - y1) - (y2 - y4) * (x2 - x1);
    let u_denom = (x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3);
    if t_denom == 0.0 || u_denom == 0.0 {
        //parallel edges
        return None;
    }

    let t = t_nom / t_denom; //refers to the position along e1
    let u = u_nom / u_denom; //refers to the position along e2
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        return Some((t, u));
    }
    None
}

fn edge_intersect_fast_fail(e1: &Edge, e2: &Edge) -> bool {
    let Point(x1, y1) = e1.start;
    let Point(x2, y2) = e1.end;
    let Point(x3, y3) = e2.start;
    let Point(x4, y4) = e2.end;

    // Check if endpoints of e2 are on the same side of e1
    let side1 = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);
    let side2 = (x2 - x1) * (y4 - y1) - (y2 - y1) * (x4 - x1);

    if side1 * side2 > 1e-4 * f32::max(side1.abs(), side2.abs()) {
        //debug_assert!(matches!(edge_intersection_old(e1, e2, false),Intersection::No), "{side1}, {side2}");
        return false; // Both endpoints of e2 are on the same side of e1
    }

    // Check if endpoints of e1 are on the same side of e2
    let side3 = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
    let side4 = (x4 - x3) * (y2 - y3) - (y4 - y3) * (x2 - x3);

    if side3 * side4 > 1e-4 * f32::max(side3.abs(), side4.abs()) {
        //debug_assert!(matches!(edge_intersection_old(e1, e2, false),Intersection::No), "{side3}, {side4}");
        return false; // Both endpoints of e1 are on the same side of e2
    }

    true // Segments could intersect
}
