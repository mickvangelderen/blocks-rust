use functions;
use gl;
use name::Name;

pub struct VertexArrayName(Name);

impl VertexArrayName {
    pub unsafe fn new() -> Option<Self> {
        let mut names: [u32; 1] = ::std::mem::uninitialized();
        gl::GenVertexArrays(names.len() as i32, names.as_mut_ptr());
        Name::new(names[0]).map(VertexArrayName)
    }

    pub unsafe fn as_u32(&self) -> u32 {
        self.0.get()
    }
}

impl Drop for VertexArrayName {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.0 as *const Name as *const u32);
        }
    }
}

pub trait VertexArrayNameArray {
    unsafe fn new() -> Self;
}

macro_rules! impl_vertex_array_name_array {
    ($($N:expr)+) => {
        $(
            impl VertexArrayNameArray for [Option<VertexArrayName>; $N] {
                #[inline]
                unsafe fn new() -> Self {
                    let mut names: [Option<VertexArrayName>; $N] = ::std::mem::uninitialized();
                    functions::gen_vertex_arrays(&mut names);
                    names
                }
            }
        )+
    }
}

impl_vertex_array_name_array! {
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
        assert_eq!(size_of::<Option<VertexArrayName>>(), size_of::<u32>());
        assert_eq!(
            size_of::<[Option<VertexArrayName>; 32]>(),
            size_of::<[u32; 32]>()
        );
    }
}
