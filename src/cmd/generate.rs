use crate::Generate;
use crate::{art, metadata};
use std::{fs::remove_dir_all, path::Path};

pub fn handle(options: Generate) {
    if !options.skip_metadata {
        println!("Cleaning output directory...");
        remove_dir_all(Path::new(&options.output))
            .expect("Error occured cleaning output directory");

        metadata::generate(&options.config, &options.assets, &options.output);
    } else {
        println!("Skipping metadata generation");
    }

    art::generate(&options.config, &options.assets, &options.output);
}
