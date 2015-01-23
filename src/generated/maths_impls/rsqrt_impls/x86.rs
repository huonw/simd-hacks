#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x4 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse_rsqrt_ps(in_)}}; out }
}
#[cfg(all(not(any()), any(feature="avx")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x8 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x8 {
        let in_ = self; let out = {unsafe{::llvmint::x86::avx_rsqrt_ps_256(in_)}}; out }
}
