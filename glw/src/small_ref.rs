use core::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr;

fn phantom_data_from<T>(_: T) -> PhantomData<T> {
    PhantomData
}

pub struct SmallRef<'a, T> {
    copy: ManuallyDrop<T>,
    _borrow: PhantomData<&'a T>,
}

impl<'a, T> SmallRef<'a, T> {
    #[inline]
    pub fn new(value: &'a T) -> Self {
        use std::mem::size_of;
        assert!(
            size_of::<T>() < size_of::<&T>(),
            "SmallRef only makes sense for types smaller than a pointer."
        );
        unsafe {
            SmallRef {
                copy: ManuallyDrop::new(ptr::read(value)),
                _borrow: phantom_data_from(value),
            }
        }
    }
}

impl<'a, T> std::ops::Deref for SmallRef<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &*self.copy
    }
}

#[cfg(test)]
mod tests {
    use super::SmallRef;

    #[test]
    fn it_is_indeed_smaller() {
        use std::mem::size_of;

        assert!(size_of::<SmallRef<u32>>() < size_of::<&u32>());
    }
}
