impl ::maths::sqrt::Sqrt for f32 {
    #[inline(always)] fn sqrt(self) -> f32 {
        extern { #[link_name = "llvm.sqrt.f32"] fn sqrt(x: f32) -> f32; }
        unsafe {sqrt(self)}
    }
}
impl ::maths::sqrt::Sqrt for f64 {
    #[inline(always)] fn sqrt(self) -> f64 {
        extern { #[link_name = "llvm.sqrt.f64"] fn sqrt(x: f64) -> f64; }
        unsafe {sqrt(self)}
    }
}
impl ::maths::sqrt::Sqrt for ::simdty::f32x2 {
    #[inline(always)] fn sqrt(self) -> ::simdty::f32x2 {
        extern { #[link_name = "llvm.sqrt.v2f32"] fn sqrt(x: ::simdty::f32x2) -> ::simdty::f32x2; }
        unsafe {sqrt(self)}
    }
}
impl ::maths::sqrt::Sqrt for ::simdty::f64x2 {
    #[inline(always)] fn sqrt(self) -> ::simdty::f64x2 {
        extern { #[link_name = "llvm.sqrt.v2f64"] fn sqrt(x: ::simdty::f64x2) -> ::simdty::f64x2; }
        unsafe {sqrt(self)}
    }
}
impl ::maths::sqrt::Sqrt for ::simdty::f32x4 {
    #[inline(always)] fn sqrt(self) -> ::simdty::f32x4 {
        extern { #[link_name = "llvm.sqrt.v4f32"] fn sqrt(x: ::simdty::f32x4) -> ::simdty::f32x4; }
        unsafe {sqrt(self)}
    }
}
impl ::maths::sqrt::Sqrt for ::simdty::f64x4 {
    #[inline(always)] fn sqrt(self) -> ::simdty::f64x4 {
        extern { #[link_name = "llvm.sqrt.v4f64"] fn sqrt(x: ::simdty::f64x4) -> ::simdty::f64x4; }
        unsafe {sqrt(self)}
    }
}
impl ::maths::sqrt::Sqrt for ::simdty::f32x8 {
    #[inline(always)] fn sqrt(self) -> ::simdty::f32x8 {
        extern { #[link_name = "llvm.sqrt.v8f32"] fn sqrt(x: ::simdty::f32x8) -> ::simdty::f32x8; }
        unsafe {sqrt(self)}
    }
}
