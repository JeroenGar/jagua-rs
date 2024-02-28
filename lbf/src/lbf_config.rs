use serde::{Deserialize, Serialize};

use jagua_rs::util::config::{CDEConfig, SPSurrogateConfig};
use jagua_rs::util::polygon_simplification::PolySimplConfig;

use crate::io::svg_util::SvgDrawOptions;

/// Configuration for the LBF optimizer
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LBFConfig {
    /// Configuration of the Collision Detection Engine
    pub cde_config: CDEConfig,
    /// Configuration of the polygon simplification in preprocessing
    pub poly_simpl_config: PolySimplConfig,
    /// Seed for the PRNG. If not defined, the algorithm will run in non-deterministic mode using entropy
    pub prng_seed: Option<u64>,
    /// Total number of samples per item
    pub n_samples_per_item: usize,
    /// Fraction of the samples used for the local search sampler
    pub ls_samples_fraction: f32,
    #[serde(default)]
    pub svg_draw_options: SvgDrawOptions,
}

impl Default for LBFConfig {
    fn default() -> Self {
        Self {
            cde_config: CDEConfig {
                quadtree_depth: 5,
                hpg_n_cells: 2000,
                item_surrogate_config: SPSurrogateConfig {
                    pole_coverage_goal: 0.9,
                    max_poles: 10,
                    n_ff_poles: 2,
                    n_ff_piers: 0,
                },
            },
            poly_simpl_config: PolySimplConfig::Enabled { tolerance: 0.001 },
            prng_seed: Some(0),
            n_samples_per_item: 5000,
            ls_samples_fraction: 0.2,
            svg_draw_options: SvgDrawOptions::default(),
        }
    }
}
