use gl;
use texture_parameter::*;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureWrap {
    ClampToEdge = gl::CLAMP_TO_EDGE,
    ClampToBorder = gl::CLAMP_TO_BORDER,
    MirroredRepeat = gl::MIRRORED_REPEAT,
    Repeat = gl::REPEAT,
    MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE,
}

pub const CLAMP_TO_EDGE: TextureWrap = TextureWrap::ClampToEdge;
pub const CLAMP_TO_BORDER: TextureWrap = TextureWrap::ClampToBorder;
pub const MIRRORED_REPEAT: TextureWrap = TextureWrap::MirroredRepeat;
pub const REPEAT: TextureWrap = TextureWrap::Repeat;
pub const MIRROR_CLAMP_TO_EDGE: TextureWrap = TextureWrap::MirrorClampToEdge;

impl TextureParameterI32Value for TextureWrap {
    #[inline]
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureWrapKey {
    TextureWrapS = gl::TEXTURE_WRAP_S,
    TextureWrapT = gl::TEXTURE_WRAP_T,
    TextureWrapR = gl::TEXTURE_WRAP_R,
}

pub const TEXTURE_WRAP_S: TextureWrapKey = TextureWrapKey::TextureWrapS;
pub const TEXTURE_WRAP_T: TextureWrapKey = TextureWrapKey::TextureWrapT;
pub const TEXTURE_WRAP_R: TextureWrapKey = TextureWrapKey::TextureWrapR;

impl TextureParameterI32Key for TextureWrapKey {
    type Value = TextureWrap;

    #[inline]
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}
