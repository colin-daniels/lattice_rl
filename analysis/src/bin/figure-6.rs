use itertools::Itertools;
use lattice_rl::lattice::{LCoord2D, Lattice2D};
use lattice_rl::util::linspace;
use nalgebra::Vector2;
use std::fs::File;
use std::io::Write;

// likely a simple expression for this, and could definitely make this a static function/object
// but not really worth it at the moment
fn ternary_coord_transform() -> impl Fn(f64, f64) -> Option<[f64; 3]> {
    type V2 = Vector2<f64>;

    let apex_y = V2::new(0.0, 1.0);
    let apex_x = V2::new(f64::sqrt(3.0) / 2.0, 0.5);
    let apex_z = V2::new(0.0, 0.0);

    let build_axis_fn = |a1: V2, a2: V2, a3: V2| {
        let midpoint = (a2 + a3) / 2.0;
        let axis = a1 - midpoint;
        let norm_sq = axis.norm_squared();
        move |v: V2| -> f64 { (v - midpoint).dot(&axis) / norm_sq }
    };

    let get_x = build_axis_fn(apex_x, apex_y, apex_z);
    let get_y = build_axis_fn(apex_y, apex_z, apex_x);
    let get_z = build_axis_fn(apex_z, apex_x, apex_y);
    move |vx, vy| {
        let v = V2::new(vx, vy);

        let x = get_x(v);
        let y = get_y(v);
        let z = get_z(v);
        if (0.0 <= x && x <= 1.0) && (0.0 <= y && y <= 1.0) && (0.0 <= z && z <= 1.0) {
            Some([x, y, z])
        } else {
            None
        }
    }
}

fn main() {
    let m = 6;
    let samples = 256;
    let delta_tolerance = 1e-15;

    // reward of 0 if ending next to any of the three corners, otherwise -10
    let reward_fn = |LCoord2D(q, r)| match (q, r) {
        (0, c) /* x == y */ => if c == 1 || c == m - 1 { 1.0 } else { -9.0 },
        (c, 0) /* y == z */ => if c == 1 || c == m - 1 { 1.0 } else { -9.0 },
        (_, c) /* z == x */ => if c == 1 || c == m - 1 { 1.0 } else { -9.0 },
    };

    let lattice = Lattice2D::new(m, reward_fn);
    let coords = lattice.coords();

    let mut files = coords
        .iter()
        .map(|c| File::create(format!("value-3d-({}-{}).txt", c.0, c.1)).unwrap())
        .collect_vec();

    let into_ternary = ternary_coord_transform();
    for x in linspace(0.0..1.0, samples) {
        for y in linspace(0.0..1.0, samples) {
            if let Some(action_probabilities) = into_ternary(x, y) {
                let values = lattice.policy_value(action_probabilities, delta_tolerance);
                for i in 0..coords.len() {
                    write!(files[i], "{} {} {}\n", x, y, values[i]).unwrap();
                }
            } else {
                for i in 0..coords.len() {
                    write!(files[i], "{} {} NaN\n", x, y).unwrap();
                }
            }
        }
        for file in &mut files {
            writeln!(file).unwrap();
        }
    }
}
