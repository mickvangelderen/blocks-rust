use std::mem::ManuallyDrop;
use std::num::NonZeroU32;
use small_ref;

macro_rules! impl_name {
    ($T:ident) => {
        #[derive(Debug, Eq, PartialEq)]
        pub struct $T(NonZeroU32);

        impl $T {
            #[inline]
            pub unsafe fn from_raw(name: u32) -> Option<Self> {
                NonZeroU32::new(name).map($T)
            }

            #[inline]
            pub const unsafe fn from_raw_unchecked(name: u32) -> Self {
                $T(NonZeroU32::new_unchecked(name))
            }

            #[inline]
            pub unsafe fn into_raw(self) -> u32 {
                ManuallyDrop::new(self).as_u32()
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

        // Promise every NonZeroU32 is a valid $T and vice versa.
        unsafe impl small_ref::Raw for $T {
            type Raw = NonZeroU32;
        }
    };
}

impl_name!(BufferName);
impl_name!(FramebufferName);
impl_name!(TextureName);
impl_name!(VertexArrayName);

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

// pub trait OptionBufferNameArray {
//     type BufferNameArray;

//     fn unwrap_all(self) -> Self::BufferNameArray;
// }

// macro_rules! array_impls {
//     (items { $($T:ty,)+ } sizes { $($N:expr,)+ }) => {
//         array_impls!(@repeat_items { $($T,)+ } @ { $($N,)+ });
//     };
//     (@repeat_items { $($T:ty,)+ } @ $NS:tt) => {
//         $(
//             array_impls!(@repeat_sizes { $T } @ $NS);
//         )+
//     };
//     (@repeat_sizes { $T:ty } @ { $($N:expr,)+ }) => {
//         $(
//             impl OptionBufferNameArray for ([Option<$T>; $N]) {
//                 type BufferNameArray = [$T; $N];
//                 #[inline]
//                 fn unwrap_all(self) -> Self::BufferNameArray {
//                     unsafe {
//                         for name in self.iter() {
//                             name.as_ref().unwrap();
//                         }

//                         ::std::mem::transmute(self)
//                     }
//                 }
//             }
//         )+
//     };
// }

// array_impls! {
//     items {
//         BufferName,
//         ManuallyDrop<BufferName>,
//     }
//     sizes {
//          0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
//         10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
//         20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
//         30, 31, 32,
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use super::small_ref;

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
