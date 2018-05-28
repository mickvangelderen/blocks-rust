use gl;

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
