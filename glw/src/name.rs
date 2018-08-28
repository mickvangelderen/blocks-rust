use std::num::NonZeroU32;
use std::fmt;

#[derive(Eq, PartialEq)]
pub struct Name(NonZeroU32);

impl Name {
    #[inline]
    pub fn new(name: u32) -> Option<Self> {
        NonZeroU32::new(name).map(Name)
    }

    #[inline]
    pub const unsafe fn new_unchecked(name: u32) -> Self {
        Name(NonZeroU32::new_unchecked(name))
    }

    #[inline]
    pub fn get(&self) -> u32 {
        self.0.get()
    }
}

// Make Name transparant.
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn size_of_option_self_equals_size_of_u32() {
        assert_eq!(
            mem::size_of::<[Option<super::Name>; 32]>(),
            mem::size_of::<[u32; 32]>(),
        );
    }
}
