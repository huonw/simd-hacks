use std::{cmp, mem, raw};
use {Vector, Bitcast};

pub unsafe trait Convert<Out> {
    fn convert(self) -> Out;
}

unsafe impl<'a, In, Out> Convert<&'a [Out]> for &'a [In]
    where In: Vector, Out: Vector,
          <In as Vector>::Item: Bitcast<<Out as Vector>::Item> {
        #[inline]
        fn convert(self) -> &'a [Out] {
            // we need to ensure that the pointer has valid alignment
            // and that the slice has correct length to fit `Out`s
            // exactly; the first checks in each `assert` are
            // statically checkable, and so hopefully results in the
            // asserts being removed for "downcasts" (e.g. [u32x4] to
            // [u32]).
            assert!(mem::align_of::<In>() >= mem::align_of::<Out>() ||
                    self.as_ptr() as usize % mem::align_of::<Out>() == 0);
            assert!(mem::size_of::<In>() % mem::size_of::<Out>() == 0 ||
                    (self.len() * Vector::count(None::<In>)) % Vector::count(None::<Out>) == 0);

            unsafe {
                mem::transmute(raw::Slice {
                    data: self.as_ptr() as *mut Out,
                    len: self.len() * Vector::count(None::<In>) / Vector::count(None::<Out>)
                })
            }
        }
}
unsafe impl<'a, In, Out> Convert<&'a mut [Out]> for &'a mut [In]
    where In: Vector, Out: Vector,
          <In as Vector>::Item: Bitcast<<Out as Vector>::Item> {
        #[inline]
        fn convert(self) -> &'a mut [Out] {
            // we need to ensure that the pointer has valid alignment
            // and that the slice has correct length to fit `Out`s
            // exactly; the first checks in each `assert` are
            // statically checkable, and so hopefully results in the
            // asserts being removed for "downcasts" (e.g. [u32x4] to
            // [u32]).
            assert!(mem::align_of::<In>() >= mem::align_of::<Out>() ||
                    self.as_ptr() as usize % mem::align_of::<Out>() == 0);
            assert!(mem::size_of::<In>() % mem::size_of::<Out>() == 0 ||
                    (self.len() * Vector::count(None::<In>)) % Vector::count(None::<Out>) == 0);

            unsafe {
                mem::transmute(raw::Slice {
                    data: self.as_ptr() as *mut Out,
                    len: self.len() * Vector::count(None::<In>) / Vector::count(None::<Out>)
                })
            }
        }
}

/// A type that repesents a `&[U]` converted from a `&[T]`, where the
/// conversion may be forced to leave leading/trailing elements (due
/// to alignment/size mismatches).
pub struct Tails<'a,T:'a,U:'a> {
    pub start: &'a [T],
    pub middle: &'a [U],
    pub end: &'a [T],
}
/// A type that repesents a `&mut [U]` converted from a `&mut [T]`,
/// where the conversion may be forced to leave leading/trailing
/// elements (due to alignment/size mismatches).
pub struct TailsMut<'a,T:'a,U:'a> {
    pub start: &'a mut [T],
    pub middle: &'a mut [U],
    pub end: &'a mut [T],
}

unsafe impl<'a, In, Out> Convert<Tails<'a, In, Out>> for &'a [In]
    where In: Vector, Out: Vector,
          <In as Vector>::Item: Bitcast<<Out as Vector>::Item>
{
    #[inline]
    fn convert(self) -> Tails<'a,In,Out> {
        let isize = mem::size_of::<In>();
        let osize = mem::size_of::<Out>();
        let oalign = mem::align_of::<Out>();

        let start = self.as_ptr() as usize;
        let len = self.len();

        let offset = start % oalign;
        let slen = if offset == 0 {
            0
        } else {
            cmp::min(len, (oalign - offset) / isize)
        };
        let remaining = len - slen;
        let mlen_in_outs = remaining * isize / osize;
        let mlen_in_ins = mlen_in_outs * osize / isize;

        let (initial, tail) = self.split_at(slen);
        Tails {
            start: initial,
            middle: unsafe {mem::transmute(raw::Slice {
                data: tail.as_ptr() as *mut Out,
                len: mlen_in_outs,
            })},
            end: &tail[mlen_in_ins..]
        }
    }
}
unsafe impl<'a, In, Out> Convert<TailsMut<'a, In, Out>> for &'a mut [In]
    where In: Vector, Out: Vector,
          <In as Vector>::Item: Bitcast<<Out as Vector>::Item>
{
    #[inline]
    fn convert(self) -> TailsMut<'a,In,Out> {
        let isize = mem::size_of::<In>();
        let osize = mem::size_of::<Out>();
        let oalign = mem::align_of::<Out>();

        let start = self.as_ptr() as usize;
        let len = self.len();

        let offset = start % oalign;
        let slen = if offset == 0 {
            0
        } else {
            cmp::min(len, (oalign - offset) / isize)
        };
        let remaining = len - slen;

        let mlen_in_outs = remaining * isize / osize;
        let mlen_in_ins = mlen_in_outs * osize / isize;

        let (start, tail) = self.split_at_mut(slen);
        TailsMut {
            start: start,
            middle: unsafe {mem::transmute(raw::Slice {
                data: tail.as_ptr() as *mut Out,
                len: mlen_in_outs,
            })},
            end: &mut tail[mlen_in_ins..]
        }
    }
}
