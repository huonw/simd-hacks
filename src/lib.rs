//! Functionality for working with SIMD registers and instructions.
//!
//! [Source](https://github.com/huonw/simd), [crates.io](https://crates.io/crates/simd)

#![feature(trace_macros)]

extern crate simdty;
extern crate llvmint;

use std::mem;

pub use convert::bitcast::Bitcast;
pub use convert::semantic::Convert;

mod convert;

mod generated;

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
