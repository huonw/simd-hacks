extern crate simd;
extern crate simdty;

use simd::*;
use simdty::*;

#[test]
fn bitcast_value() {
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
fn convert_value() {
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

#[test]
fn bitcast_slice() {
    let a: &[i8x2] = &[i8x2(1, 2), i8x2(3, 4)];
    let b: &[u16] = a.bitcast();

    if cfg!(target_endian = "little") {
        assert_eq!(b, &[0x02_01, 0x04_03]);
    } else {
        assert_eq!(b, &[0x01_02, 0x03_03]);
    }
}

#[test]
fn convert_slice() {
    let a: &[u64x2] = &[u64x2(1, 2), u64x2(3, 4)];
    let b: &[u8x16] = a.bitcast();
    let c: &[u8] = b.convert();
    assert_eq!(c.len(), 8 * 2 * 2);
}
#[test]
#[should_panic(expected = "align_of")]
fn convert_slice_misaligned() {
    let a: &[u64x2] = &[u64x2(1, 2), u64x2(3, 4)];
    let b: &[u8x16] = a.bitcast();
    let c: &[u8] = b.convert();
    let _: &[u8x16] = c[1..17].convert();
}
#[test]
#[should_panic(expected = "size_of")]
fn convert_slice_missized() {
    let a: &[u64x2] = &[u64x2(1, 2), u64x2(3, 4)];
    let b: &[u8x16] = a.bitcast();
    let c: &[u8] = b.convert();
    let _: &[u8x16] = c[0..31].convert();
}
#[test]
fn convert_slice_mut() {
    let a: &mut [u64x2] = &mut [u64x2(1, 2), u64x2(3, 4)];
    let b: &mut [u8x16] = a.bitcast();
    let c: &mut [u8] = b.convert();
    assert_eq!(c.len(), 8 * 2 * 2);
}
#[test]
#[should_panic(expected = "align_of")]
fn convert_slice_misaligned_mut() {
    let a: &mut [u64x2] = &mut [u64x2(1, 2), u64x2(3, 4)];
    let b: &mut [u8x16] = a.bitcast();
    let c: &mut [u8] = b.convert();
    let _: &mut [u8x16] = (&mut c[1..17]).convert();
}
#[test]
#[should_panic(expected = "size_of")]
fn convert_slice_missized_mut() {
    let a: &mut [u64x2] = &mut [u64x2(1, 2), u64x2(3, 4)];
    let b: &mut [u8x16] = a.bitcast();
    let c: &mut [u8] = b.convert();
    let _: &mut [u8x16] = (&mut c[0..31]).convert();
}

#[test]
fn convert_slice_tails() {
    let a: &[u64x2] = &[u64x2(1, 2), u64x2(3, 4), u64x2(5,6)];
    let b: &[u8x16] = a.bitcast();
    let c: &[u8] = b.convert();
    let d: Tails<u8, u8x16> = c[1..35].convert();

    assert_eq!(d.start, &c[1..16]);
    assert_eq!(d.middle.len(), 1);
    let converted_middle: &[u8] = d.middle.convert();
    assert_eq!(converted_middle, &c[16..32]);
    assert_eq!(d.end, &c[32..35])
}
#[test]
fn convert_slice_tailsmut() {
    let mut v = [u64x2(1, 2), u64x2(3, 4), u64x2(5,6)];
    let mut w = v;
    let a: &mut [u64x2] = &mut w;
    let b: &mut [u8x16] = a.bitcast();
    let c: &mut [u8] = b.convert();
    let d: TailsMut<u8, u8x16> = (&mut c[1..35]).convert();


    let a_: &mut [u64x2] = &mut v;
    let b_: &mut [u8x16] = a_.bitcast();
    let c_: &mut [u8] = b_.convert();

    assert_eq!(d.start, &mut c_[1..16]);
    assert_eq!(d.middle.len(), 1);
    let converted_middle: &mut [u8] = d.middle.convert();
    assert_eq!(converted_middle, &mut c_[16..32]);
    assert_eq!(d.end, &mut c_[32..35])
}
