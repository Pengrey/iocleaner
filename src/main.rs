use clap::Parser;
use tracing::{debug, error, Level};
use tracing_subscriber::FmtSubscriber;
use std::error::Error;

mod cli;
mod handler;

use cli::Cli;
use handler::{load_config};


fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI
    let cli = Cli::parse();

    // Setup logger
    let max_level = match (cli.debug, cli.verbose) {
        (true, _) => Level::DEBUG,
        (_, true) => Level::INFO,
        _ => Level::ERROR,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(max_level)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    debug!("Parsed CLI flags");

    debug!("Value for project: {}", cli.project.display());

    let config = load_config(&cli.config).map_err(|e| {
        error!("Failed to parse config file: {}", e);
        e
    })?;

    debug!("Read config file for {} IoC", config.name);

    Ok(())
}