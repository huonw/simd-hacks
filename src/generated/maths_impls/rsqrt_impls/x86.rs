#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse")))]
impl ::maths::sqrt::RSqrt for f32 {
    #[inline(always)] fn rsqrt(self) -> f32 {
        let in_ = unsafe {::DoubleVector::merge(::DoubleVector::merge(self, ::std::mem::uninitialized()), ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::sse_rsqrt_ps(in_)}}; ::HalfVector::lower(::HalfVector::lower(out)) }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse"))), any(feature="avx")))]
impl ::maths::sqrt::RSqrt for f32 {
    #[inline(always)] fn rsqrt(self) -> f32 {
        let in_ = unsafe {::DoubleVector::merge(::DoubleVector::merge(::DoubleVector::merge(self, ::std::mem::uninitialized()), ::std::mem::uninitialized()), ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_rsqrt_ps_256(in_)}}; ::HalfVector::lower(::HalfVector::lower(::HalfVector::lower(out))) }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x2 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::sse_rsqrt_ps(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse"))), any(feature="avx")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x2 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x2 {
        let in_ = unsafe {::DoubleVector::merge(::DoubleVector::merge(self, ::std::mem::uninitialized()), ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_rsqrt_ps_256(in_)}}; ::HalfVector::lower(::HalfVector::lower(out)) }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x4 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse_rsqrt_ps(in_)}}; out }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse"))), any(feature="avx")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x4 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x4 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_rsqrt_ps_256(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any()), any(feature="avx")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x8 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x8 {
        let in_ = self; let out = {unsafe{::llvmint::x86::avx_rsqrt_ps_256(in_)}}; out }
}
