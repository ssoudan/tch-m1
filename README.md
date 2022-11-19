# Demo of tch-rs on M1

This repo demonstrate how to use [LaurentMazare/tch-rs](https://github.com/LaurentMazare/tch-rs) on M1.

## Steps to reproduce

- install miniforge with homebrew -- See https://naolin.medium.com/conda-on-m1-mac-with-miniforge-bbc4e3924f2b
- create a new conda environment: `conda env create -f environment.yml`
- activate the new environment: `conda activate tch-rs-demo`
- create a symlink in this repo: `ln -sf /opt/homebrew/Caskroom/miniforge/base/envs/tch-rs-demo/lib/python3.8/site-packages/torch/ torch`
- run: `cargo run`

## More

- .cargo/config.toml is used to set envs for torch-sys to find the library and headers.
