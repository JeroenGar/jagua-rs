use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use jagua_rs::io::import::Importer;
use jagua_rs::io::svg::s_layout_to_svg;
use jagua_rs::probs::bpp::io::ext_repr::ExtBPInstance;
use jagua_rs::probs::bpp;
use lbf::config::LBFConfig;
use lbf::io::cli::Cli;
use lbf::io::output::BPOutput;
use lbf::io::read_bpp_instance;
use lbf::opt::lbf_bpp::LBFOptimizerBP;
use lbf::{EPOCH, io};
use log::{info, warn};
use rand::SeedableRng;
use rand::prelude::SmallRng;

//more efficient allocator
fn main() -> Result<()> {
    let args = Cli::parse();
    io::init_logger(args.log_level)?;

    let config = match args.config_file {
        None => {
            warn!("[MAIN] No config file provided, use --config-file to provide a custom config");
            LBFConfig::default()
        }
        Some(config_file) => {
            let file = File::open(config_file)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).context("incorrect config file format")?
        }
    };

    info!("Successfully parsed LBFConfig: {config:?}");

    let input_file_stem = args.input_file.file_stem().unwrap().to_str().unwrap();

    if !args.solution_folder.exists() {
        fs::create_dir_all(&args.solution_folder).unwrap_or_else(|_| {
            panic!(
                "could not create solution folder: {:?}",
                args.solution_folder
            )
        });
    }

    let ext_bp_instance = read_bpp_instance(args.input_file.as_path())?;
    main_bpp(
        ext_bp_instance,
        config,
        input_file_stem,
        args.solution_folder,
    )
}

fn main_bpp(
    ext_instance: ExtBPInstance,
    config: LBFConfig,
    input_stem: &str,
    output_folder: PathBuf,
) -> Result<()> {
    let importer = Importer::new(
        config.cde_config,
        config.poly_simpl_tolerance,
        config.min_item_separation,
    );
    let rng = match config.prng_seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::seed_from_u64(0x12345678),
    };
    let instance = bpp::io::import(&importer, &ext_instance)?;
    let sol = LBFOptimizerBP::new(instance.clone(), config, rng).solve();

    {
        let output = BPOutput {
            instance: ext_instance,
            solution: bpp::io::export(&instance, &sol, *EPOCH),
            config,
        };

        let solution_path = output_folder.join(format!("sol_{input_stem}.json"));

        io::write_json(&output, Path::new(&solution_path))?;
    }

    {
        for (i, s_layout) in sol.layout_snapshots.values().enumerate() {
            let svg_path = output_folder.join(format!("sol_{input_stem}_{i}.svg"));
            let svg = s_layout_to_svg(s_layout, &instance, config.svg_draw_options, "");

            io::write_svg(&svg, Path::new(&svg_path))?;
        }
    }

    Ok(())
}
