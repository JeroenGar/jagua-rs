use serde::{Deserialize, Serialize};
use jagua_rs_base::collision_detection::CDEConfig;
use jagua_rs_base::geometry::fail_fast::SPSurrogateConfig;
use jagua_rs_base::io::svg::SvgDrawOptions;

/// Configuration for the LBF optimizer
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LBFConfig {
    /// Configuration of the Collision Detection Engine
    pub cde_config: CDEConfig,
    /// Max deviation from the original polygon area as a fraction. If undefined, the algorithm will run without simplification
    pub poly_simpl_tolerance: Option<f32>,
    /// Minimum distance between items and other hazards.
    /// If undefined, the algorithm will run without this constraint
    pub min_item_separation: Option<f32>,
    /// Seed for the PRNG. If undefined, the algorithm will run in non-deterministic mode using entropy
    pub prng_seed: Option<u64>,
    /// Total budget of samples per item per layout
    pub n_samples: usize,
    /// Fraction of `n_samples_per_item` used for the local search sampler, the rest is sampled uniformly.
    pub ls_frac: f32,
    /// Optional SVG drawing options
    #[serde(default)]
    pub svg_draw_options: SvgDrawOptions,
}

impl Default for LBFConfig {
    fn default() -> Self {
        Self {
            cde_config: CDEConfig {
                quadtree_depth: 5,
                item_surrogate_config: SPSurrogateConfig {
                    n_pole_limits: [(100, 0.0), (20, 0.75), (10, 0.90)],
                    n_ff_poles: 2,
                    n_ff_piers: 0,
                },
            },
            poly_simpl_tolerance: Some(0.001),
            min_item_separation: None,
            prng_seed: Some(0),
            n_samples: 5000,
            ls_frac: 0.2,
            svg_draw_options: SvgDrawOptions::default(),
        }
    }
}
