#![cfg(feature = "shims")]
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::u8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::u16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::u32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::u64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::i8x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::i16x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(target_arch = "x86_64",feature="sse2"))))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::i64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(target_arch = "x86_64",feature="sse2"))))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(target_arch = "x86_64",feature="sse2"))))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(target_arch = "x86_64",feature="sse2"))))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::u8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::u16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::u32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::u64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::i8x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::i16x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(target_arch = "x86_64",feature="sse2"))))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(feature="avx"))))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::i64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(target_arch = "x86_64",feature="sse2"))))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(feature="avx"))))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(feature="avx"))))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(feature="avx"))))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::u8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::u16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::u32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::u64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::i8x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::i16x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(feature="avx"))))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::i64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any(any(feature="avx"))))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x8> for ::simdty::f64x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::u8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::u16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::u32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::u64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::i8x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::i16x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::i32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::i64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::f32x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x16> for ::simdty::f64x16 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::u8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::u16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::u32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::u64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::i8x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::i16x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::i32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::i64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::f32x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x32> for ::simdty::f64x32 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::u8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::u16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::u32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::u64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::i8x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::i16x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::i32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::i64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::f32x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u8x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u16x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u32x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::u64x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::u64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::u64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i8x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i8x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i8x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i16x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i16x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i16x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i32x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::i64x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::i64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::i64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f32x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
#[cfg(not(any()))]
unsafe impl ::Convert<::simdty::f64x64> for ::simdty::f64x64 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert()) }
}
