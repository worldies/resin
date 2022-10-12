use indexmap::IndexMap;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::config::{self, Attribute, Creator};

pub fn generate(config_location: &String, _assets_directory: &String, output_directory: &String) {
    println!("Generating metadata...");

    let config = config::parse(config_location.as_str()).expect("Error parsing config");

    create_dir_all(Path::new(output_directory).join(".resin")).expect(&format!(
        "Could not create output directory at {}",
        output_directory
    ));

    // Collection of generated rolls used if `require_unique` is enabled
    let mut generated_rolls: Vec<Vec<Trait>> = Vec::new();

    let mut guaranteed_rolls = config.guaranteed_attribute_rolls.clone();
    let attribute_names: Vec<&String> = config.attributes.keys().collect();
    // How often to insert a guaranteed roll into generated rolls
    let insert_frequency = config.amount / (guaranteed_rolls.len() as u32 + 1);
    for i in 0..config.amount {
        if i > 0 && guaranteed_rolls.len() > 0 && i % insert_frequency == 0 {
            let roll_attributes = {
                guaranteed_rolls[0]
                    .iter()
                    .enumerate()
                    .map(|(i, t)| Trait {
                        trait_type: attribute_names[i].clone(),
                        value: t.to_string(),
                    })
                    .collect()
            };
            create_metadata(i, roll_attributes, &config, output_directory);
            guaranteed_rolls.remove(0);
        } else {
            generate_attributes(i, &config, output_directory, &mut generated_rolls, None);
        }
    }
}

fn generate_attributes(
    n: u32,
    config: &config::Config,
    output_directory: &String,
    generated_rolls: &mut Vec<Vec<Trait>>,
    retries: Option<u32>,
) {
    let retries = retries.unwrap_or_default();
    let mut attributes = Vec::new();
    let mut rng = thread_rng();

    for (attribute_name, keys) in &config.attributes {
        let mut subattribute: IndexMap<String, f32> = IndexMap::new();

        for (raw_key, a) in keys {
            match a {
                Attribute::Keyed(a) => {
                    if raw_key == "_" {
                        continue;
                    }

                    let mut good_match = raw_key.split("&").all(|k| {
                        let (key, value) = k.trim().split_once(":").unwrap_or(("_key", k));

                        attributes.iter().any(|t: &Trait| {
                            t.trait_type == key && stylize_asset_name(&t.value) == value
                        })
                    });

                    if !good_match {
                        good_match = raw_key.split("|").any(|k| {
                            let (key, value) = k.trim().split_once(":").unwrap_or(("_key", k));

                            attributes.iter().any(|t: &Trait| {
                                t.trait_type == key && stylize_asset_name(&t.value) == value
                            })
                        });
                    }

                    if good_match {
                        subattribute = a.clone();
                        break;
                    }
                }
                Attribute::Standard(_) => continue,
            }
        }

        if subattribute.is_empty() {
            for (k, a) in keys {
                match a {
                    Attribute::Keyed(_) => continue,
                    Attribute::Standard(v) => subattribute.insert(k.to_string(), *v),
                };
            }
        }

        calculate_rng_for_attribute(attribute_name, &subattribute, &mut attributes, &mut rng);
    }

    if config.require_unique.unwrap_or_default() && generated_rolls.contains(&attributes) {
        if retries > config.max_retries.unwrap_or(64) {
            panic!(
                "Exceeded retry count to ensure uniqueness. Your config may need more attributes."
            )
        }
        // If it already exists, re-roll
        return generate_attributes(
            n,
            config,
            output_directory,
            generated_rolls,
            Some(retries + 1),
        );
    }

    generated_rolls.push(attributes.clone());

    create_metadata(n, attributes, config, output_directory)
}

fn calculate_rng_for_attribute(
    attribute_name: &String,
    attribute: &IndexMap<String, f32>,
    attributes: &mut Vec<Trait>,
    rng: &mut ThreadRng,
) {
    let choices: Vec<&String> = attribute.keys().collect();
    let weights: Vec<&f32> = attribute.values().collect();

    let dist = WeightedIndex::new(weights)
        .expect("Could not create weighted index, are any odds less than 0?");

    let result = dist.sample(rng);

    attributes.push(Trait {
        trait_type: attribute_name.to_string(),
        value: choices[result].to_string(),
    });
}

fn create_metadata(
    id: u32,
    attributes: Vec<Trait>,
    config: &config::Config,
    output_directory: &String,
) {
    let resin_metadata_directory = Path::new(output_directory).join(".resin");
    let image_name = &format!("{}.png", id);
    let mut generated_metadata = NFTMetadata {
        name: &format!("{} #{}", &config.name, id),
        symbol: &config.symbol,
        description: &config.description,
        seller_fee_basis_points: 0,
        image: image_name,
        external_url: &config.external_url,
        edition: 0,
        attributes: attributes
            .to_vec()
            .drain(..)
            .filter(|attribute| !attribute.trait_type.starts_with("_"))
            .map(|mut attribute| {
                attribute.value = stylize_asset_name(&attribute.value).to_string();
                attribute
            })
            .collect(),
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
    );

    generated_metadata.attributes = attributes;
    write_metadata(
        id,
        &serde_json::to_string(&generated_metadata).expect("Could not serialize generated JSON"),
        &resin_metadata_directory.to_string_lossy().to_string(),
    );
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

fn stylize_asset_name(original: &str) -> &str {
    Path::new(original)
        .file_stem()
        .unwrap_or(OsStr::new(original))
        .to_str()
        .unwrap_or(original)
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
