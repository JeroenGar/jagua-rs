// The query loops rely on being inlined into their callers
#![allow(clippy::inline_always)]

use crate::collision_detection::hazards::HazKey;
use crate::geometry::primitives::Rect;

/// Bounding boxes of hazards in flat arrays sorted by `x_min`, tested 64 at a time. A query
/// stops at the first box starting right of it and skips the chunks of 64 boxes that all end left
/// of it.
#[derive(Clone, Debug, Default)]
pub(crate) struct HazardIndex {
    keys: Vec<HazKey>,
    x_min: Vec<f32>,
    y_min: Vec<f32>,
    x_max: Vec<f32>,
    y_max: Vec<f32>,
    /// Largest `x_max` of each chunk of 64 consecutive boxes
    chunk_x_max: Vec<f32>,
}

impl HazardIndex {
    pub fn insert(&mut self, hkey: HazKey, bbox: Rect) {
        let i = self.x_min.partition_point(|&x| x <= bbox.x_min);
        self.keys.insert(i, hkey);
        self.x_min.insert(i, bbox.x_min);
        self.y_min.insert(i, bbox.y_min);
        self.x_max.insert(i, bbox.x_max);
        self.y_max.insert(i, bbox.y_max);
        self.update_chunks(i / 64);
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
        self.update_chunks(i / 64);
    }

    /// Recomputes the bounds of the chunks from `first` on, whose boxes have shifted.
    fn update_chunks(&mut self, first: usize) {
        self.chunk_x_max.truncate(first);
        let maxima = self.x_max[first * 64..]
            .chunks(64)
            .map(|chunk| chunk.iter().copied().fold(f32::NEG_INFINITY, f32::max));
        self.chunk_x_max.extend(maxima);
    }

    /// Calls `f` for every hazard whose bounding box overlaps `r` (borders included),
    /// until it returns `true`. Returns whether it did.
    #[inline(always)]
    pub fn any_overlapping(&self, r: Rect, mut f: impl FnMut(HazKey) -> bool) -> bool {
        self.any_chunk(r, |start, mut mask| {
            while mask != 0 {
                let i = start + mask.trailing_zeros() as usize;
                mask &= mask - 1;
                if f(self.keys[i]) {
                    return true;
                }
            }
            false
        })
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
        self.any_chunk(union, |start, candidates| {
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
            false
        })
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

    /// Largest `x_max` of each chunk of 64 consecutive boxes.
    pub fn chunk_x_max(&self) -> &[f32] {
        &self.chunk_x_max
    }

    /// Calls `f` with the start and overlap mask of every chunk with a box overlapping `r`,
    /// until it returns `true`. Returns whether it did.
    #[inline(always)]
    fn any_chunk(&self, r: Rect, mut f: impl FnMut(usize, u64) -> bool) -> bool {
        // Boxes from `hi` on start right of `r`
        let hi = if self.x_min.len() <= 64 {
            // A single chunk: scanning it all is cheaper than searching
            self.x_min.len()
        } else {
            self.x_min.partition_point(|&x| x <= r.x_max)
        };
        for (c, &x_max) in self.chunk_x_max.iter().enumerate() {
            let start = c * 64;
            if start >= hi {
                break;
            }
            if x_max < r.x_min {
                // Every box in the chunk ends left of `r`
                continue;
            }
            let mask = self.chunk_mask(start, hi, r);
            if mask != 0 && f(start, mask) {
                return true;
            }
        }
        false
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
