use lattice_rl::env::policy::PlainSoftMax1D;
use lattice_rl::env::GamblersEnv;
use lattice_rl::io::write_gplot_matrix;
use nalgebra::DVector;

fn main() {
    let discretization_bins = 1024;
    let samples_per_bin = 64;
    let timesteps = 2048;
    let sga_stepsize = 2e-4;
    let transition_cutoff_probability = 1e-12;

    let env = GamblersEnv::new(
        3,    // s_0
        9,    // L
        0.0,  // additional reward for reaching s = 0
        9.0,  // additional reward for reaching s = L
        -1.0, // per-step penalty
    );

    let transition_matrix = env.sga_transition_matrix(
        sga_stepsize,
        &PlainSoftMax1D,
        discretization_bins,
        samples_per_bin,
        transition_cutoff_probability,
    );

    let evolution_data = |p| {
        let mut initial = DVector::zeros(discretization_bins);
        let bin = f64::floor(p * discretization_bins as f64) as usize;
        initial[bin] = 1.0;

        transition_matrix.time_evolution(initial, timesteps)
    };

    write_gplot_matrix("fig1-time-evo-0.50.bin", &evolution_data(0.50)).unwrap();
    write_gplot_matrix("fig1-time-evo-0.56.bin", &evolution_data(0.56)).unwrap();
}
