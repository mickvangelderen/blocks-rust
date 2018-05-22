use super::name::Name;
// use std::marker::PhantomData;
// use phantomdata::into_phantom_data;
use gl;

#[derive(Debug)]
pub struct TextureName(Name);

impl TextureName {
    #[inline]
    pub unsafe fn as_u32(&self) -> u32 {
        (self.0).get()
    }
}

impl Drop for TextureName {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.as_u32());
        }
    }
}

#[repr(u32)]
pub enum TextureTarget {
    TEXTURE_2D = gl::TEXTURE_2D,
    TEXTURE_2D_ARRAY = gl::TEXTURE_2D_ARRAY,
}

pub const TEXTURE_2D: TextureTarget = TextureTarget::TEXTURE_2D;
pub const TEXTURE_2D_ARRAY: TextureTarget = TextureTarget::TEXTURE_2D_ARRAY;

#[repr(u32)]
pub enum TextureFilter {
    /// Returns the value of the texture element that is nearest (in
    /// Manhattan distance) to the specified texture coordinates.
    NEAREST = gl::NEAREST,

    /// Returns the weighted average of the four texture elements that
    /// are closest to the specified texture coordinates. These can
    /// include items wrapped or repeated from other parts of a texture,
    /// depending on the values of GL_TEXTURE_WRAP_S and
    /// GL_TEXTURE_WRAP_T, and on the exact mapping.
    LINEAR = gl::LINEAR,

    /// Chooses the mipmap that most closely matches the size of the
    /// pixel being textured and uses the GL_NEAREST criterion (the
    /// texture element closest to the specified texture coordinates) to
    /// produce a texture value.
    NEAREST_MIPMAP_NEAREST = gl::NEAREST_MIPMAP_NEAREST,

    /// Chooses the mipmap that most closely matches the size of the
    /// pixel being textured and uses the GL_LINEAR criterion (a
    /// weighted average of the four texture elements that are closest
    /// to the specified texture coordinates) to produce a texture
    /// value.
    LINEAR_MIPMAP_NEAREST = gl::LINEAR_MIPMAP_NEAREST,

    /// Chooses the two mipmaps that most closely match the size of the
    /// pixel being textured and uses the GL_NEAREST criterion (the
    /// texture element closest to the specified texture coordinates )
    /// to produce a texture value from each mipmap. The final texture
    /// value is a weighted average of those two values.
    NEAREST_MIPMAP_LINEAR = gl::NEAREST_MIPMAP_LINEAR,

    /// Chooses the two mipmaps that most closely match the size of the
    /// pixel being textured and uses the GL_LINEAR criterion (a
    /// weighted average of the texture elements that are closest to the
    /// specified texture coordinates) to produce a texture value from
    /// each mipmap. The final texture value is a weighted average of
    /// those two values.
    LINEAR_MIPMAP_LINEAR = gl::LINEAR_MIPMAP_LINEAR,
}

pub const NEAREST: TextureFilter = TextureFilter::NEAREST;
pub const LINEAR: TextureFilter = TextureFilter::LINEAR;
pub const NEAREST_MIPMAP_NEAREST: TextureFilter = TextureFilter::NEAREST_MIPMAP_NEAREST;
pub const LINEAR_MIPMAP_NEAREST: TextureFilter = TextureFilter::LINEAR_MIPMAP_NEAREST;
pub const NEAREST_MIPMAP_LINEAR: TextureFilter = TextureFilter::NEAREST_MIPMAP_LINEAR;
pub const LINEAR_MIPMAP_LINEAR: TextureFilter = TextureFilter::LINEAR_MIPMAP_LINEAR;

#[repr(u32)]
pub enum TextureWrap {
    CLAMP_TO_EDGE = gl::CLAMP_TO_EDGE,
    CLAMP_TO_BORDER = gl::CLAMP_TO_BORDER,
    MIRRORED_REPEAT = gl::MIRRORED_REPEAT,
    REPEAT = gl::REPEAT,
    MIRROR_CLAMP_TO_EDGE = gl::MIRROR_CLAMP_TO_EDGE,
}

pub const CLAMP_TO_EDGE: TextureWrap = TextureWrap::CLAMP_TO_EDGE;
pub const CLAMP_TO_BORDER: TextureWrap = TextureWrap::CLAMP_TO_BORDER;
pub const MIRRORED_REPEAT: TextureWrap = TextureWrap::MIRRORED_REPEAT;
pub const REPEAT: TextureWrap = TextureWrap::REPEAT;
pub const MIRROR_CLAMP_TO_EDGE: TextureWrap = TextureWrap::MIRROR_CLAMP_TO_EDGE;

#[inline]
pub fn gen_textures(names: &mut [Option<TextureName>]) {
    // This *should* work because Option<NonZero<u32>> should be
    // represented as a single u32 where 0 means None and x > 0 means
    // Some(NonZero(x)).
    unsafe {
        gl::GenTextures(names.len() as i32, names.as_mut_ptr() as *mut u32);
    }
}

#[derive(Clone,Copy)]
pub struct MaxCombinedTextureImageUnits(u32);

impl MaxCombinedTextureImageUnits {
    pub fn new() -> Self {
        MaxCombinedTextureImageUnits(unsafe {
            let mut values: [i32; 1] = ::std::mem::uninitialized();
            gl::GetIntegerv(gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS, values.as_mut_ptr());
            values[0] as u32
        })
    }
}

#[derive(Clone,Copy)]
pub struct TextureUnit(u32);

impl TextureUnit {
    #[inline]
    pub fn new(unit: u32, max: MaxCombinedTextureImageUnits) -> Option<Self> {
        if unit < max.0 {
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

#[inline]
pub fn active_texture(unit: TextureUnit) {
    unsafe {
        gl::ActiveTexture(unit.as_u32());
    }
}

#[inline]
pub fn bind_texture(target: TextureTarget, name: &TextureName) {
    unsafe {
        gl::BindTexture(target as u32, name.as_u32());
    }
}

// pub fn tex_parameter_i(target: TextureTarget, parameter: TextureParameterI) {
//     unsafe {
//         gl::TexParameteri(target as u32, parameter.param(), parameter.value())
//     }
// }

#[inline]
pub fn tex_parameter_min_filter(target: TextureTarget, value: TextureFilter) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_MIN_FILTER, value as i32);
    }
}

#[inline]
pub fn tex_parameter_mag_filter(target: TextureTarget, value: TextureFilter) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_MAG_FILTER, value as i32);
    }
}

#[inline]
pub fn tex_parameter_wrap_s(target: TextureTarget, value: TextureWrap) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_S, value as i32)
    }
}

#[inline]
pub fn tex_parameter_wrap_t(target: TextureTarget, value: TextureWrap) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_T, value as i32)
    }
}

#[inline]
pub fn tex_parameter_wrap_r(target: TextureTarget, value: TextureWrap) {
    unsafe {
        gl::TexParameteri(target as u32, gl::TEXTURE_WRAP_R, value as i32)
    }
}

#[inline]
pub fn generate_mipmap(target: TextureTarget) {
    unsafe {
        gl::GenerateMipmap(target as u32);
    }
}

#[inline]
pub unsafe fn tex_image_2d(
    target: TextureTarget,
    mipmap_level: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    format: u32,
    component_format: u32,
    data: *const ::std::os::raw::c_void,
) {
    gl::TexImage2D(
        target as u32,
        mipmap_level,
        internal_format,
        width,
        height,
        0, // border, must be zero
        format,
        component_format,
        data,
    );
}
