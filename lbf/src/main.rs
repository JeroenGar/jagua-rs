use std::any::Any;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::Parser as ClapParser;
use jagua_rs::entities::instances::bin_packing::BPInstance;
use jagua_rs::entities::instances::strip_packing::SPInstance;
use jagua_rs::io::parser;
use jagua_rs::io::parser::Parser;
use jagua_rs::util::polygon_simplification::PolySimplConfig;
use lbf::io::cli::Cli;
use lbf::io::json_output::JsonOutput;
use lbf::io::layout_to_svg::s_layout_to_svg;
use lbf::lbf_config::LBFConfig;
use lbf::lbf_optimizer::LBFOptimizer;
use lbf::{EPOCH, LBFInstance, LBFSolution, io};
use log::{error, warn};
use mimalloc::MiMalloc;
use rand::SeedableRng;
use rand::prelude::SmallRng;

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
    let instance = {
        let instance = parser.parse(&json_instance);
        let any = instance.as_ref() as &dyn Any;
        if let Some(spi) = any.downcast_ref::<SPInstance>() {
            LBFInstance::SP(spi.clone())
        } else if let Some(bpi) = any.downcast_ref::<BPInstance>() {
            LBFInstance::BP(bpi.clone())
        } else {
            panic!("unsupported instance type");
        }
    };

    let input_file_stem = args.input_file.file_stem().unwrap().to_str().unwrap();

    let solution_path = args
        .solution_folder
        .join(format!("sol_{}.json", input_file_stem));

    if !args.solution_folder.exists() {
        fs::create_dir_all(&args.solution_folder).unwrap_or_else(|_| {
            panic!(
                "could not create solution folder: {:?}",
                args.solution_folder
            )
        });
    }

    let rng = match config.prng_seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_os_rng(),
    };

    let solution = match &instance {
        LBFInstance::SP(sp) => {
            let mut optimizer = LBFOptimizer::from_sp_instance(sp.clone(), config, rng);
            let sol = optimizer.solve();
            LBFSolution::SP(sol)
        }
        LBFInstance::BP(bp) => {
            let mut optimizer = LBFOptimizer::from_bp_instance(bp.clone(), config, rng);
            let sol = optimizer.solve();
            LBFSolution::BP(sol)
        }
    };

    //output
    match (&instance, &solution) {
        (LBFInstance::SP(spi), LBFSolution::SP(sol)) => {
            let json_sol = parser::compose_json_solution_spp(&sol, spi, *EPOCH);
            let json_output = JsonOutput {
                instance: json_instance.clone(),
                solution: json_sol,
                config,
            };
            io::write_json_output(&json_output, Path::new(&solution_path));

            let svg_path = args
                .solution_folder
                .join(format!("sol_{}.svg", input_file_stem));

            io::write_svg(
                &s_layout_to_svg(&sol.layout_snapshot, spi, config.svg_draw_options),
                Path::new(&svg_path),
            );
        }
        (LBFInstance::BP(bpi), LBFSolution::BP(sol)) => {
            let json_sol = parser::compose_json_solution_bpp(&sol, bpi, *EPOCH);
            let json_output = JsonOutput {
                instance: json_instance.clone(),
                solution: json_sol,
                config,
            };

            io::write_json_output(&json_output, Path::new(&solution_path));

            for (i, (_, s_layout)) in sol.layout_snapshots.iter().enumerate() {
                let svg_path = args
                    .solution_folder
                    .join(format!("sol_{}_{}.svg", input_file_stem, i));
                io::write_svg(
                    &s_layout_to_svg(s_layout, bpi, config.svg_draw_options),
                    Path::new(&svg_path),
                );
            }
        }
        _ => panic!("solution and instance types do not match"),
    };
}
