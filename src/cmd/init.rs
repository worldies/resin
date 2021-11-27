use std::{
    collections::BTreeMap,
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::Path,
};

use crate::{config::Config, Init};

const EXAMPLE_CONFIG: &str = r#"{
    "name": "NFT Title",
    "symbol": "SNFT",
    "description": "Hello, NFT!",
    "creators": [{
        "address": "BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD",
        "share": 100
    }],
    "royaltyPercentage": 10,
    "collection": {
        "name": "NFT Collection",
        "family": "NFT Family"
    },
    "attributes": {
        "LAYER_NAME": {
            "FILE_NAME.png": 0.01
        },
        "LAYER_NAME_2": {
            "FILE_NAME_2.png": 0.01
        }
    },
    "layerOrder": [
        "LAYER_NAME",
        "LAYER_NAME_2"
    ],
    "guaranteedAttributeRolls": [
        [
            "FILE_NAME.png",
            "FILE_NAME_2.png"
        ]
    ],
    "amount": 10
}"#;

pub fn handle(options: Init) {
    println!("Initializing assets directory...");

    match options.from_existing {
        Some(_) => create_from_existing(options),
        None => create_from_scratch(options),
    }
}

fn create_from_scratch(options: Init) {
    let folder_path = Path::new(&options.folder);
    if folder_path.exists() {
        if options.overwrite {
            remove_dir_all(&folder_path)
                .expect("Encountered error removing existing assets directory");
        } else {
            panic!("Folder already exists, pass --overwrite to overwrite");
        }
    }
    create_dir_all(&folder_path).expect("Encountered error creating new assets directory");

    let mut config_file = File::create(folder_path.join("config.json"))
        .expect("Encountered error creating sample file");
    write!(config_file, "{}", EXAMPLE_CONFIG).expect("Encountered error writing config file");

    create_dir_all(folder_path.join("LAYER_NAME"))
        .expect("Encountered error creating assets directory subfolder");
    File::create(folder_path.join("LAYER_NAME").join("FILE_NAME.png"))
        .expect("Encountered error creating sample file");
}

fn create_from_existing(options: Init) {
    let path = options.from_existing.unwrap();
    let folder_path = Path::new(&path);
    if !folder_path.exists() {
        panic!("Folder at path {} does not exist!", path);
    }
    if !folder_path.is_dir() {
        panic!("Path {} is not a directory!", path);
    }
    let config_path = folder_path.join("config.json");
    if config_path.exists() && !options.overwrite {
        panic!(
            "Config already exists at path {}, pass --overwrite to overwrite",
            config_path.display()
        );
    }

    let mut parsed_example_config: Config =
        serde_json::from_str(EXAMPLE_CONFIG).expect("Unable to parse example config");

    parsed_example_config.attributes = BTreeMap::new();

    for attribute in folder_path
        .read_dir()
        .expect("Encountered error reading assets directory")
    {
        let attribute = attribute.expect("Encountered error reading folder in assets directory");

        if !attribute.path().is_dir() {
            continue;
        }

        let mut attribute_map: BTreeMap<String, f32> = BTreeMap::new();

        for layer in attribute.path().read_dir().expect(&format!(
            "Encountered error reading folder in {}",
            attribute.path().display()
        )) {
            let layer = layer.expect("Encountered error reading layer in assets directory");
            let layer_path = layer.path();

            if layer_path.is_dir() {
                continue;
            }

            let layer_name = layer_path.file_name().unwrap().to_str().unwrap();
            attribute_map.insert(layer_name.to_string(), 0.1);
        }

        parsed_example_config.attributes.insert(
            attribute
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            attribute_map,
        );
    }

    let serialized_config = &serde_json::to_string(&parsed_example_config)
        .expect("Could not serialize generated config JSON");
    let mut config_file =
        File::create(config_path).expect("Encountered error creating config file");
    write!(config_file, "{}", serialized_config).expect("Encountered error writing config file");
}
