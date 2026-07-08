#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn f64_to_usize(x: f64) -> usize {
    x.round().abs() as usize
}
