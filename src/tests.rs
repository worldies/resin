#[cfg(test)]
mod config {
    use crate::config;
    use std::io::Write;
    use tempfile::NamedTempFile;

    pub const SAMPLE_CONFIG: &str = r#"
    {
        "name": "Very Special NFT",
        "symbol": "SNFT",
        "description": "This is the description of my NFT, it can be literally anything!",
        "externalUrl": "https://veryspecial.nft",
        "creators": [{
            "address": "BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD",
            "share": 100
        }],
        "royaltyPercentage": 10,
        "collection": {
            "name": "Special NFT: Season 1",
            "family": "Special NFTs"
        },
        "attributes": {
            "background": {
                "blue.png": 0.04,
                "brown.png": 0.04,
                "flesh.png": 0.05,
                "green.png": 0.02,
                "light-blue.png": 0.06,
                "light-green.png": 0.01,
                "light-pink.png": 0.07,
                "light-purple.png": 0.05,
                "light-yellow.png": 0.06,
                "orange.png": 0.07,
                "pink.png": 0.02,
                "purple.png": 0.03,
                "red.png": 0.05,
                "yellow.png": 0.43
            },
            "eyes": {
                "egg-eyes.png": 0.3,
                "heart-eyes.png": 0.12,
                "square-eyes.png": 0.02,
                "star-eyes.png": 0.56
            },
            "face": {
                "cyan-face.png": 0.07,
                "dark-green-face.png": 0.04,
                "flesh-face.png": 0.03,
                "gold-face.png": 0.11,
                "grapefruit-face.png": 0.07,
                "green-face.png": 0.05,
                "pink-face.png": 0.05,
                "purple-face.png": 0.02,
                "sun-face.png": 0.1,
                "teal-face.png": 0.46
            },
            "mouth": {
                "block-mouth.png": 0.23,
                "smile-mouth.png": 0.09,
                "triangle-mouth.png": 0.68
            }
        },
        "layerOrder": [
            "background",
            "face",
            "eyes",
            "mouth"
        ],
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
        assert_eq!(parsed_config.symbol, "SNFT");
        assert_eq!(
            parsed_config.description,
            "This is the description of my NFT, it can be literally anything!"
        );
        assert_eq!(parsed_config.external_url, "https://veryspecial.nft");
        assert_eq!(
            parsed_config.creators[0].address,
            "BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD"
        );
        assert_eq!(parsed_config.creators[0].share, 100);
        assert_eq!(parsed_config.royalty_percentage, 10);
        assert_eq!(parsed_config.collection.name, "Special NFT: Season 1");
        assert_eq!(parsed_config.collection.family, "Special NFTs");
        assert_eq!(parsed_config.attributes.len(), 4);
        assert_eq!(
            parsed_config.attributes.get("background").unwrap().len(),
            14
        );
        assert_eq!(
            parsed_config
                .attributes
                .get("background")
                .unwrap()
                .get("blue.png")
                .unwrap(),
            &0.04f32
        );
        assert_eq!(parsed_config.attributes.get("eyes").unwrap().len(), 4);
        assert_eq!(
            parsed_config
                .attributes
                .get("eyes")
                .unwrap()
                .get("egg-eyes.png")
                .unwrap(),
            &0.3f32
        );
        assert_eq!(parsed_config.attributes.get("face").unwrap().len(), 10);
        assert_eq!(parsed_config.attributes.get("mouth").unwrap().len(), 3);
        assert_eq!(parsed_config.layer_order.len(), 4);
        assert_eq!(
            parsed_config.layer_order,
            vec!["background", "face", "eyes", "mouth"]
        );
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
    use crate::{cmd::init, config, Init};
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
        assert_eq!(parsed_config.symbol, "SNFT");
        assert_eq!(parsed_config.description, "Hello, NFT!");
        assert_eq!(parsed_config.external_url, "https://example.com");
        assert_eq!(
            parsed_config.creators[0].address,
            "BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD"
        );
        assert_eq!(parsed_config.creators[0].share, 100);
        assert_eq!(parsed_config.royalty_percentage, 10);
        assert_eq!(parsed_config.collection.name, "NFT Collection");
        assert_eq!(parsed_config.collection.family, "NFT Family");
        assert_eq!(parsed_config.attributes.len(), 2);
        assert_eq!(parsed_config.attributes.get("LAYER_NAME").unwrap().len(), 1);
        assert_eq!(
            parsed_config
                .attributes
                .get("LAYER_NAME")
                .unwrap()
                .get("FILE_NAME.png")
                .unwrap(),
            &0.01f32
        );
        assert_eq!(
            parsed_config.attributes.get("LAYER_NAME_2").unwrap().len(),
            1
        );
        assert_eq!(
            parsed_config
                .attributes
                .get("LAYER_NAME_2")
                .unwrap()
                .get("FILE_NAME_2.png")
                .unwrap(),
            &0.01f32
        );
        assert_eq!(parsed_config.layer_order.len(), 2);
        assert_eq!(
            parsed_config.layer_order,
            vec!["LAYER_NAME", "LAYER_NAME_2"]
        );
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
        assert_eq!(parsed_config.symbol, "SNFT");
        assert_eq!(parsed_config.description, "Hello, NFT!");
        assert_eq!(parsed_config.external_url, "https://example.com");
        assert_eq!(
            parsed_config.creators[0].address,
            "BPr18DCdtzASf1YVbUVZ4dZ7mA6jpMYZSUP3YuiMgGeD"
        );
        assert_eq!(parsed_config.creators[0].share, 100);
        assert_eq!(parsed_config.royalty_percentage, 10);
        assert_eq!(parsed_config.collection.name, "NFT Collection");
        assert_eq!(parsed_config.collection.family, "NFT Family");
        assert_eq!(parsed_config.attributes.len(), 3);
        assert_eq!(
            parsed_config.attributes.get("attribute 1").unwrap().len(),
            3
        );
        assert_eq!(
            parsed_config
                .attributes
                .get("attribute 1")
                .unwrap()
                .get("file 1.png")
                .unwrap(),
            &0.1f32
        );
        assert_eq!(
            parsed_config.attributes.get("attribute 2").unwrap().len(),
            3
        );
        assert_eq!(parsed_config.layer_order.len(), dirs_to_create.len());
        assert_eq!(parsed_config.layer_order, dirs_to_create);
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
