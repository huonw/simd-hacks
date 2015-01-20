#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(any(target_arch = "x86_64",feature="sse2"))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { unsafe { ::llvmint::x86::sse2_cvtdq2ps(self) } }
}
#[cfg(any(feature="avx"))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { unsafe { ::llvmint::x86::avx_cvtdq2_pd_256(self) } }
}
#[cfg(any(target_arch = "x86_64",feature="sse2"))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { unsafe { ::llvmint::x86::sse2_cvtps2dq(self) } }
}
#[cfg(any(feature="avx"))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 { unsafe { ::llvmint::x86::avx_cvt_ps2_pd_256(self) } }
}
#[cfg(any(feature="avx"))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 { unsafe { ::llvmint::x86::avx_cvt_pd2dq_256(self) } }
}
#[cfg(any(feature="avx"))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 { unsafe { ::llvmint::x86::avx_cvt_pd2_ps_256(self) } }
}
#[cfg(any(feature="avx"))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 { unsafe { ::llvmint::x86::avx_cvtdq2_ps_256(self) } }
}
#[cfg(any(feature="avx"))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 { unsafe { ::llvmint::x86::avx_cvt_ps2dq_256(self) } }
}
