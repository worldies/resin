# `resin`
Solana NFT generative artwork program

# Installation
Depends on `imagemagick` for art generation, which can be installed here: https://imagemagick.org/script/download.php

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
Initialize the assets directory under `./assets`
```sh
resin init
```

1. Replace the layer folders and images with your own
2. Edit the `config.json`
3. Run `resin generate`
4. View generated files in `./generated`!


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

# Develop locally
```sh
git clone https://github.com/glockenberry/resin.git
cd resin
cargo run
```
