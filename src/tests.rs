#[cfg(test)]
mod config {
    use crate::config::{self, Attribute};
    use std::io::Write;
    use tempfile::NamedTempFile;

    pub const SAMPLE_CONFIG: &str = r#"
    {
        "name": "Very Special NFT",
        "collectionName": "Special NFTs",
        "symbol": "SNFT",
        "description": "This is the description of my NFT, it can be literally anything!",
        "externalUrl": "https://veryspecial.nft",
        "attributes": {
            "_key": {
                "joker": 0.01
            },
            "background": {
                "_": {
                    "blue.png": 0.04,
                    "green.png": 0.02,
                    "orange.png": 0.07,
                    "pink.png": 0.02,
                    "purple.png": 0.03,
                    "red.png": 0.05
                }
            },
            "face": {
                "joker": {
                    "gold-face.png": 0.11
                },
                "_": {
                    "cyan-face.png": 0.07,
                    "green-face.png": 0.05,
                    "pink-face.png": 0.05,
                    "purple-face.png": 0.02,
                    "teal-face.png": 0.46
                }
            },
            "eyes": {
                "_": {
                    "egg-eyes.png": 0.3,
                    "heart-eyes.png": 0.12,
                    "square-eyes.png": 0.02,
                    "star-eyes.png": 0.56
                }
            },
            "mouth": {
                "_key:joker": {
                    "triangle-mouth.png": 0.68
                },
                "_": {
                    "block-mouth.png": 0.23,
                    "smile-mouth.png": 0.09
                }
            }
        },
        "guaranteedAttributeRolls": [
            [
                "black.png",
                "white-face.png",
                "square-eyes.png",
                "smile-mouth.png"
            ]
        ],
        "amount": 1337
    }
    "#;

    #[test]
    fn parse() {
        let file = NamedTempFile::new().expect("Could not create temp config file");
        write!(file.as_file(), "{}", SAMPLE_CONFIG).expect("Could not write to temp config file");
        let parsed_config = config::parse(file.path().to_str().unwrap()).unwrap();

        assert_eq!(parsed_config.name, "Very Special NFT");
        assert_eq!(parsed_config.collection_name, "Special NFTs");
        assert_eq!(parsed_config.symbol, "SNFT");
        assert_eq!(
            parsed_config.description,
            "This is the description of my NFT, it can be literally anything!"
        );
        assert_eq!(parsed_config.external_url, "https://veryspecial.nft");
        assert_eq!(parsed_config.attributes.len(), 5);

        let background_attribute = parsed_config.attributes.get("background").unwrap();
        if let Attribute::Keyed(a) = background_attribute.get("_").unwrap() {
            assert_eq!(a.len(), 6);
            assert_eq!(a.get("blue.png").unwrap(), &0.04f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize background_attribute key into keyed attribute"
            )
        }

        let face_attribute = parsed_config.attributes.get("face").unwrap();
        if let Attribute::Keyed(a) = face_attribute.get("_").unwrap() {
            assert_eq!(a.len(), 5);
            assert_eq!(a.get("cyan-face.png").unwrap(), &0.07f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize face_attribute key into keyed attribute"
            )
        }
        if let Attribute::Keyed(a) = face_attribute.get("joker").unwrap() {
            assert_eq!(a.len(), 1);
            assert_eq!(a.get("gold-face.png").unwrap(), &0.11f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize face_attribute key into keyed attribute"
            )
        }

        let mouth_attribute = parsed_config.attributes.get("mouth").unwrap();
        if let Attribute::Keyed(a) = mouth_attribute.get("_").unwrap() {
            assert_eq!(a.len(), 2);
            assert_eq!(a.get("block-mouth.png").unwrap(), &0.23f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize mouth_attribute key into keyed attribute"
            )
        }
        if let Attribute::Keyed(a) = mouth_attribute.get("_key:joker").unwrap() {
            assert_eq!(a.len(), 1);
            assert_eq!(a.get("triangle-mouth.png").unwrap(), &0.68f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize mouth_attribute key into keyed attribute"
            )
        }

        assert_eq!(parsed_config.guaranteed_attribute_rolls.len(), 1);
        assert_eq!(parsed_config.guaranteed_attribute_rolls[0].len(), 4);
        assert_eq!(
            parsed_config.guaranteed_attribute_rolls[0],
            vec![
                "black.png",
                "white-face.png",
                "square-eyes.png",
                "smile-mouth.png"
            ]
        );
        assert_eq!(parsed_config.amount, 1337);
    }

    #[test]
    #[should_panic]
    fn invalid_path() {
        config::parse("/path/to/nowhere").unwrap();
    }

    #[test]
    #[should_panic]
    fn corrupted_file() {
        let file = NamedTempFile::new().expect("Could not create temp config file");
        write!(file.as_file(), "invalid json").unwrap();

        let config_path = file.path();
        config::parse(config_path.to_str().unwrap()).unwrap();
    }
}

#[cfg(test)]
mod metadata {
    #[allow(unused_imports)]
    use crate::metadata;

    #[test]
    fn integration() {
        assert!(true);
    }

    #[test]
    fn attribute_generation() {
        assert!(true);
    }

    #[test]
    fn creation() {
        assert!(true);
    }
}

#[cfg(test)]
mod art {
    #[allow(unused_imports)]
    use crate::art;

    #[test]
    fn read_metadata() {
        assert!(true);
    }

    #[test]
    fn creation() {
        assert!(true);
    }
}

#[cfg(test)]
mod init {
    use crate::{
        cmd::init,
        config::{self, Attribute},
        Init,
    };
    use std::fs::{create_dir_all, File};
    use tempfile::tempdir;

    #[test]
    fn from_scratch() {
        let dir = tempdir().unwrap().path().join("assets");
        let command_input = Init {
            folder: dir.to_str().unwrap().to_string(),
            overwrite: false,
            from_existing: None,
        };
        init::handle(command_input);

        let parsed_config = config::parse(dir.join("config.json").to_str().unwrap()).unwrap();

        assert_eq!(parsed_config.name, "NFT Title");
        assert_eq!(parsed_config.collection_name, "NFT Collection");
        assert_eq!(parsed_config.symbol, "SNFT");
        assert_eq!(parsed_config.description, "Hello, NFT!");
        assert_eq!(parsed_config.external_url, "https://example.com");
        assert_eq!(parsed_config.attributes.len(), 3);

        let layer_1 = parsed_config.attributes.get("LAYER_NAME").unwrap();
        if let Attribute::Keyed(a) = layer_1.get("_").unwrap() {
            assert_eq!(a.len(), 1);
            assert_eq!(a.get("FILE_NAME.png").unwrap(), &0.01f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize LAYER_NAME key into keyed attribute"
            )
        }

        let layer_2 = parsed_config.attributes.get("LAYER_NAME_2").unwrap();
        if let Attribute::Keyed(a) = layer_2.get("_").unwrap() {
            assert_eq!(a.len(), 1);
            assert_eq!(a.get("FILE_NAME_2.png").unwrap(), &0.01f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize LAYER_NAME_2 key into keyed attribute"
            )
        }
        if let Attribute::Keyed(a) = layer_2.get("KEY").unwrap() {
            assert_eq!(a.len(), 1);
            assert_eq!(a.get("FILE_NAME_3.png").unwrap(), &0.01f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize LAYER_NAME_2 key into keyed attribute"
            )
        }

        assert_eq!(parsed_config.guaranteed_attribute_rolls.len(), 1);
        assert_eq!(parsed_config.guaranteed_attribute_rolls[0].len(), 2);
        assert_eq!(
            parsed_config.guaranteed_attribute_rolls[0],
            vec!["FILE_NAME.png", "FILE_NAME_2.png"]
        );
        assert_eq!(parsed_config.amount, 10);
    }

    #[test]
    fn from_existing() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        let dirs_to_create = vec!["attribute 1", "attribute 2", "attribute 3"];
        for dir in &dirs_to_create {
            create_dir_all(dir_path.join(dir)).unwrap();
        }

        let files_to_create = vec!["file 1.png", "file 2.png", "file 3.png"];
        for file in files_to_create {
            for dir in &dirs_to_create {
                File::create(dir_path.join(dir).join(file)).unwrap();
            }
        }

        let command_input = Init {
            folder: "".to_string(),
            overwrite: false,
            from_existing: dir_path.to_str().map(str::to_string),
        };
        init::handle(command_input);

        let parsed_config = config::parse(dir_path.join("config.json").to_str().unwrap()).unwrap();

        assert_eq!(parsed_config.name, "NFT Title");
        assert_eq!(parsed_config.collection_name, "NFT Collection");
        assert_eq!(parsed_config.symbol, "SNFT");
        assert_eq!(parsed_config.description, "Hello, NFT!");
        assert_eq!(parsed_config.external_url, "https://example.com");
        assert_eq!(parsed_config.attributes.len(), 3);

        let layer_1 = parsed_config.attributes.get("attribute 1").unwrap();
        if let Attribute::Keyed(a) = layer_1.get("_").unwrap() {
            assert_eq!(a.len(), 3);
            assert_eq!(a.get("file 1.png").unwrap(), &0.1f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize attribute 1 key into keyed attribute"
            )
        }

        let layer_2 = parsed_config.attributes.get("attribute 2").unwrap();
        if let Attribute::Keyed(a) = layer_2.get("_").unwrap() {
            assert_eq!(a.len(), 3);
            assert_eq!(a.get("file 2.png").unwrap(), &0.1f32);
        } else {
            assert!(
                false,
                "wasn't able to deserialize attribute 2 key into keyed attribute"
            )
        }

        assert_eq!(parsed_config.guaranteed_attribute_rolls.len(), 0);
        assert_eq!(parsed_config.amount, 10);
    }

    #[test]
    #[should_panic]
    fn directory_already_exists() {
        let dir = tempdir().unwrap();
        let command_input = Init {
            folder: dir.path().to_str().unwrap().to_string(),
            overwrite: false,
            from_existing: None,
        };
        init::handle(command_input);
    }
}
