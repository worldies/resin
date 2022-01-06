use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::config::{self, Attribute, Creator};

pub fn generate(config_location: &String, _assets_directory: &String, output_directory: &String) {
    println!("Generating metadata...");

    let config = config::parse(config_location.as_str()).expect("Error parsing config");

    create_dir_all(output_directory).expect(&format!(
        "Could not create output directory at {}",
        output_directory
    ));

    let mut guaranteed_rolls = config.guaranteed_attribute_rolls.clone();
    // How often to insert a guaranteed roll into generated rolls
    let insert_frequency = config.amount / (config.guaranteed_attribute_rolls.len() as u32 + 1);
    for i in 0..config.amount {
        if i > 0 && guaranteed_rolls.len() > 0 && i % insert_frequency == 0 {
            let roll_attributes = {
                guaranteed_rolls[0]
                    .iter()
                    .enumerate()
                    .map(|(i, t)| Trait {
                        trait_type: config.layer_order[i].clone(),
                        value: t.to_owned(),
                    })
                    .collect()
            };
            create_metadata(i, roll_attributes, &config, output_directory);
            guaranteed_rolls.remove(0);
        } else {
            generate_attributes(i, &config, output_directory);
        }
    }
}

fn generate_attributes(n: u32, config: &config::Config, output_directory: &String) {
    let mut attributes = Vec::new();
    let mut rng = thread_rng();

    for attribute_name in &config.layer_order {
        let attribute_layers = config
            .attributes
            .get(attribute_name)
            .expect(format!("Could not find attribute {} in attributes", attribute_name).as_str());

        match attribute_layers {
            Attribute::Keyed(attribute) => {
                let subattribute;
                match attribute.get("alchemistaa") {
                    Some(n) => subattribute = n,
                    None => {
                        subattribute = attribute
                            .get("_")
                            .expect("Could not find fallback attribute '_'")
                    }
                }
                let choices: Vec<&String> = subattribute.keys().collect();
                let weights: Vec<&f32> = subattribute.values().collect();

                let dist = WeightedIndex::new(weights)
                    .expect("Could not create weighted index, are any odds less than 0?");

                let result = dist.sample(&mut rng);

                // Remove file extension (.png)
                let name = choices[result]
                    .strip_suffix(".png")
                    .unwrap_or(choices[result]);

                attributes.push(Trait {
                    trait_type: attribute_name.to_string(),
                    value: name.to_string(),
                })
            }
            Attribute::Standard(attribute) => {
                println!("standard for {}", attribute_name)
            }
        }
    }

    create_metadata(n, attributes, config, output_directory)
}

fn create_metadata(
    id: u32,
    attributes: Vec<Trait>,
    config: &config::Config,
    output_directory: &String,
) {
    let image_name = &format!("{}.png", id);
    let generated_metadata = NFTMetadata {
        name: &format!("{} #{}", &config.name, id),
        symbol: &config.symbol,
        description: &config.description,
        seller_fee_basis_points: 0,
        image: image_name,
        external_url: &config.external_url,
        edition: 0,
        attributes,
        properties: Properties {
            files: vec![PropertyFile {
                uri: image_name,
                r#type: "image/png",
            }],
            category: "image",
            creators: config.creators.clone(),
        },
        collection: config.collection.clone(),
    };
    write_metadata(
        id,
        &serde_json::to_string(&generated_metadata).expect("Could not serialize generated JSON"),
        output_directory,
    )
}

fn write_metadata(id: u32, data: &str, output_directory: &String) {
    let path_buffer = Path::new(output_directory).join(format!("{}.json", id));

    let mut file = File::create(&path_buffer).expect(&format!(
        "Could not create file at path {}",
        path_buffer.display()
    ));
    write!(file, "{}", data).expect(&format!(
        "Could not write to file at path {}",
        path_buffer.display()
    ));
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
    pub attributes: Vec<Trait>,
    properties: Properties<'a>,
    collection: config::Collection,
}

#[derive(Serialize, Deserialize)]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
struct Properties<'a> {
    files: Vec<PropertyFile<'a>>,
    category: &'a str,
    creators: Vec<Creator>,
}

#[derive(Serialize, Deserialize)]
struct PropertyFile<'a> {
    uri: &'a str,
    r#type: &'a str,
}
