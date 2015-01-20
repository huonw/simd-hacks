unsafe impl ::Vector for u8 {
    type Item = u8; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for i8 {
    type Item = i8; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for u16 {
    type Item = u16; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for i16 {
    type Item = i16; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for ::simdty::u8x2 {
    type Item = u8; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::i8x2 {
    type Item = i8; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for u32 {
    type Item = u32; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for i32 {
    type Item = i32; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for f32 {
    type Item = f32; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for ::simdty::u16x2 {
    type Item = u16; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::i16x2 {
    type Item = i16; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::u8x4 {
    type Item = u8; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::i8x4 {
    type Item = i8; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for u64 {
    type Item = u64; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for i64 {
    type Item = i64; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for f64 {
    type Item = f64; #[inline(always)] fn count(_: Option<Self>) -> usize { 1 }
}
unsafe impl ::Vector for ::simdty::u32x2 {
    type Item = u32; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::i32x2 {
    type Item = i32; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::f32x2 {
    type Item = f32; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::u16x4 {
    type Item = u16; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::i16x4 {
    type Item = i16; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::u8x8 {
    type Item = u8; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::i8x8 {
    type Item = i8; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::u64x2 {
    type Item = u64; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::i64x2 {
    type Item = i64; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::f64x2 {
    type Item = f64; #[inline(always)] fn count(_: Option<Self>) -> usize { 2 }
}
unsafe impl ::Vector for ::simdty::u32x4 {
    type Item = u32; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::i32x4 {
    type Item = i32; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::f32x4 {
    type Item = f32; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::u16x8 {
    type Item = u16; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::i16x8 {
    type Item = i16; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::u8x16 {
    type Item = u8; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::i8x16 {
    type Item = i8; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::u64x4 {
    type Item = u64; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::i64x4 {
    type Item = i64; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::f64x4 {
    type Item = f64; #[inline(always)] fn count(_: Option<Self>) -> usize { 4 }
}
unsafe impl ::Vector for ::simdty::u32x8 {
    type Item = u32; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::i32x8 {
    type Item = i32; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::f32x8 {
    type Item = f32; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::u16x16 {
    type Item = u16; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::i16x16 {
    type Item = i16; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::u8x32 {
    type Item = u8; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::i8x32 {
    type Item = i8; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::u64x8 {
    type Item = u64; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::i64x8 {
    type Item = i64; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::f64x8 {
    type Item = f64; #[inline(always)] fn count(_: Option<Self>) -> usize { 8 }
}
unsafe impl ::Vector for ::simdty::u32x16 {
    type Item = u32; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::i32x16 {
    type Item = i32; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::f32x16 {
    type Item = f32; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::u16x32 {
    type Item = u16; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::i16x32 {
    type Item = i16; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::u8x64 {
    type Item = u8; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::i8x64 {
    type Item = i8; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::u64x16 {
    type Item = u64; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::i64x16 {
    type Item = i64; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::f64x16 {
    type Item = f64; #[inline(always)] fn count(_: Option<Self>) -> usize { 16 }
}
unsafe impl ::Vector for ::simdty::u32x32 {
    type Item = u32; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::i32x32 {
    type Item = i32; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::f32x32 {
    type Item = f32; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::u16x64 {
    type Item = u16; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::i16x64 {
    type Item = i16; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::u64x32 {
    type Item = u64; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::i64x32 {
    type Item = i64; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::f64x32 {
    type Item = f64; #[inline(always)] fn count(_: Option<Self>) -> usize { 32 }
}
unsafe impl ::Vector for ::simdty::u32x64 {
    type Item = u32; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::i32x64 {
    type Item = i32; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::f32x64 {
    type Item = f32; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::u64x64 {
    type Item = u64; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::i64x64 {
    type Item = i64; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
unsafe impl ::Vector for ::simdty::f64x64 {
    type Item = f64; #[inline(always)] fn count(_: Option<Self>) -> usize { 64 }
}
