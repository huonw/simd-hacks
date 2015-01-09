use std::simd;

pub type usize = uint;

/// A SIMD vector, storing `count()` elements of type `Item`.
pub trait Vector {
    /// The type that this vector contains.
    type Item;

    /// Return the number of items in `self`.
    fn count(&self) -> usize;
}

macro_rules! vector_impl {
    ($( $main: ident, $item: ty, $count: expr; )* ) => {
        $(
            impl Vector for simd::$main {
                type Item = $item;
                fn count(&self) -> usize {
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
