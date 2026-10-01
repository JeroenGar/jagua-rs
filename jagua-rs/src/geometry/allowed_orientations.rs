use std::f32::consts::{PI, TAU};

use anyhow::{Result, ensure};

use crate::geometry::geo_enums::RotationRange;
use crate::geometry::{DTransformation, normalize_rotation};

/// Permitted rotations without reflection and after canonical x-axis reflection.
///
/// Choose no reflection or one permitted local reflection axis, then an allowed rotation.
/// Reflecting across an axis at angle `a`, followed by rotation `r`, is represented by
/// [`DTransformation`] with `reflected = true` and rotation `r + 2*a`.
/// All angles in this type are radians. Translation is unrestricted.
#[derive(Clone, Debug, PartialEq)]
pub struct AllowedOrientations {
    rotations: RotationRange,
    rotations_after_reflection: Option<RotationRange>,
}

impl AllowedOrientations {
    /// Maximum entries per angle list and per reflected Cartesian expansion,
    /// before deduplication of the resulting angles. Shared by JSON rotation modes.
    pub const MAX_ANGLES: usize = 65_536;

    /// Validates finite angles and normalizes rotations modulo 2π and axes modulo π.
    /// Empty discrete rotations mean fixed zero; empty axes disable reflection.
    /// Duplicate canonical angles are removed by exact equality after normalization.
    pub fn new(rotations: RotationRange, mut reflection_axes: Vec<f32>) -> Result<Self> {
        ensure!(
            reflection_axes.len() <= Self::MAX_ANGLES,
            "reflection axis list exceeds the count limit"
        );
        let rotations = match rotations {
            RotationRange::Discrete(angles) => discrete_rotations(angles)?,
            RotationRange::None => RotationRange::None,
            RotationRange::Continuous => RotationRange::Continuous,
        };
        ensure!(
            reflection_axes.iter().all(|a| a.is_finite()),
            "reflection axes must be finite"
        );
        for axis in &mut reflection_axes {
            *axis = axis.rem_euclid(PI) % PI;
        }
        reflection_axes.sort_by(f32::total_cmp);
        reflection_axes.dedup();
        let rotations_after_reflection = if reflection_axes.is_empty() {
            None
        } else {
            Some(match &rotations {
                RotationRange::Continuous => RotationRange::Continuous,
                RotationRange::None => {
                    discrete_rotations(reflection_axes.iter().map(|a| 2.0 * a).collect())?
                }
                RotationRange::Discrete(angles) => {
                    ensure!(
                        angles.len() <= Self::MAX_ANGLES / reflection_axes.len(),
                        "reflected orientation expansion exceeds the count limit"
                    );
                    discrete_rotations(
                        reflection_axes
                            .iter()
                            .flat_map(|a| angles.iter().map(move |r| r + 2.0 * a))
                            .collect(),
                    )?
                }
            })
        };
        Ok(Self {
            rotations,
            rotations_after_reflection,
        })
    }

    /// Permitted rotations without reflection, in radians.
    /// [`RotationRange::None`] means fixed zero.
    #[must_use]
    pub fn rotations(&self) -> &RotationRange {
        &self.rotations
    }

    /// Permitted rotations after canonical x-axis reflection, in radians.
    /// These are transformation angles, not reflection axes.
    /// `None` disables reflection; [`RotationRange::None`] means fixed zero after reflection.
    #[must_use]
    pub fn rotations_after_reflection(&self) -> Option<&RotationRange> {
        self.rotations_after_reflection.as_ref()
    }

    /// Checks orientation only, allowing 4 f32 epsilons of a full turn for matrix
    /// decomposition and angle wrapping. This tolerance is not used to deduplicate inputs.
    #[must_use]
    pub fn allows(&self, transformation: &DTransformation) -> bool {
        let rotation = transformation.rotation();
        if !rotation.is_finite() {
            return false;
        }
        let rotation = normalize_rotation(rotation);
        let matches = |r: f32| {
            let distance = (rotation - r).abs();
            distance.min(TAU - distance) <= 4.0 * f32::EPSILON * TAU
        };
        let rotations = if transformation.reflected {
            self.rotations_after_reflection()
        } else {
            Some(self.rotations())
        };
        match rotations {
            None => false,
            Some(RotationRange::Continuous) => true,
            Some(RotationRange::None) => matches(0.0),
            Some(RotationRange::Discrete(angles)) => angles.iter().copied().any(matches),
        }
    }
}

fn discrete_rotations(mut angles: Vec<f32>) -> Result<RotationRange> {
    ensure!(
        angles.len() <= AllowedOrientations::MAX_ANGLES,
        "rotation angle list exceeds the count limit"
    );
    ensure!(
        angles.iter().all(|a| a.is_finite()),
        "rotations must be finite"
    );
    for angle in &mut angles {
        *angle = normalize_rotation(*angle);
    }
    angles.sort_by(f32::total_cmp);
    angles.dedup();
    Ok(if angles.is_empty() || angles == [0.0] {
        RotationRange::None
    } else {
        RotationRange::Discrete(angles)
    })
}
