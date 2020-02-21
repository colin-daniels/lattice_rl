use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Hash, Ord, Eq, Default)]
pub struct LCoord2D(pub i32, pub i32);

impl LCoord2D {
    #[inline]
    pub fn neighbors(&self) -> [Self; 3] {
        [
            Self(self.0 + 1, self.1),     // +x
            Self(self.0 - 1, self.1 + 1), // +y
            Self(self.0, self.1 - 1),     // +z
        ]
    }
}

pub struct Lattice2D {
    coords: Vec<LCoord2D>,
    neighbors: Vec<[(usize, f64); 3]>,
}

impl Lattice2D {
    pub fn new(m: i32, reward_fn: impl Fn(LCoord2D) -> f64) -> Self {
        let mut coords = (1..m)
            .flat_map(|q| (1..(m - q)).map(move |r| (LCoord2D(q, r), -1.0)))
            .collect::<Vec<_>>();

        let mut indices = coords
            .iter()
            .enumerate()
            .map(|(i, &(c, _))| (c, i))
            .collect::<HashMap<_, _>>();

        let n_inner_coords = coords.len();

        // boundary states
        for c in 1..m {
            for &bound in &[
                LCoord2D(0, c),     // (x = y)
                LCoord2D(c, 0),     // (y = z)
                LCoord2D(m - c, c), // (x = z)
            ] {
                indices.insert(bound, coords.len());
                coords.push((bound, -1.0 + reward_fn(bound)));
            }
        }

        let neighbors = coords[..n_inner_coords]
            .iter()
            .map(|&(c, _)| {
                let [n1, n2, n3] = c.neighbors();
                let [i1, i2, i3] = [
                    *indices.get(&n1).unwrap(),
                    *indices.get(&n2).unwrap(),
                    *indices.get(&n3).unwrap(),
                ];
                [(i1, coords[i1].1), (i2, coords[i2].1), (i3, coords[i3].1)]
            })
            .collect::<Vec<_>>();

        Self {
            coords: coords.iter().map(|&(c, _)| c).collect(),
            neighbors,
        }
    }

    pub fn coords(&self) -> &[LCoord2D] {
        &self.coords[..self.neighbors.len()]
    }

    pub fn policy_value(&self, action_probabilities: [f64; 3], delta_tolerance: f64) -> Vec<f64> {
        let n_states = self.coords.len();
        let inner_states = self.neighbors.len();

        assert!(delta_tolerance > 0.0 && !delta_tolerance.is_nan());

        let [px, py, pz] = action_probabilities;
        let mut values = vec![0.0; n_states];

        loop {
            let mut delta: f64 = 0.0;
            for i in 0..inner_states {
                let [(i1, r1), (i2, r2), (i3, r3)] = self.neighbors[i];
                let new_val =
                    px * (values[i1] + r1) + py * (values[i2] + r2) + pz * (values[i3] + r3);

                delta = delta.max((values[i] - new_val).abs());
                values[i] = new_val;
            }

            if delta < delta_tolerance {
                values.resize(inner_states, 0.0);
                break values;
            }
        }
    }
}
