# `resin`
Solana NFT generative artwork program

# Installation
Depends on `libvips` for art generation, which can be installed here: https://www.libvips.org/install.html

Install from source:
```sh
cargo install --git https://github.com/glockenberry/resin
resin --help
```

Or build from source:
```sh
git clone https://github.com/glockenberry/resin.git
cd resin
cargo build --release
./target/release/resin --help
```

# Usage
## From existing
Initialize from existing assets directory under `./assets`
```sh
resin init --from-existing=./assets
```
> Make sure each attribute is a subfolder and each subfolder contains the layers you want to use (reference the structure below)

1. Edit the `config.json` to configure odds
2. Run `resin generate`
3. View generated files in `./generated`!

## From scratch
Initialize new assets directory under `./assets`
```sh
resin init
```

1. Replace the layer folders and images with your own
2. Edit the `config.json`
3. Run `resin generate`
4. View generated files in `./generated`!

Refer to [config.example.json](https://github.com/glockenberry/resin/blob/main/config.example.json) for the format of `config.json`

### Assets directory structure:
```
/assets
    /config.json
    /background
        /blue.png
        /brown.png
        ...
    /eyes
        /egg-eyes.png
        /heart-eyes.png
        ...
    ...
```

## Computed Keys
If you want to use a thing for another thign, computed keys can help you with that.

# Develop locally
```sh
git clone https://github.com/glockenberry/resin.git
cd resin
cargo run
cargo test # make sure all tests pass
```
