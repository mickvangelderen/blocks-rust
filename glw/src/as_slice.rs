pub trait AsSlice<T> {
    fn as_slice(&self) -> &[T];
}

impl<T> AsSlice<T> for T {
    #[inline]
    fn as_slice(&self) -> &[T] {
        unsafe { ::std::slice::from_raw_parts(self as *const T, 1) }
    }
}

impl<T> AsSlice<T> for [T] {
    #[inline]
    fn as_slice(&self) -> &[T] {
        self
    }
}
