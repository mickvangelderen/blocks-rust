use buffer_name::*;
use buffer_target::*;
use gl;
use program::*;
use texture_filter::*;
use texture_name::*;
use texture_target::*;
use texture_unit::*;
use texture_wrap::*;
use vertex_array_name::*;
use framebuffer_name::*;

#[inline]
pub unsafe fn bind_buffer(target: BufferTarget, name: &BufferName) {
    gl::BindBuffer(target as u32, name.as_u32());
}

#[inline]
pub unsafe fn bind_vertex_array(name: &VertexArrayName) {
    gl::BindVertexArray(name.as_u32());
}

#[inline]
pub unsafe fn use_program(program: &LinkedProgramName) {
    gl::UseProgram(program.as_u32());
}

#[inline]
pub unsafe fn active_texture(unit: TextureUnit) {
    gl::ActiveTexture(unit.as_u32());
}

#[inline]
pub unsafe fn bind_texture(target: TextureTarget, name: &TextureName) {
    gl::BindTexture(target as u32, name.as_u32());
}

pub trait TextureParameterI32Value {
    fn as_i32(&self) -> i32;
}

impl TextureParameterI32Value for TextureFilter {
    #[inline]
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl TextureParameterI32Value for TextureWrap {
    #[inline]
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}

pub trait TextureParameterI32Key {
    type Value: TextureParameterI32Value;

    fn as_u32(&self) -> u32;
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

#[inline]
pub unsafe fn tex_parameter_i<P, V>(target: TextureTarget, parameter: P, value: V)
where
    P: TextureParameterI32Key<Value = V>,
    V: TextureParameterI32Value,
{
    gl::TexParameteri(target as u32, parameter.as_u32(), value.as_i32());
}

#[inline]
pub unsafe fn generate_mipmap(target: TextureTarget) {
    gl::GenerateMipmap(target as u32);
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

/// Careful, simply overwrites {names} and doesn't drop the values.
#[inline]
pub unsafe fn gen_textures(names: &mut [Option<TextureName>]) {
    assert_eq!(
        ::std::mem::size_of::<[Option<TextureName>; 2]>(),
        ::std::mem::size_of::<[u32; 2]>(),
    );
    gl::GenTextures(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

/// Careful, simply overwrites {names} and doesn't drop the values.
#[inline]
pub unsafe fn gen_buffers(names: &mut [Option<BufferName>]) {
    assert_eq!(
        ::std::mem::size_of::<[Option<BufferName>; 2]>(),
        ::std::mem::size_of::<[u32; 2]>(),
    );
    gl::GenBuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

/// Careful, simply overwrites {names} and doesn't drop the values.
#[inline]
pub unsafe fn gen_vertex_arrays(names: &mut [Option<VertexArrayName>]) {
    assert_eq!(
        ::std::mem::size_of::<[Option<VertexArrayName>; 2]>(),
        ::std::mem::size_of::<[u32; 2]>(),
    );
    gl::GenVertexArrays(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

/// Careful, simply overwrites {names} and doesn't drop the values.
#[inline]
pub unsafe fn gen_framebuffers(names: &mut [Option<FramebufferName>]) {
    assert_eq!(
        ::std::mem::size_of::<[Option<FramebufferName>; 2]>(),
        ::std::mem::size_of::<[u32; 2]>(),
    );
    gl::GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
}
