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
