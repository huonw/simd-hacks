extern crate simd;
extern crate simdty;

use simd::{Vector,Bitcast,Convert};
use simdty::*;

#[test]
fn bitcast() {
    let a = f32x4(1.0, 2.0, 3.0, 4.0);
    let b: i32x4 = a.bitcast();
    let c: u8x16 = b.bitcast();
    let d: f32x4 = c.bitcast();

    assert_eq!(a.0, d.0);
    assert_eq!(a.1, d.1);
    assert_eq!(a.2, d.2);
    assert_eq!(a.3, d.3);
}

#[test]
fn convert() {
    let a = f32x4(1.0, 2.0, 3.0, 4.0);
    let b: i32x4 = a.convert();
    let c: f32x4 = b.convert();

    assert_eq!(a.0, c.0);
    assert_eq!(a.1, c.1);
    assert_eq!(a.2, c.2);
    assert_eq!(a.3, c.3);

    let a = f32x8(1e9, 1.0, 2.0, 3.0, -1e9, -5.0, -100.0, 20.0);
    let b: i32x8 = a.convert();
    let c: f32x8 = b.convert();
    assert_eq!(a.0, c.0);
    assert_eq!(a.1, c.1);
    assert_eq!(a.2, c.2);
    assert_eq!(a.3, c.3);
    assert_eq!(a.4, c.4);
    assert_eq!(a.5, c.5);
    assert_eq!(a.6, c.6);
    assert_eq!(a.7, c.7);
}
