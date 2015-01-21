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

/// SIMD vectors which can be separated into two SIMD vectors of half
/// the size with the same elements.
pub unsafe trait HalfVector: Vector {
    type Half /*: Vector<Item = Self::Item>*/;

    /// Retrieve the upper and lower halves of the `self` vector.
    fn split(self) -> (Self::Half, Self::Half);

    /// Retrieve the lower half of the `self` vector.
    #[inline]
    fn lower(self) -> Self::Half { self.split().0 }
    /// Retrieve the upper half of the `self` vector.
    #[inline]
    fn upper(self) -> Self::Half { self.split().1 }
}
/// SIMD vectors which can be merged with another of the same type to
/// create one of double the length with the same elements.
pub unsafe trait DoubleVector: Vector {
    type Double /*: Vector<Item = Self::Item>*/;

    /// Concatenate the elements of `self` and `other` into a single
    /// SIMD vector.
    fn merge(self, other: Self) -> Self::Double;
}
