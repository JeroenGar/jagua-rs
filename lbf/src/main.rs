use std::{fs};
use std::fs::File;
use std::io::{BufReader};
use std::path::{Path};
use std::sync::Arc;

use clap::Parser as ClapParser;
use log::{info, LevelFilter, warn};
use rand::prelude::SmallRng;
use rand::SeedableRng;

use jaguars::io::parser;
use jaguars::io::parser::Parser;

use lbf::config::Config;
use lbf::{EPOCH, io};
use lbf::lbf_optimizer::LBFOptimizer;
use lbf::io::cli::Cli;
use lbf::io::json_output::JsonOutput;
use lbf::io::layout_to_svg::layout_to_svg;

fn main() {
    io::init_logger(Some(LevelFilter::Info));

    let args = Cli::parse();
    let config_file = File::open(args.config_file).expect("could not open config file");
    let config: Config = serde_json::from_reader(BufReader::new(config_file)).unwrap_or_else(|err| {
        warn!("Config file could not be parsed: {}", err);
        warn!("Falling back on default config");
        Config::default()
    });

    info!("Config: {}", serde_json::to_string(&config).unwrap());


    let json_instance = io::read_json_instance(args.input_file.as_path());
    let parser = Parser::new(config.poly_simpl_config, config.cde_config, true);
    let instance = Arc::new(parser.parse(&json_instance));

    let rng = match config.deterministic_mode {
        true => SmallRng::seed_from_u64(0),
        false => SmallRng::from_entropy(),
    };

    let mut optimizer = LBFOptimizer::new(instance.clone(), config, rng);
    let solution = optimizer.solve();

    let json_output = JsonOutput{
        instance: json_instance.clone(),
        solution: parser::compose_json_solution(&solution, &instance, EPOCH.clone()),
        config: config.clone(),
    };


    if !args.solution_folder.exists() {
        fs::create_dir_all(&args.solution_folder)
            .unwrap_or_else(|_| panic!("could not create solution folder: {:?}", args.solution_folder));
    }

    let input_file_stem = args.input_file.file_stem().unwrap().to_str().unwrap();

    let solution_path = args.solution_folder.join(format!("sol_{}.json", input_file_stem));
    io::write_json_output(&json_output, Path::new(&solution_path));

    for (i,s_layout) in solution.layout_snapshots().iter().enumerate(){
        let svg_path = args.solution_folder.join(format!("sol_{}_{}.svg", input_file_stem, i));
        io::write_svg(&layout_to_svg(s_layout, &instance, config.svg_draw_options), Path::new(&svg_path));
    }
}