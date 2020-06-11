# Distributional analysis of policy gradient in POMDPs using explicit value distributions

This repository is the official implementation for the paper _Distributional analysis of policy gradient in POMDPs using explicit value distributions_.

![Figure 4](https://raw.githubusercontent.com/colin-daniels/lattice_rl/assets/figure-4.png)

## Requirements

The requirements for the library and executables themselves are simply an installation of `rust`
and `cargo`. To render the plots used in the paper, `gnuplot` and `bash` are required. To set up:

```setup
git clone https://github.com/colin-daniels/lattice_rl
cd lattice_rl
cargo build --release --all
```

## Evaluation

To generate the figures, simply `cd` into the `plots` folder, run `generate.sh`, and wait.

```eval
cd plots
./generate.sh
```
