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

# Develop locally
```sh
git clone https://github.com/glockenberry/resin.git
cd resin
cargo run
```
