use lattice_rl::env::GamblersEnv;
use lattice_rl::util::linspace;
use std::iter::once;

fn main() {
    let samples = 100;
    let numerical_diff_step = 1e-6;

    for start in (1..9).rev() {
        let env = GamblersEnv::new(
            start, // s_0
            9,     // L
            0.0,   // additional reward for reaching s = 0
            9.0,   // additional reward for reaching s = L
            -1.0,  // per-step penalty
        );

        let value_fn = env.value_fn();

        // just use central difference
        let h = numerical_diff_step;
        let gradient_fn = |p| (value_fn(p + h) - value_fn(p - h)) / (2.0 * h);

        // the 1e-4's are to avoid issues with endpoints + the fact that this linspace function
        // does not include the endpoint at 1.0
        for p in linspace(1e-4..1.0, samples).chain(once(1.0 - 1e-4)) {
            print!("{} {} {} {}\n", p, value_fn(p), gradient_fn(p), start);
        }
        print!("\n\n");
    }
}
