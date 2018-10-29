#[macro_export]
macro_rules! impl_name_arrays {
    ($T:ident, $TA:ident, $OTA:ident, $($N:expr,)+) => {
        $(
            impl $TA for [$T; $N] {
                type $OTA = [Option<$T>; $N];

                #[inline]
                fn wrap_all(self) -> Self::$OTA {
                    // Safe because:
                    // 1. every BufferName is a valid Option<BufferName>.
                    unsafe {
                        ::std::mem::transmute(self)
                    }
                }
            }

            impl $OTA for [Option<$T>; $N] {
                type $TA = [$T; $N];

                #[inline]
                fn unwrap_all(self) -> Option<Self::$TA> {
                    // Safe because:
                    // 1. we ensure all names are Some,
                    // 2. every Some<BufferName> is a valid BufferName.
                    unsafe {
                        for name in self.iter() {
                            if name.is_none() {
                                return None
                            }
                        }

                        Some(::std::mem::transmute(self))
                    }
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_name {
    ($T:ident) => {
        #[derive(Debug, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct $T(::std::num::NonZeroU32);

        impl $T {
            #[inline]
            pub unsafe fn from_raw(name: u32) -> Option<Self> {
                ::std::num::NonZeroU32::new(name).map($T)
            }

            #[inline]
            pub const unsafe fn from_raw_unchecked(name: u32) -> Self {
                $T(::std::num::NonZeroU32::new_unchecked(name))
            }

            #[inline]
            pub unsafe fn into_raw(self) -> u32 {
                ::std::mem::ManuallyDrop::new(self).as_u32()
            }

            #[inline]
            pub fn as_u32(&self) -> u32 {
                self.0.get()
            }
        }

        impl Drop for $T {
            #[inline(never)]
            #[cold]
            fn drop(&mut self) {
                if ::std::thread::panicking() == false {
                    ::std::process::abort();
                }
            }
        }
    };
    ($T:ident, $TA:ident, $OTA:ident) => {
        $crate::impl_name!($T);

        pub trait $TA {
            type $OTA;

            fn wrap_all(self) -> Self::$OTA;
        }

        pub trait $OTA {
            type $TA;

            fn unwrap_all(self) -> Option<Self::$TA>;
        }
        $crate::impl_name_arrays! {
            $T, $TA, $OTA,
            0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
            10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32,
        }

        // Promise every NonZeroU32 is a valid $T and vice versa.
        unsafe impl $crate::small_ref::Raw for $T {
            type Raw = ::std::num::NonZeroU32;
        }
    };
}

impl_name!(BufferName, BufferNameArray, OptionBufferNameArray);
impl_name!(FramebufferName, FramebufferNameArray, OptionFramebufferNameArray);
impl_name!(TextureName, TextureNameArray, OptionTextureNameArray);
impl_name!(VertexArrayName, VertexArrayNameArray, OptionVertexArrayNameArray);

pub struct DefaultFramebufferName();

pub const DEFAULT_FRAMEBUFFER_NAME: DefaultFramebufferName = DefaultFramebufferName();

impl DefaultFramebufferName {
    #[inline]
    pub fn as_u32(&self) -> u32 {
        0
    }
}

pub trait MaybeDefaultFramebufferName: seal::MaybeDefaultFramebufferName {
    fn as_u32(&self) -> u32;
}

impl MaybeDefaultFramebufferName for DefaultFramebufferName {
    #[inline]
    fn as_u32(&self) -> u32 {
        DefaultFramebufferName::as_u32(self)
    }
}

impl MaybeDefaultFramebufferName for FramebufferName {
    #[inline]
    fn as_u32(&self) -> u32 {
        FramebufferName::as_u32(self)
    }
}

mod seal {
    pub trait MaybeDefaultFramebufferName {}
    impl MaybeDefaultFramebufferName for super::DefaultFramebufferName {}
    impl MaybeDefaultFramebufferName for super::FramebufferName {}
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::small_ref;

    type BufferNameRef<'a> = small_ref::SmallRef<'a, BufferName>;

    use std::mem;

    #[test]
    fn option_buffer_name_is_a_u32() {
        unsafe {
            // Ensure None is encoded as 0u32.
            assert_eq!(
                mem::transmute::<Option<BufferName>, u32>(BufferName::from_raw(0)),
                0
            );

            // Ensure Some(BufferName(1)) is encoded as 1u32.
            assert_eq!(
                mem::transmute::<Option<BufferName>, u32>(BufferName::from_raw(1)),
                1
            );
        }
    }

    #[test]
    fn option_buffer_name_ref_is_a_u32() {
        // Assert size.
        let _ = mem::transmute::<Option<BufferNameRef>, u32>;

        unsafe {
            let b1 = BufferName::from_raw(1).unwrap();

            {
                // Can create multiple references.
                let b1r1 = BufferNameRef::new(&b1);
                let b1r2 = BufferNameRef::new(&b1);

                // Can copy references.
                let b1r3 = b1r1;
                let b1r4 = b1r1;

                assert_eq!(b1r1.as_u32(), 1);
                assert_eq!(b1r2.as_u32(), 1);
                assert_eq!(b1r3.as_u32(), 1);
                assert_eq!(b1r4.as_u32(), 1);
            }

            ::std::mem::forget(b1);
        }
    }
}
