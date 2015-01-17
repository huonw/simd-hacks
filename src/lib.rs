//! Functionality for working with SIMD registers and instructions.
//!
//! [Source](https://github.com/huonw/simd), [crates.io](https://crates.io/crates/simd)

#![feature(trace_macros)]
use std::{simd, mem};

/// A SIMD vector, storing precisely `count()` elements of (primitive)
/// type `Item` sequentially in memory.
///
/// This trait is `unsafe` because it is used inside `unsafe` code
/// relying on the correctness of `Item` and `count()`.
pub unsafe trait Vector: Sized + Copy {
    /// The type that this vector contains.
    type Item;

    /// Return the number of items in `self`.
    #[inline(always)]
    fn count(_: Option<Self>) -> usize {
        let vsize = mem::size_of::<Self>();
        let esize = mem::size_of::<Self::Item>();
        assert!(vsize % esize == 0);

        vsize / esize
    }
}

macro_rules! prim_impl {
    ($( $ty: ident ),*) => {
        $(
            unsafe impl Vector for $ty {
                type Item = $ty;
                #[inline(always)]
                fn count(_: Option<$ty>) -> usize { 1 }
            }
            )*
    }
}

prim_impl! {
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    usize, isize,

    f32, f64
}

macro_rules! vector_impl {
    ($( $main: ident, $item: ty, $count: expr; )* ) => {
        $(
            unsafe impl Vector for simd::$main {
                type Item = $item;
                #[inline(always)]
                fn count(_: Option<simd::$main>) -> usize {
                    $count
                }
            }
            )*
    }
}

vector_impl! {
    i8x16, i8, 16;
    u8x16, u8, 16;

    i16x8, i16, 8;
    u16x8, u16, 8;

    i32x4, i32, 4;
    u32x4, u32, 4;

    i64x2, i64, 2;
    u64x2, u64, 2;

    f32x4, f32, 4;
    f64x2, f64, 2;
}
