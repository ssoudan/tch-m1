# Demo of tch-rs on M1

This repo demonstrate how to use [LaurentMazare/tch-rs](https://github.com/LaurentMazare/tch-rs) on M1.

## Steps to reproduce

- install micromamba with homebrew -- `brew install micromamba` - 
- create a new conda environment: `micromamba env create -f environment.yml`
- activate the new environment: `micromamba activate tch-rs-demo`
- create a symlink in this repo: `ln -sf ~/micromamba/envs/tch-rs-demo/lib/python3.10/site-packages/torch/ torch`
- run: `cargo run`

## More

- .cargo/config.toml is used to set envs for torch-sys to find the library and headers.
