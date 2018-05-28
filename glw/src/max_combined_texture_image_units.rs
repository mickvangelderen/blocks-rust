use gl;

#[derive(Clone, Copy)]
pub struct MaxCombinedTextureImageUnits(u32);

impl MaxCombinedTextureImageUnits {
    pub unsafe fn new() -> Self {
        MaxCombinedTextureImageUnits({
            let mut values: [i32; 1] = ::std::mem::uninitialized();
            gl::GetIntegerv(gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS, values.as_mut_ptr());
            values[0] as u32
        })
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}
