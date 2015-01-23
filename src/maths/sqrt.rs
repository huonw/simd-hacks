//! Square roots and related functionality.

/// Square root operation.
///
/// NB. most platforms implement this as an approximation.
pub trait Sqrt {
    fn sqrt(self) -> Self;
}

/// Reciprocal-square root operation.
///
/// NB. most platforms implement this as an approximation.
pub trait RSqrt {
    fn rsqrt(self) -> Self;
}
