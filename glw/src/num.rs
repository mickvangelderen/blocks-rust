use std::num::NonZeroU32;

pub struct NonMinusOneI32(NonZeroU32);

impl NonMinusOneI32 {
    /// Create a non-minus-one without checking the value.
    ///
    /// # Safety
    ///
    /// The value must not be minus one.
    #[inline]
    pub const unsafe fn new_unchecked(n: i32) -> Self {
        NonMinusOneI32(NonZeroU32::new_unchecked((n + 1) as u32))
    }

    /// Create a non-minus-one if the given value is not minus one.
    #[inline]
    pub fn new(n: i32) -> Option<Self> {
        NonZeroU32::new((n + 1) as u32).map(NonMinusOneI32)
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub fn get(&self) -> i32 {
        (self.0.get()) as i32 - 1
    }
}

pub struct NonMinusOneU32(NonZeroU32);

impl NonMinusOneU32 {
    /// Create a non-minus-one without checking the value.
    ///
    /// # Safety
    ///
    /// The value must not be minus one.
    #[inline]
    pub const unsafe fn new_unchecked(n: u32) -> Self {
        NonMinusOneU32(NonZeroU32::new_unchecked(n.wrapping_add(1)))
    }

    /// Create a non-minus-one if the given value is not minus one.
    #[inline]
    pub fn new(n: u32) -> Option<Self> {
        NonZeroU32::new(n.wrapping_add(1)).map(NonMinusOneU32)
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub fn get(&self) -> u32 {
        self.0.get().wrapping_sub(1)
    }
}
