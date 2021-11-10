use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::HashMap, fs};

pub fn parse(location: &str) -> Result<Config> {
    let config_file = fs::read_to_string(location).expect("Could not read configuration file");
    let config: Config = serde_json::from_str(&config_file)?;
    Ok(config)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub creators: Vec<String>,
    pub collection: Collection,
    pub rarities: HashMap<String, HashMap<String, f32>>,
    pub order: Vec<String>,
    pub guaranteed_rolls: Vec<Vec<String>>,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Collection {
    name: String,
    family: String,
}
