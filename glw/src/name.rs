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
        return self.0.get();
    }
}
