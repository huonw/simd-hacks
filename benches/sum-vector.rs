extern crate test;
extern crate simd;
extern crate simdty;

use simdty::*;
use simd::*;

#[bench]
fn simd_(b: &mut test::Bencher) {
    let v = (0..1000).map(|i| i as f32).collect::<Vec<_>>();

    // sum the roots of an f32 vector as i32's, via SIMD.
    b.iter(|| {
        let Tails {start,middle,end}: Tails<_, f32x4> = test::black_box(&v[]).convert();
        let mut ret = i32x4(0, 0, 0, 0);
        for elem in middle.iter() {
            ret = ret + sqrt(*elem).convert();
        }
        let leftovers = start.iter().chain(end.iter()).fold(0, |a, b| a + sqrt(*b) as i32);

        ret.0 + ret.1 + ret.2 + ret.3 + leftovers
    })
}
#[bench]
fn nonsimd(b: &mut test::Bencher) {
    let v = (0..1000).map(|i| i as f32).collect::<Vec<f32>>();

    b.iter(|| test::black_box(&v[]).iter().fold(0, |a, b| a + sqrt(*b) as i32))
}
