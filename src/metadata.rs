use std::{
  fs::{
    File,
    create_dir_all
  },
  io::Write,
  path::Path
};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::config;


#[derive(Serialize, Deserialize)]
pub struct NFTMetadata {
  name: String,
  symbol: String,
  description: String,
  seller_fee_basis_points: u32,
  image: String,
  external_url: String,
  edition: u16,
  attributes: Vec<Trait>,
  properties: Properties
}

#[derive(Serialize, Deserialize)]
struct Trait {
  trait_type: String,
  value: String
}

#[derive(Serialize, Deserialize)]
struct Properties {
  files: Vec<PropertyFile>,
  category: String,
  creators: Vec<Creator>
}

#[derive(Serialize, Deserialize)]
struct PropertyFile {
  uri: String,
  r#type: String,
}

#[derive(Serialize, Deserialize)]
struct Creator {
  address: String,
  share: u8
}


pub fn generate(config_location: &String, _assets_directory: &String, output_directory: &String) {
  println!("Generating metadata...");

  let config = config::parse(config_location.as_str()).expect("Error parsing config");

  create_dir_all(output_directory).expect("Could not create output directory");

  for i in 0..5 {
    generate_stats(i, &config, output_directory);
  }
}

fn generate_stats(n: u32, config: &config::Config, output_directory: &String) {
  // do rng here, decide stats
  create_metadata(n, config, output_directory)
}

fn create_metadata(id: u32, config: &config::Config, output_directory: &String) {
  let generated_metadata = NFTMetadata {
    name: "Test NFT".to_string(),
    symbol: "TEST".to_string(),
    description: "Test NFT".to_string(),
    seller_fee_basis_points: 0,
    image: "".to_string(),
    external_url: "".to_string(),
    edition: 0,
    attributes: vec![],
    properties: Properties {
      files: vec![],
      category: "".to_string(),
      creators: vec![]
    }
  };
  write_metadata(id, &serde_json::to_string(&generated_metadata).expect("Could not serialize JSON"), output_directory)
}

fn write_metadata(id: u32, data: &str, output_directory: &String) {
  let path_buffer = Path::new(output_directory).join(format!("{}.json", id));
  // println!("Writing metadata to {}", path_buffer.display());

  let mut file = File::create(&path_buffer).expect("Could not create file");
  write!(file, "{}", data).expect("Could not write to file");
}
