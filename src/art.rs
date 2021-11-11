use std::fs::{read_dir, read_to_string};

use crate::metadata::NFTMetadata;

pub fn generate(_config_location: &String, assets_location: &String, output_location: &String) {
    println!("Generating artwork from metadata...");
    read_metadata(assets_location, output_location);
}

fn read_metadata(assets_location: &String, output_location: &String) {
    let files = read_dir(assets_location).expect("Could not read assets directory");
    for file_raw in files {
        let file = file_raw.expect("Could not read file");
        if file.path().extension().unwrap() != "json" {
            continue;
        }

        let contents = read_to_string(file.path()).expect("Could not read file contents");
        let parsed_metadata: NFTMetadata =
            serde_json::from_str(&contents).expect("Could not parse metadata JSON");
        create_image(&parsed_metadata, assets_location, output_location);
    }
}

fn create_image(metadata: &NFTMetadata, assets_location: &String, output_location: &String) {
    // read layer order from attribute order
}

fn write_image() {}
