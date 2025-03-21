use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::Parser as ClapParser;
use log::{error, warn};
use mimalloc::MiMalloc;
use rand::SeedableRng;
use rand::prelude::SmallRng;

use jagua_rs::io::parser;
use jagua_rs::io::parser::Parser;
use jagua_rs::util::polygon_simplification::PolySimplConfig;
use lbf::io::cli::Cli;
use lbf::io::json_output::JsonOutput;
use lbf::io::layout_to_svg::s_layout_to_svg;
use lbf::lbf_config::LBFConfig;
use lbf::lbf_optimizer::LBFOptimizer;
use lbf::{EPOCH, io};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

//more efficient allocator
fn main() {
    let args = Cli::parse();
    io::init_logger(args.log_level);

    let config = match args.config_file {
        None => {
            warn!("No config file provided, use --config-file to provide a custom config");
            warn!(
                "Falling back default config:\n{}",
                serde_json::to_string(&LBFConfig::default()).unwrap()
            );
            LBFConfig::default()
        }
        Some(config_file) => {
            let file = File::open(config_file).unwrap_or_else(|err| {
                panic!("Config file could not be opened: {}", err);
            });
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|err| {
                error!("Config file could not be parsed: {}", err);
                error!("Omit the --config-file argument to use the default config");
                panic!();
            })
        }
    };

    let json_instance = io::read_json_instance(args.input_file.as_path());
    let poly_simpl_config = match config.poly_simpl_tolerance {
        Some(tolerance) => PolySimplConfig::Enabled { tolerance },
        None => PolySimplConfig::Disabled,
    };

    let parser = Parser::new(poly_simpl_config, config.cde_config, true);
    let instance = parser.parse(&json_instance);

    let rng = match config.prng_seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_os_rng(),
    };

    let mut optimizer = LBFOptimizer::new(instance.clone(), config, rng);
    let solution = optimizer.solve();

    let json_output = JsonOutput {
        instance: json_instance.clone(),
        solution: parser::compose_json_solution(&solution, &instance, *EPOCH),
        config,
    };

    if !args.solution_folder.exists() {
        fs::create_dir_all(&args.solution_folder).unwrap_or_else(|_| {
            panic!(
                "could not create solution folder: {:?}",
                args.solution_folder
            )
        });
    }

    let input_file_stem = args.input_file.file_stem().unwrap().to_str().unwrap();

    let solution_path = args
        .solution_folder
        .join(format!("sol_{}.json", input_file_stem));
    io::write_json_output(&json_output, Path::new(&solution_path));

    for (i, s_layout) in solution.layout_snapshots.iter().enumerate() {
        let svg_path = args
            .solution_folder
            .join(format!("sol_{}_{}.svg", input_file_stem, i));
        io::write_svg(
            &s_layout_to_svg(s_layout, &instance, config.svg_draw_options),
            Path::new(&svg_path),
        );
    }
}
