use gl;
use max_combined_texture_image_units::MaxCombinedTextureImageUnits;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TextureUnit(u32);

impl TextureUnit {
    #[inline]
    pub fn new(unit: u32, max: MaxCombinedTextureImageUnits) -> Option<Self> {
        if unit < max.get() {
            Some(TextureUnit(gl::TEXTURE0 + unit))
        } else {
            None
        }
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

pub const TEXTURE0: TextureUnit = TextureUnit(gl::TEXTURE0);
pub const TEXTURE1: TextureUnit = TextureUnit(gl::TEXTURE1);
pub const TEXTURE2: TextureUnit = TextureUnit(gl::TEXTURE2);
pub const TEXTURE3: TextureUnit = TextureUnit(gl::TEXTURE3);
pub const TEXTURE4: TextureUnit = TextureUnit(gl::TEXTURE4);
pub const TEXTURE5: TextureUnit = TextureUnit(gl::TEXTURE5);
pub const TEXTURE6: TextureUnit = TextureUnit(gl::TEXTURE6);
pub const TEXTURE7: TextureUnit = TextureUnit(gl::TEXTURE7);
pub const TEXTURE8: TextureUnit = TextureUnit(gl::TEXTURE8);
pub const TEXTURE9: TextureUnit = TextureUnit(gl::TEXTURE9);
pub const TEXTURE10: TextureUnit = TextureUnit(gl::TEXTURE10);
pub const TEXTURE11: TextureUnit = TextureUnit(gl::TEXTURE11);
pub const TEXTURE12: TextureUnit = TextureUnit(gl::TEXTURE12);
pub const TEXTURE13: TextureUnit = TextureUnit(gl::TEXTURE13);
pub const TEXTURE14: TextureUnit = TextureUnit(gl::TEXTURE14);
pub const TEXTURE15: TextureUnit = TextureUnit(gl::TEXTURE15);
