use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub input_file: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    pub config_file: Option<PathBuf>,
    #[arg(short, long, value_name = "FOLDER")]
    pub solution_folder: PathBuf,
}
