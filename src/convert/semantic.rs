use std::{mem, raw};
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
