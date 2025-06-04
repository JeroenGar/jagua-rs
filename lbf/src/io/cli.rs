use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input_file: PathBuf,
    #[arg(short, long, value_name = "FOLDER")]
    pub solution_folder: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    pub config_file: Option<PathBuf>,
    #[arg(
        short,
        long,
        value_name = "[off, error, warn, info, debug, trace]",
        default_value = "info"
    )]
    pub log_level: LevelFilter,
}
