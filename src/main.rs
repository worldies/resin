use clap::Parser;

mod art;
mod cmd;
mod config;
mod metadata;
mod tests;

/// Generative art program for Solana NFTs
#[derive(Parser, Debug)]
#[clap()]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Generate(Generate),
    Init(Init),
    Verify(Verify),
}

/// A subcommand for controlling testing
#[derive(Parser, Debug)]
pub struct Generate {
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

#[derive(Parser, Debug)]
pub struct Init {
    /// Location of assets folder to initialize
    #[clap(default_value = "./assets")]
    folder: String,

    /// Overwrite assets folder if already exists
    #[clap(long)]
    overwrite: bool,
}

#[derive(Parser, Debug)]
pub struct Verify {
    /// Location of generated folder to verify
    #[clap(default_value = "./generated")]
    folder: String,
}

fn main() {
    let options = Options::parse();
    println!("Starting generator");

    match options.subcmd {
        SubCommand::Generate(c) => cmd::generate::handle(c),
        SubCommand::Init(c) => cmd::init::handle(c),
        SubCommand::Verify(c) => cmd::verify::handle(c),
    }

    println!("Generator finished");
}
