pub fn f64_to_usize(x: f64) -> usize {
    assert!(x >= 0.0);
    assert!(x <= usize::MAX as f64);
    assert!(x == x as usize as f64);
    x as usize
}
