pub trait Array<T> {
    fn as_mut_slice(&mut self) -> &mut [T];
}

macro_rules! impl_array {
    ($($N:expr,)+) => {
        $(
            impl<T> Array<T> for [T; $N] {
                #[inline]
                fn as_mut_slice(&mut self) -> &mut [T] {
                    &mut self[..]
                }
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
