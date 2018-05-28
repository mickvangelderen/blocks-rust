use functions;
use gl;
use name::Name;

pub struct BufferName(Name);

impl BufferName {
    #[inline]
    pub unsafe fn new() -> Option<Self> {
        let [name]: [Option<BufferName>; 1] = BufferNameArray::new();
        name
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

pub trait BufferNameArray {
    unsafe fn new() -> Self;
}

macro_rules! impl_vertex_buffer_name_array {
    ($($N:expr)+) => {
        $(
            impl BufferNameArray for [Option<BufferName>; $N] {
                #[inline]
                unsafe fn new() -> Self {
                    let mut names: [Option<BufferName>; $N] = ::std::mem::uninitialized();
                    functions::gen_buffers(&mut names);
                    names
                }
            }
        )+
    }
}

impl_vertex_buffer_name_array! {
    0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
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
