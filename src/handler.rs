use serde::Deserialize;
use std::fs;
use std::error::Error;
use std::path::Path;
use regex::RegexBuilder;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub version: String,
    ioc: Vec<IoC>,
}

#[derive(Debug, Deserialize)]
pub struct IoC {
    path: String,
    name: String,
    description: String,
    regex: String,
    replacement: String,
}

pub fn load_config(conf_path: &Path) -> Result<Config, Box<dyn Error>> {
    let config_contents = fs::read_to_string(conf_path)?;
    let config: Config = toml::from_str(&config_contents)?;
    Ok(config)
}

pub fn check_presence(proj_path: &Path, config: &Config) -> Result<bool, Box<dyn Error>> {
    for ioc in &config.ioc {
        let regex = RegexBuilder::new(&ioc.regex)
            .multi_line(true)
            .build()
            .map_err(|e| {
                Box::<dyn Error>::from(format!(
                    "Invalid regex for IoC '{}': {}", 
                    ioc.name, 
                    e
                ))
            })?;

        let path = proj_path.join(&ioc.path);
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            if !regex.is_match(&content) {
                return Err(Box::from(format!(
                    "IoC '{}' regex '{}' not matched in file '{}'",
                    ioc.name,
                    ioc.regex,
                    path.display()
                )));
            }
        } else {
            return Err(Box::from(format!(
                "Path '{}' does not exist",
                path.display()
            )));
        }
    }
    
    Ok(true)
}