use num::NonMinusOneU32;

pub struct AttributeLocation(NonMinusOneU32);

impl AttributeLocation {
    pub unsafe fn from_raw(loc: i32) -> Option<Self> {
        NonMinusOneU32::new(loc as u32).map(AttributeLocation)
    }

    pub unsafe fn as_u32(&self) -> u32 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_option_self_equals_size_of_u32() {
        use std::mem::size_of;
        assert_eq!(
            size_of::<Option<AttributeLocation>>(),
            size_of::<u32>()
        );
    }
}
