use std::ops::{Add, Mul, Sub};

/// Requires two generics, the first one is the parameters type and the second the return type
pub(super) fn lerp<U, T>(a: U, b: U, t: U) -> T
where
    T: From<U>,
    U: Into<T>,
    U: Sub<U> + Mul<U>,
    U: Add<<<U as Sub>::Output as Mul<U>>::Output, Output = U>,
    <U as Sub>::Output: Mul<U>,
    U: Copy,
{
    T::from(a + (b - a) * t)
}
