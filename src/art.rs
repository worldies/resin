use std::{
    fs::{read_dir, read_to_string, File},
    io::Write,
    path::Path,
};

use crate::metadata::NFTMetadata;

pub fn generate(_config_location: &String, assets_directory: &String, output_directory: &String) {
    println!("Generating artwork from metadata...");
    read_metadata(assets_directory, output_directory);
}

fn read_metadata(assets_directory: &String, output_directory: &String) {
    let files = read_dir(assets_directory).expect("Could not read assets directory");
    for file_raw in files {
        let file = file_raw.expect("Could not read file");
        if file.path().extension().unwrap() != "json" {
            continue;
        }

        let contents = read_to_string(file.path()).expect("Could not read file contents");
        let parsed_metadata: NFTMetadata =
            serde_json::from_str(&contents).expect("Could not parse metadata JSON");
        create_image(&parsed_metadata, assets_directory, output_directory);
    }
}

fn create_image(metadata: &NFTMetadata, assets_directory: &String, output_directory: &String) {
    // read layer order from attribute order
}

fn write_image(id: u32, data: &String, output_directory: &String) {
    let path_buffer = Path::new(output_directory).join(format!("{}.png", id));

    let mut file = File::create(&path_buffer).expect("Could not create file");
    write!(file, "{}", data).expect("Could not write to file");
}
