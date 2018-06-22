use gl;
use texture_parameter::*;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureFilter {
    /// Returns the value of the texture element that is nearest (in
    /// Manhattan distance) to the specified texture coordinates.
    Nearest = gl::NEAREST,

    /// Returns the weighted average of the four texture elements that
    /// are closest to the specified texture coordinates. These can
    /// include items wrapped or repeated from other parts of a texture,
    /// depending on the values of GL_TEXTURE_WRAP_S and
    /// GL_TEXTURE_WRAP_T, and on the exact mapping.
    Linear = gl::LINEAR,

    /// Chooses the mipmap that most closely matches the size of the
    /// pixel being textured and uses the GL_NEAREST criterion (the
    /// texture element closest to the specified texture coordinates) to
    /// produce a texture value.
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,

    /// Chooses the mipmap that most closely matches the size of the
    /// pixel being textured and uses the GL_LINEAR criterion (a
    /// weighted average of the four texture elements that are closest
    /// to the specified texture coordinates) to produce a texture
    /// value.
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,

    /// Chooses the two mipmaps that most closely match the size of the
    /// pixel being textured and uses the GL_NEAREST criterion (the
    /// texture element closest to the specified texture coordinates )
    /// to produce a texture value from each mipmap. The final texture
    /// value is a weighted average of those two values.
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,

    /// Chooses the two mipmaps that most closely match the size of the
    /// pixel being textured and uses the GL_LINEAR criterion (a
    /// weighted average of the texture elements that are closest to the
    /// specified texture coordinates) to produce a texture value from
    /// each mipmap. The final texture value is a weighted average of
    /// those two values.
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
}

pub const NEAREST: TextureFilter = TextureFilter::Nearest;
pub const LINEAR: TextureFilter = TextureFilter::Linear;
pub const NEAREST_MIPMAP_NEAREST: TextureFilter = TextureFilter::NearestMipmapNearest;
pub const LINEAR_MIPMAP_NEAREST: TextureFilter = TextureFilter::LinearMipmapNearest;
pub const NEAREST_MIPMAP_LINEAR: TextureFilter = TextureFilter::NearestMipmapLinear;
pub const LINEAR_MIPMAP_LINEAR: TextureFilter = TextureFilter::LinearMipmapLinear;

impl TextureParameterI32Value for TextureFilter {
    #[inline]
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TextureFilterKey {
    TextureMinFilter = gl::TEXTURE_MIN_FILTER,
    TextureMagFilter = gl::TEXTURE_MAG_FILTER,
}

pub const TEXTURE_MIN_FILTER: TextureFilterKey = TextureFilterKey::TextureMinFilter;
pub const TEXTURE_MAG_FILTER: TextureFilterKey = TextureFilterKey::TextureMagFilter;

impl TextureParameterI32Key for TextureFilterKey {
    type Value = TextureFilter;

    #[inline]
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}
