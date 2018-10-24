use core::marker::PhantomData;
use std::ptr;
use marker::Freeze;

fn phantom_data_from<T>(_: T) -> PhantomData<T> {
    PhantomData
}

/// To enable implementing Copy, we need users of SmallRef to provide a
/// copyable type that can hold all valid bit patterns of `T`. With the
/// `Raw` trait we promise that we can take Self, store it in a Raw and
/// interpret an immutable reference to Raw as an immutable reference to
/// Self.
pub unsafe trait Raw: Freeze {
    type Raw: Freeze + Copy;
}

/// https://users.rust-lang.org/t/references-to-values-smaller-than-references/21448/5
pub struct SmallRef<'a, T: 'a>
where
    T: Raw,
{
    copy: T::Raw,
    _borrow: PhantomData<&'a T>,
}

impl<'a, T: 'a> SmallRef<'a, T>
where
    T: Raw,
{
    #[inline]
    pub fn new(value: &'a T) -> Self {
        use std::mem::size_of;

        assert!(size_of::<T>() == size_of::<T::Raw>());

        assert!(
            size_of::<T>() < size_of::<&T>(),
            "SmallRef only makes sense for types smaller than a pointer."
        );

        unsafe {
            SmallRef {
                copy: ptr::read(value as *const T as *const T::Raw),
                _borrow: phantom_data_from(value),
            }
        }
    }
}

// NOTE: Deriving Clone does not work for all types while it should.
impl<'a, T: 'a> Clone for SmallRef<'a, T>
where
    T: Raw,
{
    #[inline]
    fn clone(&self) -> Self {
        SmallRef {
            copy: self.copy,
            _borrow: self._borrow,
        }
    }
}

// NOTE: Deriving Copy does not work for all types while it should.
impl<'a, T: 'a> Copy for SmallRef<'a, T> where T: Raw {}

impl<'a, T: 'a> ::std::borrow::Borrow<T> for SmallRef<'a, T>
where
    T: Raw,
{
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<'a, T: 'a> ::std::ops::Deref for SmallRef<'a, T>
where
    T: Raw,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // Safe because we
        // 1. hold an immutable borrow,
        // 2. Self::Target has no interior mutability, and
        // 3. all valid bit patterns of T::Raw are valid for T.
        unsafe { &*(&self.copy as *const T::Raw as *const T) }
    }
}

#[cfg(test)]
mod tests {
    use super::Freeze;
    use super::Raw;
    use super::SmallRef;

    struct Resource(u32);

    impl Resource {
        fn as_u32(&self) -> u32 {
            self.0
        }
    }

    unsafe impl Freeze for Resource {}
    unsafe impl Raw for Resource {
        type Raw = u32;
    }

    #[test]
    fn it_is_indeed_smaller() {
        use std::mem::size_of;

        assert!(size_of::<SmallRef<Resource>>() < size_of::<&Resource>());
    }

    #[test]
    fn can_copy() {
        let x = Resource(13);
        let r = SmallRef::new(&x);
        let r2 = r;
        let r3 = r;
        assert_eq!(r2.as_u32(), 13);
        assert_eq!(r3.as_u32(), 13);
    }
}
