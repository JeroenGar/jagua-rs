// The query loops rely on being inlined into their callers
#![allow(clippy::inline_always)]

use crate::collision_detection::hazards::HazKey;
use crate::geometry::primitives::Rect;

/// Bounding boxes of hazards in flat arrays sorted by `x_min`, so a query only scans the hazards
/// in its x-window and tests their boxes 64 at a time.
#[derive(Clone, Debug, Default)]
pub(crate) struct HazardIndex {
    keys: Vec<HazKey>,
    x_min: Vec<f32>,
    y_min: Vec<f32>,
    x_max: Vec<f32>,
    y_max: Vec<f32>,
    /// Upper bound on the width of the boxes
    max_width: f32,
}

impl HazardIndex {
    pub fn insert(&mut self, hkey: HazKey, bbox: Rect) {
        let i = self.x_min.partition_point(|&x| x <= bbox.x_min);
        self.keys.insert(i, hkey);
        self.x_min.insert(i, bbox.x_min);
        self.y_min.insert(i, bbox.y_min);
        self.x_max.insert(i, bbox.x_max);
        self.y_max.insert(i, bbox.y_max);
        self.max_width = self.max_width.max(bbox.width());
    }

    /// Removes the hazard with `hkey`, inserted with `bbox`.
    pub fn remove(&mut self, hkey: HazKey, bbox: Rect) {
        let start = self.x_min.partition_point(|&x| x < bbox.x_min);
        let i = start
            + self.keys[start..]
                .iter()
                .position(|&k| k == hkey)
                .expect("hazard not in index");
        self.keys.remove(i);
        self.x_min.remove(i);
        self.y_min.remove(i);
        self.x_max.remove(i);
        self.y_max.remove(i);
    }

    /// Calls `f` for every hazard whose bounding box overlaps `r` (borders included),
    /// until it returns `true`. Returns whether it did.
    #[inline(always)]
    pub fn any_overlapping(&self, r: Rect, mut f: impl FnMut(HazKey) -> bool) -> bool {
        let (lo, hi) = self.window(r);
        for start in (lo..hi).step_by(64) {
            let mut mask = self.chunk_mask(start, hi, r);
            while mask != 0 {
                let i = start + mask.trailing_zeros() as usize;
                mask &= mask - 1;
                if f(self.keys[i]) {
                    return true;
                }
            }
        }
        false
    }

    /// Like [`Self::any_overlapping`] for several rectangles, all inside `union`: the boxes are
    /// scanned once with `union`, then the candidates are tested per rectangle.
    /// `f` receives the rectangle's position in `rects`.
    #[inline(always)]
    pub fn any_overlapping_each(
        &self,
        union: Rect,
        rects: &(impl Iterator<Item = Rect> + Clone),
        mut f: impl FnMut(usize, HazKey) -> bool,
    ) -> bool {
        let (lo, hi) = self.window(union);
        for start in (lo..hi).step_by(64) {
            let candidates = self.chunk_mask(start, hi, union);
            if candidates == 0 {
                continue;
            }
            for (ri, r) in rects.clone().enumerate() {
                let mut mask = candidates;
                while mask != 0 {
                    let i = start + mask.trailing_zeros() as usize;
                    mask &= mask - 1;
                    let overlaps = self.x_min[i] <= r.x_max
                        && self.x_max[i] >= r.x_min
                        && self.y_min[i] <= r.y_max
                        && self.y_max[i] >= r.y_min;
                    if overlaps && f(ri, self.keys[i]) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// The hazards in the index with their bounding boxes, sorted by `x_min`.
    pub fn iter(&self) -> impl Iterator<Item = (HazKey, Rect)> + '_ {
        (0..self.keys.len()).map(|i| {
            let bbox = Rect {
                x_min: self.x_min[i],
                y_min: self.y_min[i],
                x_max: self.x_max[i],
                y_max: self.y_max[i],
            };
            (self.keys[i], bbox)
        })
    }

    pub fn max_width(&self) -> f32 {
        self.max_width
    }

    /// Range of hazards that can overlap `r`: those with `x_min` in `[r.x_min - max_width, r.x_max]`.
    #[inline(always)]
    fn window(&self, r: Rect) -> (usize, usize) {
        if self.x_min.len() <= 64 {
            // A single chunk: scanning it all is cheaper than searching
            return (0, self.x_min.len());
        }
        let lo = self
            .x_min
            .partition_point(|&x| x < r.x_min - self.max_width);
        let hi = self.x_min.partition_point(|&x| x <= r.x_max);
        (lo, hi.max(lo))
    }

    /// Bit `j` is set if the hazard at `start + j` overlaps `r`, for up to 64 hazards before `hi`.
    #[inline(always)]
    fn chunk_mask(&self, start: usize, hi: usize, r: Rect) -> u64 {
        let end = (start + 64).min(hi);
        rect_mask(
            &self.x_min[start..end],
            &self.y_min[start..end],
            &self.x_max[start..end],
            &self.y_max[start..end],
            r,
        )
    }
}

/// Bit `j` is set if box `j` overlaps `r` (borders included), for up to 64 boxes.
#[inline(always)]
#[allow(clippy::needless_bitwise_bool)] // branch-free
pub fn rect_mask(x_min: &[f32], y_min: &[f32], x_max: &[f32], y_max: &[f32], r: Rect) -> u64 {
    debug_assert!(x_min.len() <= 64);
    let mut mask = 0;
    for (j, (((&x0, &y0), &x1), &y1)) in x_min.iter().zip(y_min).zip(x_max).zip(y_max).enumerate() {
        let overlaps = (x0 <= r.x_max) & (x1 >= r.x_min) & (y0 <= r.y_max) & (y1 >= r.y_min);
        mask |= u64::from(overlaps) << j;
    }
    mask
}
