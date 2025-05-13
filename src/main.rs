use clap::Parser;
use tracing::{debug, info, error, Level};
use tracing_subscriber::FmtSubscriber;
use std::{error::Error, process::exit};

mod cli;
mod handler;

use cli::Cli;
use handler::{load_config, check_presence};


fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI
    let cli = Cli::parse();
    debug!("Parsed CLI flags");
    debug!("Value for project: {}", cli.project.display());

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


    //// Load config file with IoCs
    let config = load_config(&cli.config).map_err(|e| {
        error!("Failed to parse config file: {}", e);
        exit(1);
    })?;
    debug!("Loaded config file: [Name: {}, Version: {}, Description: {}]", config.name, config.version, config.description);

    //// Check the presence of all the IoCs in the project
    if !cli.ignore {
        match check_presence(&cli.project, &config) {
            Ok(all_present) => {
                if all_present {
                    info!("All IoCs are present.");
                }
            },
            Err(e) => {
                error!("Failed IoC presence checks: {}", e);
                exit(1);
            }
        }
    }

    info!("All operations completed successfully.");
    exit(0);
}