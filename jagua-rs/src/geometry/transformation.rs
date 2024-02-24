use std::borrow::Borrow;
use std::ops::{Add, Div, Mul, Sub};

use ordered_float::NotNan;

use crate::geometry::d_transformation::DTransformation;

#[derive(Clone, Debug)]
///Proper rigid transformation in matrix form
pub struct Transformation {
    matrix: [[NotNan<f64>; 3]; 3],
}

impl Transformation {
    pub const fn empty() -> Self {
        Self {
            matrix: EMPTY_MATRIX,
        }
    }

    pub fn from_rotation(angle: f64) -> Self {
        Self {
            matrix: rotation_matrix(angle),
        }
    }

    pub fn from_translation((tx, ty): (f64, f64)) -> Self {
        Self {
            matrix: translation_matrix((tx, ty)),
        }
    }

    pub fn rotate(mut self, angle: f64) -> Self {
        self.matrix = dot_prod(&rotation_matrix(angle), &self.matrix);
        self
    }

    pub fn translate(mut self, (tx, ty): (f64, f64)) -> Self {
        self.matrix = dot_prod(&translation_matrix((tx, ty)), &self.matrix);
        self
    }

    pub fn transform(mut self, other: &Self) -> Self {
        self.matrix = dot_prod(&other.matrix, &self.matrix);
        self
    }

    pub fn transform_from_decomposed(self, other: &DTransformation) -> Self {
        self.rotate(other.rotation()).translate(other.translation())
    }

    pub fn inverse(mut self) -> Self {
        self.matrix = inverse(&self.matrix);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.matrix == EMPTY_MATRIX
    }

    pub fn matrix(&self) -> &[[NotNan<f64>; 3]; 3] {
        &self.matrix
    }

    pub fn decompose(&self) -> DTransformation {
        let m = self.matrix();
        let angle = m[1][0].atan2(m[0][0].into_inner());
        let (tx, ty) = (m[0][2].into_inner(), m[1][2].into_inner());
        DTransformation::new(angle, (tx, ty))
    }
}

impl<T> From<T> for Transformation
where
    T: Borrow<DTransformation>,
{
    fn from(dt: T) -> Self {
        dt.borrow().compose()
    }
}

const _0: NotNan<f64> = unsafe { NotNan::new_unchecked(0.0) };
const _1: NotNan<f64> = unsafe { NotNan::new_unchecked(1.0) };

const EMPTY_MATRIX: [[NotNan<f64>; 3]; 3] = { [[_1, _0, _0], [_0, _1, _0], [_0, _0, _1]] };

fn rotation_matrix(angle: f64) -> [[NotNan<f64>; 3]; 3] {
    let cos = NotNan::new(angle.cos()).unwrap_or_else(|_| panic!("cos({}) is NaN", angle));
    let sin = NotNan::new(angle.sin()).unwrap_or_else(|_| panic!("sin({}) is NaN", angle));

    [[cos, -sin, _0], [sin, cos, _0], [_0, _0, _1]]
}

fn translation_matrix((tx, ty): (f64, f64)) -> [[NotNan<f64>; 3]; 3] {
    let tx = NotNan::new(tx).unwrap_or_else(|_| panic!("tx({}) is NaN", tx));
    let ty = NotNan::new(ty).unwrap_or_else(|_| panic!("ty({}) is NaN", ty));

    [[_1, _0, tx], [_0, _1, ty], [_0, _0, _1]]
}

fn dot_prod<T>(lhs: &[[T; 3]; 3], rhs: &[[T; 3]; 3]) -> [[T; 3]; 3]
where
    T: Add<Output = T> + Mul<Output = T> + Copy + Default,
{
    let mut result = [[T::default(); 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] = result[i][j] + (lhs[i][k] * rhs[k][j]);
            }
        }
    }
    result
}

fn inverse<T>(m: &[[T; 3]; 3]) -> [[T; 3]; 3]
where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    let det =
        m[0][0] * m[1][1] * m[2][2] + m[0][1] * m[1][2] * m[2][0] + m[0][2] * m[1][0] * m[2][1]
            - m[0][2] * m[1][1] * m[2][0]
            - m[0][1] * m[1][0] * m[2][2]
            - m[0][0] * m[1][2] * m[2][1];

    [
        [
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) / det,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) / det,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) / det,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) / det,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) / det,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) / det,
        ],
        [
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) / det,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) / det,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) / det,
        ],
    ]
}
