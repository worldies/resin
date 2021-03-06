use std::{
    fs::{read_dir, read_to_string, remove_dir_all},
    path::Path,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use crate::metadata::NFTMetadata;

const NUM_THREADS: usize = 20;

pub fn generate(_config_location: &String, assets_directory: String, output_directory: String) {
    println!("Generating artwork from metadata...");

    let mut resin_metadata_directory_present = false;
    let metadata_directory = {
        let p = Path::new(&output_directory).join(".resin/");
        if p.is_dir() {
            resin_metadata_directory_present = true;
            p.to_string_lossy().to_string()
        } else {
            output_directory.clone()
        }
    };

    read_metadata(
        Box::leak(assets_directory.into_boxed_str()),
        Box::leak(metadata_directory.clone().into_boxed_str()),
        Box::leak(output_directory.into_boxed_str()),
    );

    if resin_metadata_directory_present {
        let _ = remove_dir_all(metadata_directory);
    }
}

fn read_metadata(
    assets_directory: &'static str,
    metadata_directory: &'static str,
    output_directory: &'static str,
) {
    let files = read_dir(metadata_directory).expect(&format!(
        "Could not read source directory {}",
        metadata_directory
    ));
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];
    let num_completed = Arc::new(Mutex::new(0 as u32));

    let num_clone = num_completed.clone();
    thread::spawn(move || {
        let mut old_completed = 0;
        loop {
            let n = *num_clone.lock().unwrap();
            if n != old_completed {
                old_completed = n;
                println!("{} jobs completed", n);
            }
        }
    });

    for file_raw in files {
        let file = file_raw.expect("Could not read file");
        match file.path().extension() {
            Some(extension) => {
                if extension != "json" {
                    continue;
                }
            }
            None => continue,
        }

        if threads.len() >= NUM_THREADS {
            let t = threads.remove(0);
            t.join().expect("Thread failed to finish cleanly");
            *num_completed.lock().unwrap() += 1;
        }
        threads.push(thread::spawn(move || {
            let contents = read_to_string(file.path()).expect(&format!(
                "Could not read file contents for file {}",
                file.path().display()
            ));
            let parsed_metadata: NFTMetadata = serde_json::from_str(&contents).expect(&format!(
                "Could not parse metadata JSON for file {}",
                file.path().display()
            ));
            let file_name = file.file_name();
            let id = file_name
                .to_str()
                .unwrap()
                .split('.')
                .next()
                .expect(&format!(
                    "Could not get ID from file name for file {}",
                    file.path().display()
                ));

            create_image(id, &parsed_metadata, &assets_directory, &output_directory);
        }));
    }

    for child in threads {
        let _ = child.join();
        *num_completed.lock().unwrap() += 1;
    }
}

fn create_image(id: &str, metadata: &NFTMetadata, assets_directory: &str, output_directory: &str) {
    let image_path_buffer = Path::new(output_directory).join(format!("{}.png", id));
    let image_path = image_path_buffer.to_str().expect(&format!(
        "Image is not valid path at {}",
        image_path_buffer.display()
    ));

    let mut composite_command = Command::new("vips");
    composite_command.arg("composite");

    let mut layers = vec![];
    for attribute in &metadata.attributes {
        if attribute.trait_type.starts_with("_") {
            continue;
        }
        let layer_path_buffer = Path::new(assets_directory)
            .join(&attribute.trait_type)
            .join(&attribute.value);
        let layer_path = layer_path_buffer.to_str().expect(&format!(
            "Layer is not valid path at {}",
            layer_path_buffer.display()
        ));
        if !layer_path_buffer.exists() {
            panic!("Layer does not exist at path {}", layer_path);
        }

        layers.push(layer_path.replace(" ", "\\ ")); // Escape spaces in path
    }

    composite_command
        .arg(layers.join(" "))
        .arg(image_path)
        .arg("2") // Use blending mode "source"
        .spawn()
        .expect(&format!("Error creating image {}", id))
        .wait()
        .expect(&format!("Error creating image {}", id));
}
