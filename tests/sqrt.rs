extern crate simd;
extern crate simdty;

use std::num::Float;

use simdty::*;

// these are so small/are a perfect squares that one would hope that
// the CPU get compute them exactly.

#[test]
fn test_sqrt() {
    let a = f32x4(0.0, 1.0, 4.0, 9.0);
    let b = simd::sqrt(a);
    assert_eq!(b.0, 0.0);
    assert_eq!(b.1, 1.0);
    assert_eq!(b.2, 2.0);
    assert_eq!(b.3, 3.0);
}

#[test]
fn test_rsqrt() {
    // powers of two are more likely to be computer accurately
    let a = f32x4(1.0, 1.0/4.0, 1.0/16.0, 1.0/64.0);
    let b = simd::rsqrt(a);
    assert!((b.0 - 1.0).abs() < 0.01);
    assert!((b.1 - 2.0).abs() < 0.01);
    assert!((b.2 - 4.0).abs() < 0.01);
    assert!((b.3 - 8.0).abs() < 0.01);
}
