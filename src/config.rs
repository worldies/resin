use std::{
  fs
};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  name: String,
  symbol: String,
  description: String,
  creators: Vec<String>,
  collection: Collection,
  #[serde(skip)]
  rarities: String,
  order: Vec<String>,
  guaranteed_rolls: Vec<Vec<String>>,
  amount: u32,
}

#[derive(Serialize, Deserialize)]
struct Collection {
  name: String,
  family: String,
}

pub fn parse(location: &str) -> Result<Config> {
  let config_file = fs::read_to_string(location).expect("Could not read configuration file");
  let config: Config = serde_json::from_str(&config_file)?;
  Ok(config)
}
