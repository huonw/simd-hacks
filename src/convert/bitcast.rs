use std::mem;

/// Types between which a direct bitcast (`transmute`) is safe,
/// e.g. reinterpreting the 4 bytes of a `u32` as an `i32` is fine.
///
/// This can and will cause serious brokenness if `Self` and `Out` are
/// different sizes, and equally serious brokenness if there are
/// values of type `Self` where the bit pattern is not a valid value
/// of type `Out`.
///
/// NB. there's no static guarantee that pointers to bitcastable types
/// are themselves bitcastable, as the alignment may not be correct.
pub unsafe trait Bitcast<Out> {
    /// Bitcast `self` into a value of type `Out`.
    ///
    /// This *should* be the same as `transmute::<_, Out>(self)`, but
    /// is safe.
    fn bitcast(self) -> Out;
}

macro_rules! item { ($i: item) => {$i} }

macro_rules! mk_impl {
    ($in_: ty, $out: ty, [$($params: tt)*], [$($checks: expr),*]) => {
        item!{unsafe impl <$($params)*> Bitcast<$out> for $in_ {
            #[inline(always)]
            fn bitcast(self) -> $out {
                $($checks;)*
                unsafe { mem::transmute(self) }
            }
        }}
    }
}

// consider including dynamic assertions of pointer position here too:
mk_impl! {
    &'a [In], &'a [Out], ['a, In: Bitcast<Out>, Out],
    [assert!(mem::align_of::<In>() >= mem::align_of::<Out>())]
}
mk_impl! {
    &'a mut [In], &'a mut [Out], ['a, In: Bitcast<Out>, Out],
    [assert!(mem::align_of::<In>() >= mem::align_of::<Out>())]
}
mk_impl! {
    &'a In, &'a Out, ['a, In: Bitcast<Out>, Out],
    [assert!(mem::align_of::<In>() >= mem::align_of::<Out>())]
}
mk_impl! {
    &'a mut In, &'a mut Out, ['a, In: Bitcast<Out>, Out],
    [assert!(mem::align_of::<In>() >= mem::align_of::<Out>())]
}

mk_impl! {
    *const In, *const Out, [In, Out],
    [assert!(mem::align_of::<In>() >= mem::align_of::<Out>())]
}
mk_impl! {
    *mut In, *mut Out, [In, Out],
    [assert!(mem::align_of::<In>() >= mem::align_of::<Out>())]
}
