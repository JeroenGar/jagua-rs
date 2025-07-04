#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use rand::SeedableRng;

use serde_wasm_bindgen::{from_value, to_value};
use rand::prelude::SmallRng;
use serde::Serialize;
use console_error_panic_hook;
use jagua_rs::io::import::Importer;
use jagua_rs::probs::bpp::io::ext_repr::ExtBPInstance;
use jagua_rs::probs::bpp;
use crate::config::LBFConfig;
use crate::opt::lbf_bpp::LBFOptimizerBP;
use crate::io::output::BPOutput;
use crate::time::TimeStamp;
use crate::time::now_millis;
use crate::init_logger;
use log::{warn, info};

#[derive(Serialize)]
struct WasmResult {
    output: BPOutput,
    svgs: Vec<(String, String)>,
    solve_time_ms: f64
}

#[wasm_bindgen]
pub fn init_logger_wasm() {
    init_logger();
    info!("Logger initialized for WASM!!");
}

#[wasm_bindgen]
pub fn run_bpp(
    ext_bp_instance_json: JsValue,
    config_json: JsValue,
) -> Result<JsValue, JsValue> {
    // Deserialize input
    console_error_panic_hook::set_once();
    let ext_instance: ExtBPInstance = from_value(ext_bp_instance_json)
        .map_err(|e| JsValue::from_str(&format!("ExtBPInstance decode error: {}", e)))?;

    let config: LBFConfig = from_value(config_json).unwrap_or_else(|e| {
        warn!("Invalid config, using default. Reason: {}", e);
        LBFConfig::default()
    });

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
    let start_time = TimeStamp::now();
    let sol = LBFOptimizerBP::new(instance.clone(), config, rng).solve();
    let total_time_taken = start_time.elapsed_ms();

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
        svgs.push((format!("sol_input_{}.svg", i), svg_string));
    }


    // Wrap in serializable struct
    let result = WasmResult { output, svgs, solve_time_ms: total_time_taken };

    // Serialize and return
    to_value(&result).map_err(|e| JsValue::from_str(&format!("Result encode error: {}", e)))
}
