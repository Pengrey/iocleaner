use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Project file
    #[arg(short, long, value_name = "FILE")]
    pub project: PathBuf,

    /// IoC config file
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, help = "Enable debug logging")]
    pub debug: bool,

    /// Turn verbose information on
    #[arg(short, long, help = "Enable verbose logging")]
    pub verbose: bool,
}