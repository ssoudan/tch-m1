# Demo of tch-rs on M1

## Steps

- install miniforge with homebrew -- See  https://naolin.medium.com/conda-on-m1-mac-with-miniforge-bbc4e3924f2b
- create a new conda environment: `conda create -n tch-rs-demo python=3.8`
- activate the new environment: `conda activate tch-rs-demo`
- install pytorch: `conda install pytorch`
- create a symlink in this repo: `ln -s /opt/homebrew/Caskroom/miniforge/base/envs/tch-rs-demo/lib/python3.8/site-packages/torch/ torch`
- run: `cargo run`

## Details

- .cargo/config.toml is used to set envs for torch-sys to find the library and headers.
- Tensor::print() segfaults.

