use name::Name;
use std::mem;

#[derive(Debug, Eq, PartialEq)]
pub struct BufferName(Name);

impl BufferName {
    #[inline]
    pub unsafe fn from_raw(name: u32) -> Option<Self> {
        Name::new(name).map(BufferName)
    }

    #[inline]
    pub const unsafe fn from_raw_unchecked(name: u32) -> Self {
        BufferName(Name::new_unchecked(name))
    }

    #[inline]
    pub unsafe fn into_raw(self) -> u32 {
        mem::ManuallyDrop::new(self).as_u32()
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0.get()
    }
}

prevent_drop!(BufferName, forgot_to_explicitly_drop_BufferName);

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
}
