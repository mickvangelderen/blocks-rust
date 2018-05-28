use gl;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureTarget {
    Texture2D = gl::TEXTURE_2D,
    Texture2DArray = gl::TEXTURE_2D_ARRAY,
}

pub const TEXTURE_2D: TextureTarget = TextureTarget::Texture2D;
pub const TEXTURE_2D_ARRAY: TextureTarget = TextureTarget::Texture2DArray;
