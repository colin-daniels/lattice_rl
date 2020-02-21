use nalgebra::{DMatrix, DVector};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_gplot_matrix(filename: impl AsRef<Path>, data: &DMatrix<f64>) -> std::io::Result<()> {
    let (m, n) = data.shape();
    let bytes = to_gplot_matrix_bytes(
        &data,
        &DVector::from_fn(n, |i, _| i as f64),
        &DVector::from_fn(m, |j, _| j as f64),
    );

    File::create(filename)?.write_all(&bytes)
}

pub fn to_gplot_matrix_bytes(
    data: &DMatrix<f64>,
    x_coords: &DVector<f64>,
    y_coords: &DVector<f64>,
) -> Vec<u8> {
    let (m, n) = data.shape();
    assert_eq!(x_coords.len(), n);
    assert_eq!(y_coords.len(), m);

    let expected_elem = (m + 1) * (n + 1);
    let expected_bytes = expected_elem * std::mem::size_of::<f32>();

    let mut bytes = Vec::with_capacity(expected_bytes);
    // gnuplot binary format (single precision floats):
    //
    //     <M>  <y1>   <y2>   <y3>   ... <yM>
    //     <x1> <z1,1> <z1,2> <z1,3> ... <z1,M>
    //     <x2> <z2,1> <z2,2> <z2,3> ... <z2,M>
    //      :      :      :      :   ...    :
    //     <xN> <zN,1> <zN,2> <zN,3> ... <zN,M>
    //
    // header row
    bytes.extend_from_slice(&f32::to_le_bytes(m as f32));
    for &y in y_coords {
        bytes.extend_from_slice(&f32::to_le_bytes(y as f32));
    }

    // data rows
    for (&x, col) in x_coords.iter().zip(data.column_iter()) {
        bytes.extend_from_slice(&f32::to_le_bytes(x as f32));
        for &d in col.iter() {
            bytes.extend_from_slice(&f32::to_le_bytes(d as f32));
        }
    }

    assert_eq!(bytes.len(), expected_bytes);
    bytes
}
