use core::marker::PhantomData;
use num::NonMinusOneI32;

pub struct UniformLocation<T: ?Sized>(NonMinusOneI32, PhantomData<*const T>);

impl<T: ?Sized> UniformLocation<T> {
    pub unsafe fn from_raw(loc: i32) -> Option<Self> {
        NonMinusOneI32::new(loc).map(|n| UniformLocation(n, PhantomData))
    }

    pub unsafe fn as_i32(&self) -> i32 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_option_self_equals_size_of_u32() {
        use std::mem::size_of;
        assert_eq!(
            size_of::<Option<UniformLocation<[f32; 4]>>>(),
            size_of::<u32>()
        );
    }
}
