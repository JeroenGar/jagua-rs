use serde::Serialize;
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use rand::SeedableRng;

use serde_wasm_bindgen::{from_value, to_value};
use rand::prelude::SmallRng;

use jagua_rs::io::import::Importer;
use jagua_rs::probs::bpp::io::ext_repr::ExtBPInstance;
use jagua_rs::probs::bpp;
use crate::config::LBFConfig;
use crate::opt::lbf_bpp::LBFOptimizerBP;
use crate::opt::lbf_bpp::now_millis;
use crate::io::output::BPOutput;
extern crate console_error_panic_hook;

pub mod config;
pub mod io;
pub mod opt;
pub mod samplers;

//limits the number of items to be placed, for debugging purposes
pub const ITEM_LIMIT: usize = usize::MAX;

#[derive(Serialize)]
struct WasmResult {
    output: BPOutput,
    svgs: Vec<(String, String)>,
    solve_time_ms: f64
}

#[wasm_bindgen]
pub fn run_bpp(
    ext_bp_instance_json: JsValue,
    input_stem: String,
) -> Result<JsValue, JsValue> {
    // Deserialize input
    console_error_panic_hook::set_once();
    let ext_instance: ExtBPInstance = from_value(ext_bp_instance_json)
        .map_err(|e| JsValue::from_str(&format!("ExtBPInstance decode error: {}", e)))?;

    let config = LBFConfig::default();

    let importer = Importer::new(
        config.cde_config,
        config.poly_simpl_tolerance,
        config.min_item_separation,
    );

    let rng = match config.prng_seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::seed_from_u64(0x12345678),
    };

    // Import instance
    let instance = bpp::io::import(&importer, &ext_instance)
        .map_err(|e| JsValue::from_str(&format!("Importer error: {}", e)))?;

    // Solve
    let start_time = now_millis();
    let sol = LBFOptimizerBP::new(instance.clone(), config, rng).solve();
    let total_time_taken = now_millis() - start_time;

    // Export solution
    let output = BPOutput {
        instance: ext_instance.clone(),
        solution: bpp::io::export(&instance, &sol, now_millis()),
        config,
    };

    // Convert snapshots to SVG strings
    let mut svgs = vec![];
    for (i, s_layout) in sol.layout_snapshots.values().enumerate() {
        let svg = jagua_rs::io::svg::s_layout_to_svg(s_layout, &instance, config.svg_draw_options, "");
        let svg_string = svg.to_string();
        svgs.push((format!("sol_{}_{}.svg", input_stem, i), svg_string));
    }


    // Wrap in serializable struct
    let result = WasmResult { output, svgs, solve_time_ms: total_time_taken };

    // Serialize and return
    to_value(&result).map_err(|e| JsValue::from_str(&format!("Result encode error: {}", e)))
}
