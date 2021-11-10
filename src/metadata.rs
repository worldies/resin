use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::config;

pub fn generate(config_location: &String, _assets_directory: &String, output_directory: &String) {
    println!("Generating metadata...");

    let config = config::parse(config_location.as_str()).expect("Error parsing config");

    create_dir_all(output_directory).expect("Could not create output directory");

    for i in 0..config.amount {
        generate_stats(i, &config, output_directory);
    }
}

fn generate_stats(n: u32, config: &config::Config, output_directory: &String) {
    // do rng here, decide stats
    create_metadata(n, config, output_directory)
}

fn create_metadata(id: u32, config: &config::Config, output_directory: &String) {
    let generated_metadata = NFTMetadata {
        name: &config.name,
        symbol: &config.symbol,
        description: &config.description,
        seller_fee_basis_points: 0,
        image: &format!("{}.png", id),
        external_url: "",
        edition: 0,
        attributes: vec![],
        properties: Properties {
            files: vec![],
            category: "image",
            creators: vec![],
        },
        collection: config.collection.clone(),
    };
    write_metadata(
        id,
        &serde_json::to_string(&generated_metadata).expect("Could not serialize JSON"),
        output_directory,
    )
}

fn write_metadata(id: u32, data: &str, output_directory: &String) {
    let path_buffer = Path::new(output_directory).join(format!("{}.json", id));
    // println!("Writing metadata to {}", path_buffer.display());

    let mut file = File::create(&path_buffer).expect("Could not create file");
    write!(file, "{}", data).expect("Could not write to file");
}

#[derive(Serialize, Deserialize)]
pub struct NFTMetadata<'a> {
    name: &'a str,
    symbol: &'a str,
    description: &'a str,
    seller_fee_basis_points: u32,
    image: &'a str,
    external_url: &'a str,
    edition: u16,
    attributes: Vec<Trait<'a>>,
    properties: Properties<'a>,
    collection: config::Collection,
}

#[derive(Serialize, Deserialize)]
struct Trait<'a> {
    trait_type: &'a str,
    value: &'a str,
}

#[derive(Serialize, Deserialize)]
struct Properties<'a> {
    files: Vec<PropertyFile<'a>>,
    category: &'a str,
    creators: Vec<Creator<'a>>,
}

#[derive(Serialize, Deserialize)]
struct PropertyFile<'a> {
    uri: &'a str,
    r#type: &'a str,
}

#[derive(Serialize, Deserialize)]
struct Creator<'a> {
    address: &'a str,
    share: u8,
}
