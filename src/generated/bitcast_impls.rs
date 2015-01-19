unsafe impl ::Bitcast<u8> for u8 {
    #[inline(always)] fn bitcast(self) -> u8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i8> for u8 {
    #[inline(always)] fn bitcast(self) -> i8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u8> for i8 {
    #[inline(always)] fn bitcast(self) -> u8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i8> for i8 {
    #[inline(always)] fn bitcast(self) -> i8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u16> for u16 {
    #[inline(always)] fn bitcast(self) -> u16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i16> for u16 {
    #[inline(always)] fn bitcast(self) -> i16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x2> for u16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x2> for u16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u16> for i16 {
    #[inline(always)] fn bitcast(self) -> u16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i16> for i16 {
    #[inline(always)] fn bitcast(self) -> i16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x2> for i16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x2> for i16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u16> for ::simdty::u8x2 {
    #[inline(always)] fn bitcast(self) -> u16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i16> for ::simdty::u8x2 {
    #[inline(always)] fn bitcast(self) -> i16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x2> for ::simdty::u8x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x2> for ::simdty::u8x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u16> for ::simdty::i8x2 {
    #[inline(always)] fn bitcast(self) -> u16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i16> for ::simdty::i8x2 {
    #[inline(always)] fn bitcast(self) -> i16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x2> for ::simdty::i8x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x2> for ::simdty::i8x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u32> for u32 {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i32> for u32 {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f32> for u32 {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x2> for u32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x2> for u32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x4> for u32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x4> for u32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for u32 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for u32 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u32> for i32 {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i32> for i32 {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f32> for i32 {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x2> for i32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x2> for i32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x4> for i32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x4> for i32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for i32 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for i32 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u32> for f32 {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i32> for f32 {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f32> for f32 {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x2> for f32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x2> for f32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x4> for f32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x4> for f32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for f32 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for f32 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u32> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i32> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f32> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x2> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x2> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x4> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x4> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for ::simdty::u16x2 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u32> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i32> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f32> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x2> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x2> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x4> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x4> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for ::simdty::i16x2 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u32> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i32> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f32> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x2> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x2> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x4> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x4> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for ::simdty::u8x4 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u32> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i32> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f32> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x2> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x2> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x4> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x4> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for ::simdty::i8x4 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<u32> for usize {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<i32> for usize {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<f32> for usize {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<::simdty::u16x2> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<::simdty::i16x2> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<::simdty::u8x4> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<::simdty::i8x4> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32", target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for usize {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32", target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for usize {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<u32> for isize {
    #[inline(always)] fn bitcast(self) -> u32 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<i32> for isize {
    #[inline(always)] fn bitcast(self) -> i32 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<f32> for isize {
    #[inline(always)] fn bitcast(self) -> f32 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<::simdty::u16x2> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<::simdty::i16x2> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<::simdty::u8x4> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32"))] unsafe impl ::Bitcast<::simdty::i8x4> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32", target_pointer_width="32"))] unsafe impl ::Bitcast<usize> for isize {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="32", target_pointer_width="32"))] unsafe impl ::Bitcast<isize> for isize {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for u64 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for u64 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for u64 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for u64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for u64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for u64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for u64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for u64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for u64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for u64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for u64 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for u64 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for i64 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for i64 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for i64 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for i64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for i64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for i64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for i64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for i64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for i64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for i64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for i64 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for i64 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for f64 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for f64 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for f64 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for f64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for f64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for f64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for f64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for f64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for f64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for f64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for f64 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for f64 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for ::simdty::u32x2 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for ::simdty::i32x2 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for ::simdty::f32x2 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for ::simdty::u16x4 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for ::simdty::i16x4 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for ::simdty::u8x8 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<u64> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<i64> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<f64> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x2> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x2> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x2> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x4> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x4> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x8> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x8> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for ::simdty::i8x8 {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<u64> for usize {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<i64> for usize {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<f64> for usize {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::u32x2> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::i32x2> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::f32x2> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::u16x4> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::i16x4> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::u8x8> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::i8x8> for usize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64", target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for usize {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64", target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for usize {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<u64> for isize {
    #[inline(always)] fn bitcast(self) -> u64 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<i64> for isize {
    #[inline(always)] fn bitcast(self) -> i64 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<f64> for isize {
    #[inline(always)] fn bitcast(self) -> f64 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::u32x2> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::i32x2> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::f32x2> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x2 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::u16x4> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::i16x4> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x4 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::u8x8> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64"))] unsafe impl ::Bitcast<::simdty::i8x8> for isize {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x8 { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64", target_pointer_width="64"))] unsafe impl ::Bitcast<usize> for isize {
    #[inline(always)] fn bitcast(self) -> usize { unsafe { ::std::mem::transmute(self) } }
}
#[cfg(all(target_pointer_width="64", target_pointer_width="64"))] unsafe impl ::Bitcast<isize> for isize {
    #[inline(always)] fn bitcast(self) -> isize { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::u64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::i64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::f64x2 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::u32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::i32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::f32x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::u16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::i16x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::u8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x2> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x2> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x2> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x2 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x4> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x4> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x4> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x8> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x8> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x16> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x16> for ::simdty::i8x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::u64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::i64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::f64x4 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::u32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::i32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::f32x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::u16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::i16x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::u8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x4> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x4> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x4> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x4 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x8> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x8> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x8> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x16> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x16> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x32> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x32> for ::simdty::i8x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::u64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::i64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::f64x8 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::u32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::i32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::f32x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::u16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::i16x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::u8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x8> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x8> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x8> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x8 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x16> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x16> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x16> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x32> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x32> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u8x64> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i8x64> for ::simdty::i8x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i8x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x16> for ::simdty::u64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x16> for ::simdty::u64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x16> for ::simdty::u64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x32> for ::simdty::u64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x32> for ::simdty::u64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x32> for ::simdty::u64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x64> for ::simdty::u64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x64> for ::simdty::u64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x16> for ::simdty::i64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x16> for ::simdty::i64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x16> for ::simdty::i64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x32> for ::simdty::i64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x32> for ::simdty::i64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x32> for ::simdty::i64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x64> for ::simdty::i64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x64> for ::simdty::i64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x16> for ::simdty::f64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x16> for ::simdty::f64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x16> for ::simdty::f64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x32> for ::simdty::f64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x32> for ::simdty::f64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x32> for ::simdty::f64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x64> for ::simdty::f64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x64> for ::simdty::f64x16 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x16> for ::simdty::u32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x16> for ::simdty::u32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x16> for ::simdty::u32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x32> for ::simdty::u32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x32> for ::simdty::u32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x32> for ::simdty::u32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x64> for ::simdty::u32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x64> for ::simdty::u32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x16> for ::simdty::i32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x16> for ::simdty::i32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x16> for ::simdty::i32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x32> for ::simdty::i32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x32> for ::simdty::i32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x32> for ::simdty::i32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x64> for ::simdty::i32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x64> for ::simdty::i32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x16> for ::simdty::f32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x16> for ::simdty::f32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x16> for ::simdty::f32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x32> for ::simdty::f32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x32> for ::simdty::f32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x32> for ::simdty::f32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x64> for ::simdty::f32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x64> for ::simdty::f32x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x16> for ::simdty::u16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x16> for ::simdty::u16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x16> for ::simdty::u16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x32> for ::simdty::u16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x32> for ::simdty::u16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x32> for ::simdty::u16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x64> for ::simdty::u16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x64> for ::simdty::u16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x16> for ::simdty::i16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x16> for ::simdty::i16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x16> for ::simdty::i16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x16 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x32> for ::simdty::i16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x32> for ::simdty::i16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x32> for ::simdty::i16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u16x64> for ::simdty::i16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i16x64> for ::simdty::i16x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i16x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x32> for ::simdty::u64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x32> for ::simdty::u64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x32> for ::simdty::u64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x64> for ::simdty::u64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x64> for ::simdty::u64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x64> for ::simdty::u64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x32> for ::simdty::i64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x32> for ::simdty::i64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x32> for ::simdty::i64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x64> for ::simdty::i64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x64> for ::simdty::i64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x64> for ::simdty::i64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x32> for ::simdty::f64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x32> for ::simdty::f64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x32> for ::simdty::f64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x64> for ::simdty::f64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x64> for ::simdty::f64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x64> for ::simdty::f64x32 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x32> for ::simdty::u32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x32> for ::simdty::u32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x32> for ::simdty::u32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x64> for ::simdty::u32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x64> for ::simdty::u32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x64> for ::simdty::u32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x32> for ::simdty::i32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x32> for ::simdty::i32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x32> for ::simdty::i32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x64> for ::simdty::i32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x64> for ::simdty::i32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x64> for ::simdty::i32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x32> for ::simdty::f32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x32> for ::simdty::f32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x32> for ::simdty::f32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x32 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u32x64> for ::simdty::f32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i32x64> for ::simdty::f32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f32x64> for ::simdty::f32x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f32x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x64> for ::simdty::u64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x64> for ::simdty::u64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x64> for ::simdty::u64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x64> for ::simdty::i64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x64> for ::simdty::i64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x64> for ::simdty::i64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::u64x64> for ::simdty::f64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::u64x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::i64x64> for ::simdty::f64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::i64x64 { unsafe { ::std::mem::transmute(self) } }
}
unsafe impl ::Bitcast<::simdty::f64x64> for ::simdty::f64x64 {
    #[inline(always)] fn bitcast(self) -> ::simdty::f64x64 { unsafe { ::std::mem::transmute(self) } }
}
