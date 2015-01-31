#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::sse2_cvtdq2ps(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse2"))), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 {
        let in_ = unsafe {::DoubleVector::merge(::DoubleVector::merge(self, ::std::mem::uninitialized()), ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_cvtdq2_ps_256(in_)}}; ::HalfVector::lower(::HalfVector::lower(out)) }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::sse2_cvtdq2pd(in_)}}; out }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse2"))), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::i32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_cvtdq2_pd_256(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::sse2_cvttps2dq(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse2"))), any(feature="avx")))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 {
        let in_ = unsafe {::DoubleVector::merge(::DoubleVector::merge(self, ::std::mem::uninitialized()), ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_cvtt_ps2dq_256(in_)}}; ::HalfVector::lower(::HalfVector::lower(out)) }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::sse2_cvtps2pd(in_)}}; out }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse2"))), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::f32x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_cvt_ps2_pd_256(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse2_cvttpd2dq(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse2"))), any(feature="avx")))]
unsafe impl ::Convert<::simdty::i32x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_cvtt_pd2dq_256(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse2_cvtpd2ps(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse2"))), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f32x2> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x2 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_cvt_pd2_ps_256(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse2_cvttpd2dq(in_)}}; out }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::f64x2 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse2_cvtpd2ps(in_)}}; out }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse2_cvtdq2pd(in_)}}; out }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse2_cvtdq2ps(in_)}}; out }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse2"))), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_cvtdq2_ps_256(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any()), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::i32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::avx_cvtdq2_pd_256(in_)}}; out }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::f64x2> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x2 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse2_cvtps2pd(in_)}}; out }
}
#[cfg(all(not(any()), any(target_arch = "x86_64",feature="sse2")))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::sse2_cvttps2dq(in_)}}; out }
}
#[cfg(all(not(any(any(target_arch = "x86_64",feature="sse2"))), any(feature="avx")))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 {
        let in_ = unsafe {::DoubleVector::merge(self, ::std::mem::uninitialized())}; let out = {unsafe{::llvmint::x86::avx_cvtt_ps2dq_256(in_)}}; ::HalfVector::lower(out) }
}
#[cfg(all(not(any()), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f64x4> for ::simdty::f32x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f64x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::avx_cvt_ps2_pd_256(in_)}}; out }
}
#[cfg(all(not(any()), any(feature="avx")))]
unsafe impl ::Convert<::simdty::i32x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::avx_cvtt_pd2dq_256(in_)}}; out }
}
#[cfg(all(not(any()), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f32x4> for ::simdty::f64x4 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x4 {
        let in_ = self; let out = {unsafe{::llvmint::x86::avx_cvt_pd2_ps_256(in_)}}; out }
}
#[cfg(all(not(any()), any(feature="avx")))]
unsafe impl ::Convert<::simdty::f32x8> for ::simdty::i32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::f32x8 {
        let in_ = self; let out = {unsafe{::llvmint::x86::avx_cvtdq2_ps_256(in_)}}; out }
}
#[cfg(all(not(any()), any(feature="avx")))]
unsafe impl ::Convert<::simdty::i32x8> for ::simdty::f32x8 {
    #[inline(always)] fn convert(self) -> ::simdty::i32x8 {
        let in_ = self; let out = {unsafe{::llvmint::x86::avx_cvtt_ps2dq_256(in_)}}; out }
}
