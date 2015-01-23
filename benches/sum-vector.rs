extern crate test;
extern crate simd;
extern crate simdty;

use simdty::*;
use simd::*;

#[bench]
fn simd_(b: &mut test::Bencher) {
    let v = (0..10000).map(|i| i as f64).collect::<Vec<_>>();

    // sum a f64 vector as i32's, via SIMD.
    b.iter(|| {
        let Tails {start,middle,end}: Tails<_, f64x2> = test::black_box(&v[]).convert();
        let mut ret = i32x4(0, 0, 0, 0);
        for elem in middle.iter() {
            ret = ret + elem.convert();
        }
        let leftovers = start.iter().chain(end.iter()).fold(0, |a, b| a + *b as i32);

        ret.0 + ret.1 + leftovers
    })
}
#[bench]
fn nonsimd(b: &mut test::Bencher) {
    let v = (0..10000).map(|i| i as f64).collect::<Vec<f64>>();

    b.iter(|| test::black_box(&v[]).iter().fold(0, |a, b| a + *b as i32))
}
