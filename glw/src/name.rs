use std::num::NonZeroU32;

#[derive(Debug)]
pub struct Name(NonZeroU32);

impl Name {
    #[inline]
    pub fn new(value: u32) -> Option<Self> {
        NonZeroU32::new(value).map(Name)
    }

    #[inline]
    pub fn get(&self) -> u32 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn size_of_option_self_equals_size_of_u32() {
        assert_eq!(
            ::std::mem::size_of::<[Option<super::Name>; 32]>(),
            ::std::mem::size_of::<[u32; 32]>(),
        );
    }
}
