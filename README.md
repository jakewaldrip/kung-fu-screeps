# Kung-Fu Screeps

## Description
Blazingly fast [Screeps](https://screeps.com) bot implemented in Rust.

## Quickstart:
Note: Requires rust nightly

```sh
# Install wasm-pack
cargo install wasm-pack

# Install wasm-opt
cargo install wasm-opt

# Set node version to 20 (confirmed working)
nvm install 20
nvm use 20

# Clone the repo
git clone git@github.com:jakewaldrip/kung-fu-screeps.git
cd kung-fu-screeps

npm install

# Enter token from screeps.com to the desired destinations
cp .example-screeps.yaml .screeps.yaml
nvim .screeps.yaml

# compile for a configured server but don't upload
npm run deploy -- --server ptr --dryrun

# compile and upload to a configured server
npm run deploy -- --server mmo
```
