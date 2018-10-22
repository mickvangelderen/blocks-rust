use core::marker::PhantomData;
use std::ptr;

fn phantom_data_from<T>(_: T) -> PhantomData<T> {
    PhantomData
}

/// Promise that your type has no embedded/interior mutability.
/// https://users.rust-lang.org/t/references-to-values-smaller-than-references/21448/5
pub unsafe trait Freeze {}

pub struct Ref32<'a, T: 'a>
where
    T: Freeze,
{
    copy: [u8; 4],
    _borrow: PhantomData<&'a T>,
}

impl<'a, T: 'a> Ref32<'a, T>
where
    T: Freeze,
{
    #[inline]
    pub fn new(value: &'a T) -> Self {
        use std::mem::size_of;

        assert!(
            size_of::<T>() == size_of::<[u8; 4]>(),
            "Ref32 can only be implemented for types that are 32 bits in size."
        );

        assert!(
            size_of::<T>() < size_of::<&T>(),
            "Ref32 only makes sense for types smaller than a pointer."
        );

        unsafe {
            Ref32 {
                copy: ptr::read(value as *const T as *const [u8; 4]),
                _borrow: phantom_data_from(value),
            }
        }
    }
}

// NOTE: Won't derive.
impl<'a, T: 'a> Copy for Ref32<'a, T> where T: Freeze {}

// NOTE: Won't derive.
impl<'a, T: 'a> Clone for Ref32<'a, T>
where
    T: Freeze,
{
    #[inline]
    fn clone(&self) -> Self {
        Ref32 {
            copy: self.copy,
            _borrow: self._borrow,
        }
    }
}

impl<'a, T: 'a> std::ops::Deref for Ref32<'a, T>
where
    T: Freeze,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*(&self.copy as *const _ as *const T) }
    }
}

#[cfg(test)]
mod tests {
    use super::Freeze;
    use super::Ref32;

    struct Resource(u32);

    impl Resource {
        fn as_u32(&self) -> u32 {
            self.0
        }
    }

    unsafe impl Freeze for Resource {}

    #[test]
    fn it_is_indeed_smaller() {
        use std::mem::size_of;

        assert!(size_of::<Ref32<Resource>>() < size_of::<&Resource>());
    }

    #[test]
    fn can_copy_ref32() {
        let x = Resource(13);
        let r = Ref32::new(&x);
        let r2 = r;
        let r3 = r;
        assert_eq!(r2.as_u32(), 13);
        assert_eq!(r3.as_u32(), 13);
    }
}
