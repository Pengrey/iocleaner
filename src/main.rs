use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;

use cli::Cli;
use clap::Parser;

fn main() {
    // Parse CLI
    let cli = Cli::parse();

    // Setup logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(if cli.debug {
            Level::DEBUG
        } else if cli.verbose {
            Level::INFO
        } else {
            Level::ERROR
        })
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    debug!("Parsed CLI flags");

    if let Some(project_path) = cli.project.as_deref() {
        debug!("Value for project: {}", project_path.display());
    }

    if let Some(config_path) = cli.config.as_deref() {
        debug!("Value for config: {}", config_path.display());
    }
}