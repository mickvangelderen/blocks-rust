use buffer_name::*;
use buffer_target::*;
use framebuffer_attachment::*;
use framebuffer_name::*;
use framebuffer_target::*;
use gl;
use program::*;
use texture_name::*;
use texture_parameter::*;
use texture_target::*;
use texture_unit::*;
use uniform_location::*;
use vertex_array_name::*;

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

#[inline]
pub unsafe fn bind_framebuffer<T>(target: FramebufferTarget, name: &T)
where
    T: MaybeDefaultFramebufferName,
{
    gl::BindFramebuffer(target.as_u32(), name.as_u32());
}

#[inline]
pub unsafe fn framebuffer_texture_2d(
    framebuffer_target: FramebufferTarget,
    framebuffer_attachment: FramebufferAttachment,
    texture_target: TextureTarget,
    texture_name: &TextureName,
    mipmap_level: i32,
) {
    gl::FramebufferTexture2D(
        framebuffer_target.as_u32(),
        framebuffer_attachment.as_u32(),
        texture_target.as_u32(),
        texture_name.as_u32(),
        mipmap_level,
    );
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

#[inline]
pub unsafe fn uniform_1i(uniform_location: &UniformLocation<i32>, value: i32) {
    gl::Uniform1i(uniform_location.as_i32(), value);
}

#[inline]
pub unsafe fn uniform_2i<T: AsRef<[i32; 2]>>(
    uniform_location: &UniformLocation<[i32; 2]>,
    value: T,
) {
    let value = value.as_ref();
    gl::Uniform2i(uniform_location.as_i32(), value[0], value[1]);
}

#[inline]
pub unsafe fn uniform_3i<T: AsRef<[i32; 3]>>(
    uniform_location: &UniformLocation<[i32; 3]>,
    value: T,
) {
    let value = value.as_ref();
    gl::Uniform3i(uniform_location.as_i32(), value[0], value[1], value[2]);
}

#[inline]
pub unsafe fn uniform_4i<T: AsRef<[i32; 4]>>(
    uniform_location: &UniformLocation<[i32; 4]>,
    value: T,
) {
    let value = value.as_ref();
    gl::Uniform4i(
        uniform_location.as_i32(),
        value[0],
        value[1],
        value[2],
        value[3],
    );
}

#[inline]
pub unsafe fn uniform_1f(uniform_location: &UniformLocation<f32>, value: f32) {
    gl::Uniform1f(uniform_location.as_i32(), value);
}

#[inline]
pub unsafe fn uniform_2f(uniform_location: &UniformLocation<[f32; 2]>, value: [f32; 2]) {
    gl::Uniform2f(uniform_location.as_i32(), value[0], value[1]);
}

#[inline]
pub unsafe fn uniform_3f(uniform_location: &UniformLocation<[f32; 3]>, value: [f32; 3]) {
    gl::Uniform3f(uniform_location.as_i32(), value[0], value[1], value[2]);
}

#[inline]
pub unsafe fn uniform_4f(uniform_location: &UniformLocation<[f32; 4]>, value: [f32; 4]) {
    gl::Uniform4f(
        uniform_location.as_i32(),
        value[0],
        value[1],
        value[2],
        value[3],
    );
}
