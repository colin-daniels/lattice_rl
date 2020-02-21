use crate::env::analytic::tridiagonal_toeplitz_inverse_element;
use crate::env::policy::Policy1D;
use crate::env::sim::simulated_return_probabilities;
use crate::transition::{Transition, TransitionMatrix};

pub mod analytic;
pub mod policy;
pub mod sim;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct ReturnProbability {
    pub probability: f64,
    pub steps_left: usize,
    pub steps_right: usize,
    pub total_return: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct GamblersEnv {
    start: usize,
    len: usize,
    reward_left: f64,
    reward_right: f64,
    reward_step: f64,
}

impl GamblersEnv {
    pub fn new(
        start: usize,
        len: usize,
        // reward for hitting s = 0
        reward_left: f64,
        // reward for hitting s = len
        reward_right: f64,
        // reward for each step (usually negative)
        reward_step: f64,
    ) -> Self {
        assert!(len >= 3);
        assert!(0 < start && start < len);
        Self {
            start,
            len,
            reward_left,
            reward_right,
            reward_step,
        }
    }

    pub fn sga_transition_matrix(
        &self,
        step_size: f64,
        policy: &impl Policy1D,
        discretization_bins: usize,
        samples_per_bin: usize,
        cutoff_probability: f64,
    ) -> TransitionMatrix {
        let transition_function = |p: f64| {
            let mut remaining_probability = 1.0;
            let [grad_left, grad_right] = policy.log_gradient(p);
            let theta = policy.theta(p);

            simulated_return_probabilities(&self, p)
                .map(
                    move |ReturnProbability {
                              probability,
                              steps_right,
                              steps_left,
                              total_return,
                          }| {
                        let gradient = total_return
                            * (steps_left as f64 * grad_left + steps_right as f64 * grad_right);

                        let new_theta = theta + step_size * gradient;
                        let new_p = policy.p(new_theta);

                        Transition {
                            probability,
                            to: new_p,
                        }
                    },
                )
                .take_while(move |t| {
                    // we don't expect to take into account _all_ possible transitions, just
                    // most of them (dependent on cutoff probability)
                    remaining_probability -= t.probability;
                    remaining_probability > cutoff_probability
                })
        };

        TransitionMatrix::from_transition_fn(
            0.0..1.0,
            discretization_bins,
            samples_per_bin,
            transition_function,
        )
    }

    pub fn value_fn(&self) -> impl Fn(f64) -> f64 + '_ {
        let i = self.start - 1;
        let n = self.len - 1;

        move |p: f64| {
            let alpha = p;
            let beta = 1.0;
            let gamma = 1.0 - p;

            let elem = |s| tridiagonal_toeplitz_inverse_element(i, s, n, alpha, beta, gamma);

            (0..n).map(elem).sum::<f64>() * self.reward_step
                + p * self.reward_right * elem(n - 1)
                + (1.0 - p) * self.reward_left * elem(0)
        }
    }
}
