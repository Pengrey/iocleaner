use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Project file
    #[arg(short, long, value_name = "FILE")]
    pub project: Option<PathBuf>,

    /// IoC config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short = 'd', long = "debug", help = "Enable debug logging")]
    pub debug: bool,

    /// Turn verbose information on
    #[arg(short = 'v', long = "verbose", help = "Enable verbose logging")]
    pub verbose: bool,
}