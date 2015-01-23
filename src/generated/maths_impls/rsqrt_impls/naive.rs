#![cfg(feature = "shims")]
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for f32 {
    #[inline(always)] fn rsqrt(self) -> f32 {
        let in_ = self; let out = { 1.0 / ::maths::sqrt::Sqrt::sqrt(in_)}; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for f64 {
    #[inline(always)] fn rsqrt(self) -> f64 {
        let in_ = self; let out = { 1.0 / ::maths::sqrt::Sqrt::sqrt(in_)}; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x2 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x2 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f32x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x2 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x2 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f64x2 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any(any(target_arch = "x86_64",feature="sse"))))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x4 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x4 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f32x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x4 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x4 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f64x4 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any(any(feature="avx"))))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x8 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x8 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f32x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x8 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x8 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f64x8 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x16 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x16 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f32x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x16 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x16 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f64x16 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x32 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x32 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f32x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x32 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x32 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f64x32 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x64 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x64 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f32x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
#[cfg(not(any()))]
impl ::maths::sqrt::RSqrt for ::simdty::f64x64 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f64x64 {
        let in_ = self; let out = { let (a, b) = ::HalfVector::split(in_); <<::simdty::f64x64 as ::HalfVector>::Half as ::DoubleVector>::merge(a.rsqrt(), b.rsqrt()) }; out }
}
