pub trait Array<T> {
    fn len(&self) -> usize;
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
}

pub trait SourceArray<'a>: Array<&'a [u8]> {
    type RawLengthArray: Array<i32>;
    type RawSourceArray: Array<*const u8>;
}

macro_rules! impl_array {
    ($($N:expr,)+) => {
        $(
            impl<T> Array<T> for [T; $N] {
                #[inline]
                fn len(&self) -> usize {
                    $N
                }

                #[inline]
                fn as_ptr(&self) -> *const T {
                    self as *const [T; $N] as *const T
                }

                #[inline]
                fn as_mut_ptr(&mut self) -> *mut T {
                    self as *mut [T; $N] as *mut T
                }

                #[inline]
                fn as_slice(&self) -> &[T] {
                    &self[..]
                }

                #[inline]
                fn as_mut_slice(&mut self) -> &mut [T] {
                    &mut self[..]
                }
            }

            impl<'a> SourceArray<'a> for [&'a [u8]; $N] {
                type RawLengthArray = [i32; $N];
                type RawSourceArray = [*const u8; $N];
            }
        )+
    };
}

impl_array! {
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32,
}
