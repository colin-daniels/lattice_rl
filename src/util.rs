use std::ops::{Add, Mul, Range, Sub};

#[inline]
pub fn lerp<T>(from: T, to: T, t: T) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    (to - from) * t + from
}

/// Create iterator over linearly spaced points within the input range, does not include endpoint.
#[inline]
pub fn linspace(range: Range<f64>, num: usize) -> impl Iterator<Item = f64> {
    let start = range.start;
    let step = (range.end - range.start) / num as f64;
    (0..num).map(move |n| n as f64 * step + start)
}
