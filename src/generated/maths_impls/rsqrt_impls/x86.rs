#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x4 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x4 {
        (unsafe { ::llvmint::x86::sse_rsqrt_ps(self) })
    }
}
#[cfg(all(not(any()), any(feature="avx")))]
impl ::maths::sqrt::RSqrt for ::simdty::f32x8 {
    #[inline(always)] fn rsqrt(self) -> ::simdty::f32x8 {
        (unsafe { ::llvmint::x86::avx_rsqrt_ps_256(self) })
    }
}
