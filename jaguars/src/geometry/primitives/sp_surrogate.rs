use std::ops::Range;

use crate::fail_fast::{clips, poi};
use crate::geometry::primitives::circle::Circle;
use crate::geometry::convex_hull;
use crate::geometry::primitives::edge::Edge;
use crate::geometry::geo_traits::{Transformable, TransformableFrom};
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;

#[derive(Clone, Debug)]
pub struct SPSurrogate {
    config: SPSurrogateConfig,
    convex_hull_indices: Vec<usize>,
    poles: Vec<Circle>,
    clips: Vec<Edge>,
    poles_bounding_circle: Circle,
}

impl SPSurrogate {
    pub fn new(simple_poly: &SimplePolygon, config: SPSurrogateConfig) -> Self {
        let convex_hull_indices = convex_hull::convex_hull_indices(simple_poly);
        let poles = poi::generate_poles(simple_poly, config.max_poles, config.min_poles, config.coverage_goal);
        let pole_of_inaccessibility = poles.get(0).expect("no poles generated").clone();
        let poles_bounding_circle = pole_of_inaccessibility.bounding_circle(&poles[1..]);

        let relevant_poles_for_clips_slice = &poles[0..config.ff_range_poles().end];
        let clips = clips::generate(simple_poly, config.n_clips, relevant_poles_for_clips_slice);

        Self {
            config,
            convex_hull_indices,
            poles,
            clips,
            poles_bounding_circle,
        }
    }

    pub fn convex_hull_indices(&self) -> &Vec<usize> {
        &self.convex_hull_indices
    }

    pub fn clips(&self) -> &Vec<Edge> {
        &self.clips
    }

    pub fn pole_of_inaccessibility(&self) -> &Circle {
        &self.poles[0]
    }

    pub fn poles(&self) -> &[Circle] {
        &self.poles
    }

    pub fn other_poles(&self) -> &[Circle] {
        &self.poles[1..]
    }

    pub fn poles_bounding_circle(&self) -> &Circle {
        &self.poles_bounding_circle
    }

    pub fn config(&self) -> SPSurrogateConfig {
        self.config
    }
}

impl Transformable for SPSurrogate {
    fn transform(&mut self, t: &Transformation) -> &mut Self {
        //destructuring pattern used to ensure that the code is updated when the struct changes
        let Self { config: _, convex_hull_indices: _, poles, clips, poles_bounding_circle } = self;

        //transform poles
        poles.iter_mut().for_each(|c| {
            c.transform(t);
        });

        poles_bounding_circle.transform(t);

        //transform clipped lines
        clips.iter_mut().for_each(|l| {
            l.transform(t);
        });

        self
    }
}

impl TransformableFrom for SPSurrogate {
    fn transform_from(&mut self, reference: &Self, t: &Transformation) -> &mut Self {
        debug_assert!(self.config == reference.config);
        debug_assert!(self.other_poles().len() == reference.other_poles().len());
        debug_assert!(self.clips().len() == reference.clips().len());

        //destructuring pattern used to ensure that the code is updated when the struct changes
        let Self { config: _, convex_hull_indices: _, poles, clips, poles_bounding_circle } = self;

        for (pole, ref_pole) in poles.iter_mut().zip(reference.poles.iter()) {
            pole.transform_from(ref_pole, t);
        }

        poles_bounding_circle.transform_from(&reference.poles_bounding_circle, t);

        for (clip, ref_clip) in clips.iter_mut().zip(reference.clips.iter()) {
            clip.transform_from(ref_clip, t);
        }

        self
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct SPSurrogateConfig {
    pub max_poles: usize,
    pub min_poles: usize,
    pub coverage_goal: f64,
    pub n_clips: usize,
    pub n_ff_poles: usize,
    //number of poles to test during fail fast
    pub n_ff_clips: usize, //number of clips to test during fail fast
}

impl Default for SPSurrogateConfig {
    fn default() -> Self {
        Self {
            max_poles: 1,
            min_poles: 1, //one pole required
            coverage_goal: 0.0,
            n_clips: 0,
            n_ff_poles: 0,
            n_ff_clips: 0,
        }
    }
}


impl SPSurrogateConfig {
    pub fn item_default() -> Self {
        Self {
            max_poles: 10,
            min_poles: 5,
            coverage_goal: 0.9,
            n_clips: 1,
            n_ff_poles: 1,
            n_ff_clips: 1,
        }
    }

    pub fn ff_range_poles(&self) -> Range<usize> {
        //first pole is skipped since it should be covered by Dot Grid
        1..1 + self.n_ff_poles
    }

    pub fn ff_range_clips(&self) -> Range<usize> {
        0..self.n_ff_clips
    }
}