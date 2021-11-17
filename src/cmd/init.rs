use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::Path,
};

use crate::Init;

const EXAMPLE_CONFIG: &str = r#"{
    "name": "NFT Title",
    "symbol": "SNFT",
    "description": "Hello, NFT!",
    "creators": ["BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD"],
    "royaltyPercentage": 10,
    "collection": {
        "name": "NFT Collection",
        "family": "NFT Family"
    },
    "rarities": {
        "LAYER_NAME": {
            "FILE_NAME.png": 0.01
        },
        "LAYER_NAME_2": {
            "FILE_NAME_2.png": 0.01
        }
    },
    "order": [
        "LAYER_NAME",
        "LAYER_NAME_2"
    ],
    "guaranteedRolls": [
        [
            "FILE_NAME.png",
            "FILE_NAME_2.png"
        ]
    ],
    "amount": 10
}"#;

pub fn handle(options: Init) {
    println!("Initializing assets directory...");

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
