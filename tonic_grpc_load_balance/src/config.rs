use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Config{
    pub endpoints: Vec<String>,
}

impl Config {
    pub fn new(file:&str) -> anyhow::Result<Config>{
        let content =std::fs::read_to_string(file)?;
        let config:Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}