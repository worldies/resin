use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::BTreeMap, fs};

pub fn parse(location: &str) -> Result<Config> {
    let config_file = fs::read_to_string(location).expect("Could not read configuration file");
    let config: Config = serde_json::from_str(&config_file)?;
    Ok(config)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub name: String,
    pub collection_name: String,
    pub symbol: String,
    pub description: String,
    pub external_url: String,
    pub attributes: IndexMap<String, BTreeMap<String, Attribute>>,
    pub guaranteed_attribute_rolls: Vec<Vec<String>>,
    pub amount: u32,
    pub require_unique: Option<bool>,
    pub max_retries: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attribute {
    Keyed(IndexMap<String, f32>),
    Standard(f32),
}
