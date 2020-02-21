use nalgebra::{DMatrix, DVector};
use rayon::current_thread_index;
use rayon::prelude::*;
use std::ops::Range;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Transition {
    pub to: f64,
    pub probability: f64,
}

#[derive(Clone, Debug)]
pub struct TransitionMatrix(DMatrix<f64>);

impl TransitionMatrix {
    pub fn from_transition_fn<I: Iterator<Item = Transition>>(
        domain: Range<f64>,
        discretization_bins: usize,
        samples_per_bin: usize,
        transitions_from: impl Fn(f64) -> I + std::marker::Sync,
    ) -> Self {
        assert_ne!(discretization_bins, 0);

        let Range { start, end } = domain;
        assert!(start.is_finite() && end.is_finite());

        // TODO: should check conversion...no TryFrom implementation for f64 from usize though...
        let bin_step = (end - start) / discretization_bins as f64;
        let sub_step = bin_step / samples_per_bin as f64;

        // FIXME: has hack to ensure first/last columns transition to themselves, since we must
        //  ensure that all states can reach an absorbing state. In this case its a hack
        //  specifically for our gambler's 1D environment
        let process_column = |(i, column): (usize, &mut [f64])| {
            let bin_start = start + bin_step * i as f64;

            let mut total_probability = 0.0;
            if i > 2 && i + 3 < discretization_bins {
                for j in 0..samples_per_bin {
                    // points are centered in the bins
                    let x = bin_start + sub_step * (j as f64 + 0.5);

                    for Transition {
                        to: new_x,
                        probability,
                    } in transitions_from(x)
                    {
                        assert!(
                            start <= new_x,
                            "transition {} -> {} is out of range [{},{})",
                            x,
                            new_x,
                            start,
                            end
                        );

                        assert!(0.0 <= probability && probability <= 1.0);
                        let new_i = f64::floor((new_x - start) / bin_step) as usize;
                        let new_i = new_i.min(discretization_bins - 1);

                        column[new_i] += probability;
                        total_probability += probability;
                    }
                }
            }

            // ensure that transitions are normalized, etc
            if total_probability == 0.0 {
                column[i] = 1.0;
            } else {
                for v in column {
                    *v /= total_probability;
                }
            }
        };

        let mut data = vec![0.0; discretization_bins * discretization_bins];
        // parallelize if not already in a thread
        // note: both of these treat chunks as columns because DMatrix is column-major
        if current_thread_index().is_none() {
            data.par_chunks_mut(discretization_bins)
                .enumerate()
                .for_each(process_column);
        } else {
            data.chunks_mut(discretization_bins)
                .enumerate()
                .for_each(process_column);
        }

        Self(DMatrix::from_vec(
            discretization_bins,
            discretization_bins,
            data,
        ))
    }

    pub fn data(&self) -> &DMatrix<f64> {
        &self.0
    }

    /// Get absorption probabilities just by taking a large matrix power.  
    pub fn absorption_probabilities(&self) -> DMatrix<f64> {
        const POW_2: usize = 50;

        fn renormalize(m: &mut DMatrix<f64>) {
            for mut col in m.column_iter_mut() {
                col.scale_mut(1.0 / col.sum());
            }
        }

        let mut current = self.0.to_owned();
        let mut power = self.0.to_owned();
        for _ in 0..POW_2 {
            current.mul_to(&current, &mut power);
            renormalize(&mut power);
            power.mul_to(&power, &mut current);
            renormalize(&mut current);
        }
        current
    }

    pub fn time_evolution(&self, mut initial: DVector<f64>, steps: usize) -> DMatrix<f64> {
        let bins = self.0.nrows();
        assert_eq!(initial.len(), bins);

        let mut output: DMatrix<f64> = DMatrix::zeros(bins, steps);
        for i in 0..steps {
            output.set_column(i, &initial);
            initial = &self.0 * initial;
        }
        output
    }
}
