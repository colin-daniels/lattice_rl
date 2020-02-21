use itertools::Itertools;
use lattice_rl::env::policy::PlainSoftMax1D;
use lattice_rl::env::GamblersEnv;
use lattice_rl::io::write_gplot_matrix;
use lattice_rl::util::linspace;
use nalgebra::DMatrix;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let data_points = 1024;
    let discretization_bins = 1024;
    let samples_per_bin = 16;
    let transition_cutoff_probability = 1e-13;

    let step_sizes = linspace(-5.0..1.0, data_points)
        .map(|p| 10f64.powf(p))
        .collect_vec();

    for &start in &[5, 3, 1] {
        let env = GamblersEnv::new(
            start, // s_0
            9,     // L
            0.0,   // additional reward for reaching s = 0
            9.0,   // additional reward for reaching s = L
            -1.0,  // per-step penalty
        );

        let progress = AtomicUsize::new(0);
        let data = step_sizes
            .par_iter()
            .enumerate()
            .flat_map(|(_i, &step_size)| {
                let transition_matrix = env.sga_transition_matrix(
                    step_size,
                    &PlainSoftMax1D,
                    discretization_bins,
                    samples_per_bin,
                    transition_cutoff_probability,
                );

                let absorbing = transition_matrix.absorption_probabilities();
                eprintln!(
                    "{:4}/{}",
                    progress.fetch_add(1, Ordering::SeqCst),
                    data_points
                );

                let halfway_point = discretization_bins / 2;
                let data = absorbing.as_slice();
                data.chunks_exact(discretization_bins)
                    .map(|col| col[..halfway_point].iter().sum::<f64>())
                    .collect_vec()
            })
            .collect::<Vec<_>>();

        write_gplot_matrix(
            format!("fig5-converge-s-{}.bin", start),
            &DMatrix::from_vec(discretization_bins, data_points, data),
        )
        .unwrap();
    }
}
