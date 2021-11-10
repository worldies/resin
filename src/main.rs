mod metadata;
mod art;

fn main() {
    println!("Starting generator");
    metadata::generate();
    art::generate();
    println!("Generator finished");
}
