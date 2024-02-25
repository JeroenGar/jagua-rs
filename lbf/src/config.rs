use serde::{Deserialize, Serialize};

use jagua_rs::util::config::{CDEConfig, SPSurrogateConfig};
use jagua_rs::util::polygon_simplification::PolySimplConfig;

use crate::io::svg_util::SvgDrawOptions;

/// Configuration for the LBF optimizer
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Config {
    pub cde_config: CDEConfig,
    pub poly_simpl_config: PolySimplConfig,
    /// Fixes the seed for the random number generator, resulting in deterministic behavior.
    pub deterministic_mode: bool,
    /// Total number of samples per item
    pub n_samples_per_item: usize,
    /// Fraction of the samples used for the local search sampler
    pub ls_samples_fraction: f32,
    #[serde(default)]
    pub svg_draw_options: SvgDrawOptions,
}

impl Default for Config {
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
            deterministic_mode: true,
            n_samples_per_item: 5000,
            ls_samples_fraction: 0.2,
            svg_draw_options: SvgDrawOptions::default(),
        }
    }
}
