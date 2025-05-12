mod cli;

use cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    if let Some(project_path) = cli.project.as_deref() {
        println!("Value for project: {}", project_path.display());
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
}