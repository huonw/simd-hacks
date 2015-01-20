unsafe impl ::DoubleVector for u8 {
    type Double = ::simdty::u8x2;
    #[inline(always)] fn merge(self, other: u8) -> ::simdty::u8x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for u16 {
    type Double = ::simdty::u16x2;
    #[inline(always)] fn merge(self, other: u16) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for u32 {
    type Double = ::simdty::u32x2;
    #[inline(always)] fn merge(self, other: u32) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for u64 {
    type Double = ::simdty::u64x2;
    #[inline(always)] fn merge(self, other: u64) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for i8 {
    type Double = ::simdty::i8x2;
    #[inline(always)] fn merge(self, other: i8) -> ::simdty::i8x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for i16 {
    type Double = ::simdty::i16x2;
    #[inline(always)] fn merge(self, other: i16) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for i32 {
    type Double = ::simdty::i32x2;
    #[inline(always)] fn merge(self, other: i32) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for i64 {
    type Double = ::simdty::i64x2;
    #[inline(always)] fn merge(self, other: i64) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for f32 {
    type Double = ::simdty::f32x2;
    #[inline(always)] fn merge(self, other: f32) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::DoubleVector for f64 {
    type Double = ::simdty::f64x2;
    #[inline(always)] fn merge(self, other: f64) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u8x2 {
    type Half = u8;
    #[inline(always)] fn split(self) -> (u8, u8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u8x2 {
    type Double = ::simdty::u8x4;
    #[inline(always)] fn merge(self, other: ::simdty::u8x2) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u16x2 {
    type Half = u16;
    #[inline(always)] fn split(self) -> (u16, u16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u16x2 {
    type Double = ::simdty::u16x4;
    #[inline(always)] fn merge(self, other: ::simdty::u16x2) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u32x2 {
    type Half = u32;
    #[inline(always)] fn split(self) -> (u32, u32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u32x2 {
    type Double = ::simdty::u32x4;
    #[inline(always)] fn merge(self, other: ::simdty::u32x2) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u64x2 {
    type Half = u64;
    #[inline(always)] fn split(self) -> (u64, u64) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u64x2 {
    type Double = ::simdty::u64x4;
    #[inline(always)] fn merge(self, other: ::simdty::u64x2) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i8x2 {
    type Half = i8;
    #[inline(always)] fn split(self) -> (i8, i8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i8x2 {
    type Double = ::simdty::i8x4;
    #[inline(always)] fn merge(self, other: ::simdty::i8x2) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i16x2 {
    type Half = i16;
    #[inline(always)] fn split(self) -> (i16, i16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i16x2 {
    type Double = ::simdty::i16x4;
    #[inline(always)] fn merge(self, other: ::simdty::i16x2) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i32x2 {
    type Half = i32;
    #[inline(always)] fn split(self) -> (i32, i32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i32x2 {
    type Double = ::simdty::i32x4;
    #[inline(always)] fn merge(self, other: ::simdty::i32x2) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i64x2 {
    type Half = i64;
    #[inline(always)] fn split(self) -> (i64, i64) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i64x2 {
    type Double = ::simdty::i64x4;
    #[inline(always)] fn merge(self, other: ::simdty::i64x2) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f32x2 {
    type Half = f32;
    #[inline(always)] fn split(self) -> (f32, f32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f32x2 {
    type Double = ::simdty::f32x4;
    #[inline(always)] fn merge(self, other: ::simdty::f32x2) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f64x2 {
    type Half = f64;
    #[inline(always)] fn split(self) -> (f64, f64) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f64x2 {
    type Double = ::simdty::f64x4;
    #[inline(always)] fn merge(self, other: ::simdty::f64x2) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u8x4 {
    type Half = ::simdty::u8x2;
    #[inline(always)] fn split(self) -> (::simdty::u8x2, ::simdty::u8x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u8x4 {
    type Double = ::simdty::u8x8;
    #[inline(always)] fn merge(self, other: ::simdty::u8x4) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u16x4 {
    type Half = ::simdty::u16x2;
    #[inline(always)] fn split(self) -> (::simdty::u16x2, ::simdty::u16x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u16x4 {
    type Double = ::simdty::u16x8;
    #[inline(always)] fn merge(self, other: ::simdty::u16x4) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u32x4 {
    type Half = ::simdty::u32x2;
    #[inline(always)] fn split(self) -> (::simdty::u32x2, ::simdty::u32x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u32x4 {
    type Double = ::simdty::u32x8;
    #[inline(always)] fn merge(self, other: ::simdty::u32x4) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u64x4 {
    type Half = ::simdty::u64x2;
    #[inline(always)] fn split(self) -> (::simdty::u64x2, ::simdty::u64x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u64x4 {
    type Double = ::simdty::u64x8;
    #[inline(always)] fn merge(self, other: ::simdty::u64x4) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i8x4 {
    type Half = ::simdty::i8x2;
    #[inline(always)] fn split(self) -> (::simdty::i8x2, ::simdty::i8x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i8x4 {
    type Double = ::simdty::i8x8;
    #[inline(always)] fn merge(self, other: ::simdty::i8x4) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i16x4 {
    type Half = ::simdty::i16x2;
    #[inline(always)] fn split(self) -> (::simdty::i16x2, ::simdty::i16x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i16x4 {
    type Double = ::simdty::i16x8;
    #[inline(always)] fn merge(self, other: ::simdty::i16x4) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i32x4 {
    type Half = ::simdty::i32x2;
    #[inline(always)] fn split(self) -> (::simdty::i32x2, ::simdty::i32x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i32x4 {
    type Double = ::simdty::i32x8;
    #[inline(always)] fn merge(self, other: ::simdty::i32x4) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i64x4 {
    type Half = ::simdty::i64x2;
    #[inline(always)] fn split(self) -> (::simdty::i64x2, ::simdty::i64x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i64x4 {
    type Double = ::simdty::i64x8;
    #[inline(always)] fn merge(self, other: ::simdty::i64x4) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f32x4 {
    type Half = ::simdty::f32x2;
    #[inline(always)] fn split(self) -> (::simdty::f32x2, ::simdty::f32x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f32x4 {
    type Double = ::simdty::f32x8;
    #[inline(always)] fn merge(self, other: ::simdty::f32x4) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f64x4 {
    type Half = ::simdty::f64x2;
    #[inline(always)] fn split(self) -> (::simdty::f64x2, ::simdty::f64x2) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f64x4 {
    type Double = ::simdty::f64x8;
    #[inline(always)] fn merge(self, other: ::simdty::f64x4) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u8x8 {
    type Half = ::simdty::u8x4;
    #[inline(always)] fn split(self) -> (::simdty::u8x4, ::simdty::u8x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u8x8 {
    type Double = ::simdty::u8x16;
    #[inline(always)] fn merge(self, other: ::simdty::u8x8) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u16x8 {
    type Half = ::simdty::u16x4;
    #[inline(always)] fn split(self) -> (::simdty::u16x4, ::simdty::u16x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u16x8 {
    type Double = ::simdty::u16x16;
    #[inline(always)] fn merge(self, other: ::simdty::u16x8) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u32x8 {
    type Half = ::simdty::u32x4;
    #[inline(always)] fn split(self) -> (::simdty::u32x4, ::simdty::u32x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u32x8 {
    type Double = ::simdty::u32x16;
    #[inline(always)] fn merge(self, other: ::simdty::u32x8) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u64x8 {
    type Half = ::simdty::u64x4;
    #[inline(always)] fn split(self) -> (::simdty::u64x4, ::simdty::u64x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u64x8 {
    type Double = ::simdty::u64x16;
    #[inline(always)] fn merge(self, other: ::simdty::u64x8) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i8x8 {
    type Half = ::simdty::i8x4;
    #[inline(always)] fn split(self) -> (::simdty::i8x4, ::simdty::i8x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i8x8 {
    type Double = ::simdty::i8x16;
    #[inline(always)] fn merge(self, other: ::simdty::i8x8) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i16x8 {
    type Half = ::simdty::i16x4;
    #[inline(always)] fn split(self) -> (::simdty::i16x4, ::simdty::i16x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i16x8 {
    type Double = ::simdty::i16x16;
    #[inline(always)] fn merge(self, other: ::simdty::i16x8) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i32x8 {
    type Half = ::simdty::i32x4;
    #[inline(always)] fn split(self) -> (::simdty::i32x4, ::simdty::i32x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i32x8 {
    type Double = ::simdty::i32x16;
    #[inline(always)] fn merge(self, other: ::simdty::i32x8) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i64x8 {
    type Half = ::simdty::i64x4;
    #[inline(always)] fn split(self) -> (::simdty::i64x4, ::simdty::i64x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i64x8 {
    type Double = ::simdty::i64x16;
    #[inline(always)] fn merge(self, other: ::simdty::i64x8) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f32x8 {
    type Half = ::simdty::f32x4;
    #[inline(always)] fn split(self) -> (::simdty::f32x4, ::simdty::f32x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f32x8 {
    type Double = ::simdty::f32x16;
    #[inline(always)] fn merge(self, other: ::simdty::f32x8) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f64x8 {
    type Half = ::simdty::f64x4;
    #[inline(always)] fn split(self) -> (::simdty::f64x4, ::simdty::f64x4) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f64x8 {
    type Double = ::simdty::f64x16;
    #[inline(always)] fn merge(self, other: ::simdty::f64x8) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u8x16 {
    type Half = ::simdty::u8x8;
    #[inline(always)] fn split(self) -> (::simdty::u8x8, ::simdty::u8x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u8x16 {
    type Double = ::simdty::u8x32;
    #[inline(always)] fn merge(self, other: ::simdty::u8x16) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u16x16 {
    type Half = ::simdty::u16x8;
    #[inline(always)] fn split(self) -> (::simdty::u16x8, ::simdty::u16x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u16x16 {
    type Double = ::simdty::u16x32;
    #[inline(always)] fn merge(self, other: ::simdty::u16x16) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u32x16 {
    type Half = ::simdty::u32x8;
    #[inline(always)] fn split(self) -> (::simdty::u32x8, ::simdty::u32x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u32x16 {
    type Double = ::simdty::u32x32;
    #[inline(always)] fn merge(self, other: ::simdty::u32x16) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u64x16 {
    type Half = ::simdty::u64x8;
    #[inline(always)] fn split(self) -> (::simdty::u64x8, ::simdty::u64x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u64x16 {
    type Double = ::simdty::u64x32;
    #[inline(always)] fn merge(self, other: ::simdty::u64x16) -> ::simdty::u64x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i8x16 {
    type Half = ::simdty::i8x8;
    #[inline(always)] fn split(self) -> (::simdty::i8x8, ::simdty::i8x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i8x16 {
    type Double = ::simdty::i8x32;
    #[inline(always)] fn merge(self, other: ::simdty::i8x16) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i16x16 {
    type Half = ::simdty::i16x8;
    #[inline(always)] fn split(self) -> (::simdty::i16x8, ::simdty::i16x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i16x16 {
    type Double = ::simdty::i16x32;
    #[inline(always)] fn merge(self, other: ::simdty::i16x16) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i32x16 {
    type Half = ::simdty::i32x8;
    #[inline(always)] fn split(self) -> (::simdty::i32x8, ::simdty::i32x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i32x16 {
    type Double = ::simdty::i32x32;
    #[inline(always)] fn merge(self, other: ::simdty::i32x16) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i64x16 {
    type Half = ::simdty::i64x8;
    #[inline(always)] fn split(self) -> (::simdty::i64x8, ::simdty::i64x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i64x16 {
    type Double = ::simdty::i64x32;
    #[inline(always)] fn merge(self, other: ::simdty::i64x16) -> ::simdty::i64x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f32x16 {
    type Half = ::simdty::f32x8;
    #[inline(always)] fn split(self) -> (::simdty::f32x8, ::simdty::f32x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f32x16 {
    type Double = ::simdty::f32x32;
    #[inline(always)] fn merge(self, other: ::simdty::f32x16) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f64x16 {
    type Half = ::simdty::f64x8;
    #[inline(always)] fn split(self) -> (::simdty::f64x8, ::simdty::f64x8) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f64x16 {
    type Double = ::simdty::f64x32;
    #[inline(always)] fn merge(self, other: ::simdty::f64x16) -> ::simdty::f64x32 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u8x32 {
    type Half = ::simdty::u8x16;
    #[inline(always)] fn split(self) -> (::simdty::u8x16, ::simdty::u8x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u8x32 {
    type Double = ::simdty::u8x64;
    #[inline(always)] fn merge(self, other: ::simdty::u8x32) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u16x32 {
    type Half = ::simdty::u16x16;
    #[inline(always)] fn split(self) -> (::simdty::u16x16, ::simdty::u16x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u16x32 {
    type Double = ::simdty::u16x64;
    #[inline(always)] fn merge(self, other: ::simdty::u16x32) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u32x32 {
    type Half = ::simdty::u32x16;
    #[inline(always)] fn split(self) -> (::simdty::u32x16, ::simdty::u32x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u32x32 {
    type Double = ::simdty::u32x64;
    #[inline(always)] fn merge(self, other: ::simdty::u32x32) -> ::simdty::u32x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u64x32 {
    type Half = ::simdty::u64x16;
    #[inline(always)] fn split(self) -> (::simdty::u64x16, ::simdty::u64x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::u64x32 {
    type Double = ::simdty::u64x64;
    #[inline(always)] fn merge(self, other: ::simdty::u64x32) -> ::simdty::u64x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i8x32 {
    type Half = ::simdty::i8x16;
    #[inline(always)] fn split(self) -> (::simdty::i8x16, ::simdty::i8x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i8x32 {
    type Double = ::simdty::i8x64;
    #[inline(always)] fn merge(self, other: ::simdty::i8x32) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i16x32 {
    type Half = ::simdty::i16x16;
    #[inline(always)] fn split(self) -> (::simdty::i16x16, ::simdty::i16x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i16x32 {
    type Double = ::simdty::i16x64;
    #[inline(always)] fn merge(self, other: ::simdty::i16x32) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i32x32 {
    type Half = ::simdty::i32x16;
    #[inline(always)] fn split(self) -> (::simdty::i32x16, ::simdty::i32x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i32x32 {
    type Double = ::simdty::i32x64;
    #[inline(always)] fn merge(self, other: ::simdty::i32x32) -> ::simdty::i32x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::i64x32 {
    type Half = ::simdty::i64x16;
    #[inline(always)] fn split(self) -> (::simdty::i64x16, ::simdty::i64x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::i64x32 {
    type Double = ::simdty::i64x64;
    #[inline(always)] fn merge(self, other: ::simdty::i64x32) -> ::simdty::i64x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f32x32 {
    type Half = ::simdty::f32x16;
    #[inline(always)] fn split(self) -> (::simdty::f32x16, ::simdty::f32x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f32x32 {
    type Double = ::simdty::f32x64;
    #[inline(always)] fn merge(self, other: ::simdty::f32x32) -> ::simdty::f32x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::f64x32 {
    type Half = ::simdty::f64x16;
    #[inline(always)] fn split(self) -> (::simdty::f64x16, ::simdty::f64x16) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::DoubleVector for ::simdty::f64x32 {
    type Double = ::simdty::f64x64;
    #[inline(always)] fn merge(self, other: ::simdty::f64x32) -> ::simdty::f64x64 { unsafe { ::std::mem::transmute((self, other)) } }
}
unsafe impl ::HalfVector for ::simdty::u8x64 {
    type Half = ::simdty::u8x32;
    #[inline(always)] fn split(self) -> (::simdty::u8x32, ::simdty::u8x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::u16x64 {
    type Half = ::simdty::u16x32;
    #[inline(always)] fn split(self) -> (::simdty::u16x32, ::simdty::u16x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::u32x64 {
    type Half = ::simdty::u32x32;
    #[inline(always)] fn split(self) -> (::simdty::u32x32, ::simdty::u32x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::u64x64 {
    type Half = ::simdty::u64x32;
    #[inline(always)] fn split(self) -> (::simdty::u64x32, ::simdty::u64x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::i8x64 {
    type Half = ::simdty::i8x32;
    #[inline(always)] fn split(self) -> (::simdty::i8x32, ::simdty::i8x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::i16x64 {
    type Half = ::simdty::i16x32;
    #[inline(always)] fn split(self) -> (::simdty::i16x32, ::simdty::i16x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::i32x64 {
    type Half = ::simdty::i32x32;
    #[inline(always)] fn split(self) -> (::simdty::i32x32, ::simdty::i32x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::i64x64 {
    type Half = ::simdty::i64x32;
    #[inline(always)] fn split(self) -> (::simdty::i64x32, ::simdty::i64x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::f32x64 {
    type Half = ::simdty::f32x32;
    #[inline(always)] fn split(self) -> (::simdty::f32x32, ::simdty::f32x32) { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::HalfVector for ::simdty::f64x64 {
    type Half = ::simdty::f64x32;
    #[inline(always)] fn split(self) -> (::simdty::f64x32, ::simdty::f64x32) { unsafe { ::std::mem::transmute(self) } }
}
