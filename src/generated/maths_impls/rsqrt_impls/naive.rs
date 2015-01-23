#![cfg(feature = "shims")]
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for f32 {
    #[inline(always)] fn rsqrt(self) -> f32 { 1.0 / ::maths::sqrt::Sqrt::sqrt(self)}
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for f64 {
    #[inline(always)] fn rsqrt(self) -> f64 { 1.0 / ::maths::sqrt::Sqrt::sqrt(self)}
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x2 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x2 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x2 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any(any(target_arch = "x86_64",feature="sse"))))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x4 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x4 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x4 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any(any(feature="avx"))))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x8 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x8 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x8 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x16 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x16 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x16 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x32 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x32 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x32 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x64 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x64 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x64 { let (a, b) = ::HalfVector::split(self); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }
}
