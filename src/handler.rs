use serde::Deserialize;
use std::fs;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    description: String,
    version: String,
    ioc: Vec<IoC>,
}

#[derive(Debug, Deserialize)]
pub struct IoC {
    path: String,
    ioc_type: String,
    description: String,
    ioc_value: String,
    replacement: String,
}

pub fn load_config(file_path: &Path) -> Result<Config, Box<dyn Error>> {
    let config_contents = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_contents)?;
    Ok(config)
}
