use clap::Parser;

mod art;
mod config;
mod metadata;

/// Generative art program for Solana NFTs
#[derive(Parser, Debug)]
#[clap()]
struct Options {
    /// Whether to use already present metadata to generate art
    #[clap(long)]
    skip_metadata: bool,

    /// Location of assets to generate
    #[clap(short, long, default_value = "./assets")]
    assets: String,

    /// Location of configuration file
    #[clap(short, long, default_value = "./assets/config.json")]
    config: String,

    /// Ouput location of generated art
    #[clap(short, long, default_value = "./generated")]
    output: String,
}

fn main() {
    let options = Options::parse();
    println!("Starting generator");

    if !options.skip_metadata {
        metadata::generate(&options.config, &options.assets, &options.output);
    } else {
        println!("Skipping metadata generation");
    }

    art::generate(&options.config, &options.assets, &options.output);

    println!("Generator finished");
}
