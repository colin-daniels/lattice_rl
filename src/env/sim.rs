use super::{GamblersEnv, ReturnProbability};

/// Calculate return probabilities for a policy that has probability `p` to make a step to
/// the right by updating the probability density for each state in the environment iteratively.
pub fn simulated_return_probabilities(
    env: &GamblersEnv,
    p: f64,
) -> impl Iterator<Item = ReturnProbability> {
    let GamblersEnv {
        start,
        len,
        reward_left,
        reward_right,
        reward_step,
        ..
    } = *env;

    // probability values for current/last iteration
    // note that if this is a performance bottleneck, we don't actually need two of them,
    // one would work fine if we update in two passes (odd/even entries)
    let mut current = vec![0.0; len + 1];
    let mut last = vec![0.0; len + 1];
    // starting probability
    current[start] = 1.0;

    (0..).flat_map(move |step| {
        // get probabilities for the _last_ iteration

        // if we hit x = 0
        let left = Some((1.0 - p) * last[1])
            .filter(|&probability| probability != 0.0)
            .map(|probability| ReturnProbability {
                probability,
                steps_left: (step + start) / 2,
                steps_right: (step - start) / 2,
                total_return: reward_left + reward_step * step as f64,
            });

        // if we hit x = len
        let right = Some(p * last[len - 1])
            .filter(|&probability| probability != 0.0)
            .map(|probability| ReturnProbability {
                probability,
                steps_left: (step - (len - start)) / 2,
                steps_right: (step + (len - start)) / 2,
                total_return: reward_right + reward_step * step as f64,
            });

        // move current to last and update
        std::mem::swap(&mut current, &mut last);

        // calculate current probability to be at each position in the grid based on the
        // fact that each state moves to the left or right with (1 - p) or p probability,
        // respectively
        for i in 1..len {
            current[i] = last[i - 1] * p + last[i + 1] * (1.0 - p);
        }

        left.into_iter().chain(right.into_iter())
    })
}
