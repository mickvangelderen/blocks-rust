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

pub unsafe fn bind_vertex_array(name: &VertexArrayName) {
    gl::BindVertexArray(name.as_u32());
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
