use gl;
use name::Name;

pub struct BufferName(Name);

impl BufferName {
    #[inline]
    pub unsafe fn new(name: u32) -> Option<Self> {
        Name::new(name).map(BufferName)
    }

    #[inline]
    pub unsafe fn as_u32(&self) -> u32 {
        self.0.get()
    }
}

impl Drop for BufferName {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0 as *const Name as *const u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_option_self_equals_size_of_u32() {
        use std::mem::size_of;
        assert_eq!(size_of::<Option<BufferName>>(), size_of::<u32>());
        assert_eq!(
            size_of::<[Option<BufferName>; 32]>(),
            size_of::<[u32; 32]>()
        );
    }
}
