const MAX_EXACT_FLOAT: usize = 9_007_199_254_740_992;

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn round_and_clamp(x: f64) -> usize {
    x.round().abs() as usize
}

#[allow(clippy::cast_precision_loss)]
pub fn usize_to_f64(x: usize) -> f64 {
    assert!(
        x <= MAX_EXACT_FLOAT,
        "Cannot convert usize to f64 without losing precision"
    );

    x as f64
}
