use gl;
use super::*;
use std::ffi::CStr;

#[inline]
pub unsafe fn get_attrib_location(program: &LinkedProgramName, name: &CStr) -> Option<AttributeLocation> {
    AttributeLocation::new(program, name)
}

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

// NOTE: We do not trust the opengl implementation to overwrite all
// values, so we initialize the names to Nones. Since the names can't be
// dropped the process will abort.

#[inline]
pub unsafe fn gen_textures(names: &mut [Option<TextureName>]) {
    for name in names.iter_mut() {
        ::std::mem::drop(name.take());
    }
    gl::GenTextures(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn delete_textures(names: &mut [Option<TextureName>]) {
    gl::DeleteTextures(names.len() as i32, names.as_ptr() as *const u32);
    for name in names.iter_mut() {
        ::std::mem::forget(name.take());
    }
}

#[inline]
pub unsafe fn gen_buffers(names: &mut [Option<BufferName>]) {
    for name in names.iter_mut() {
        ::std::mem::drop(name.take());
    }
    gl::GenBuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn delete_buffers(names: &mut [Option<BufferName>]) {
    gl::DeleteBuffers(names.len() as i32, names.as_ptr() as *const u32);
    for name in names.iter_mut() {
        ::std::mem::forget(name.take());
    }
}

#[inline]
pub unsafe fn gen_vertex_arrays(names: &mut [Option<VertexArrayName>]) {
    for name in names.iter_mut() {
        ::std::mem::drop(name.take());
    }
    gl::GenVertexArrays(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn delete_vertex_arrays(names: &mut [Option<VertexArrayName>]) {
    gl::DeleteVertexArrays(names.len() as i32, names.as_ptr() as *const u32);
    for name in names.iter_mut() {
        ::std::mem::forget(name.take());
    }
}

#[inline]
pub unsafe fn gen_framebuffers(names: &mut [Option<FramebufferName>]) {
    for name in names.iter_mut() {
        ::std::mem::drop(name.take());
    }
    gl::GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn delete_framebuffers(names: &mut [Option<FramebufferName>]) {
    gl::GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
    for name in names.iter_mut() {
        ::std::mem::forget(name.take());
    }
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

#[inline]
pub unsafe fn uniform_1fv(uniform_location: &UniformLocation<*const f32>, value: &[f32]) {
    gl::Uniform1fv(
        uniform_location.as_i32(),
        value.len() as i32,
        value.as_ptr()
    );
}
