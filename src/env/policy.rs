pub trait Policy1D: Sync {
    fn theta(&self, p: f64) -> f64;
    fn p(&self, theta: f64) -> f64;
    fn log_gradient(&self, p: f64) -> [f64; 2];
}

/// One-dimensional softmax policy.
pub struct SoftMax1D {
    epsilon: f64,
    temperature: f64,
}

impl Policy1D for SoftMax1D {
    fn theta(&self, p: f64) -> f64 {
        // θ(p) = t * atanh((1 - 2 p) / (ϵ - 1))
        self.temperature * f64::atanh((1.0 - 2.0 * p) / (self.epsilon - 1.0))
    }

    fn p(&self, theta: f64) -> f64 {
        // p(θ) = (1 - ϵ) (1 + tanh(θ / t)) / 2 + ϵ / 2
        let p = (1.0 + f64::tanh(theta / self.temperature)) / 2.0;
        (1.0 - self.epsilon) * p + self.epsilon / 2.0
    }

    fn log_gradient(&self, p: f64) -> [f64; 2] {
        // d/dθ log(pi(+1 | θ)) = ...
        //
        //  grad_left(θ) = (ϵ - 1) sech^2(θ / t) / (t ((ϵ - 1) tanh(θ / t) + 1))
        // grad_right(θ) = (ϵ - 1) sech^2(θ / t) / (t ((ϵ - 1) tanh(θ / t) - 1))
        //
        // it we put in terms of p instead of theta, we get:
        //
        //  grad_left(p) = ((2 p - ϵ) (-2 + 2 p + ϵ)) / (2 (p - 1) t (ϵ - 1))
        // grad_right(p) = ((2 p - ϵ) (-2 + 2 p + ϵ)) / (2 p t (ϵ - 1))
        let q = 1.0 - p;
        //  grad_left(p) = -(2 p - ϵ) (2(1 - p) - ϵ) / (2 (1 - p) (1 - ϵ) t)
        //               = -(2 p - ϵ) (2 q - ϵ) / (2 q (1 - ϵ) t)
        // grad_right(p) = +(2 p - ϵ) (2 q - ϵ) / (2 p (1 - ϵ) t)

        // (2 p - ϵ) (2 q - ϵ) / (2 (1 - ϵ) t)
        let common = (2.0 * p - self.epsilon) * (2.0 * q - self.epsilon)
            / (2.0 * (1.0 - self.epsilon) * self.temperature);

        [-common / q, common / p]
    }
}

/// Identical to `SoftMax1D` with `epsilon = 0` and `temperature = 1`.
pub struct PlainSoftMax1D;

impl Policy1D for PlainSoftMax1D {
    fn theta(&self, p: f64) -> f64 {
        // θ(p) = atanh(2 p - 1)
        f64::atanh(2.0 * p - 1.0)
    }

    fn p(&self, theta: f64) -> f64 {
        // p(θ) = (1 + tanh(θ)) / 2
        (1.0 + f64::tanh(theta)) / 2.0
    }

    fn log_gradient(&self, p: f64) -> [f64; 2] {
        // d/dθ log(pi(+1 | θ)) = ...
        //
        //  grad_left(θ) = -(1 + tanh(θ))
        // grad_right(θ) =  (1 - tanh(θ))
        //
        // it we put in terms of p instead of theta, we get:
        //
        //  grad_left(p) =   - 2 p
        // grad_right(p) = 2 - 2 p
        [-2.0 * p, -2.0 * p + 2.0]
    }
}
