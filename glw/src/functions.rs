use super::*;
use array::Array;
use array::SourceArray;
use gl;
use std::ffi::CStr;

// Shader names.

#[inline]
pub unsafe fn create_shader<K: StaticShaderKind>(kind: K) -> Option<K::ShaderName> {
    K::ShaderName::from_raw(gl::CreateShader(kind.as_u32()))
}
// #[inline]
// pub unsafe fn create_shader<S: StaticShaderName>(kind: S::ShaderKind) -> Option<S> {
//     S::from_raw(gl::CreateShader(kind.as_u32()))
// }

#[inline]
pub unsafe fn delete_shader_move<S: StaticShaderName>(name: S) {
    gl::DeleteShader(name.as_u32());
    ::std::mem::forget(name);
}

#[inline]
// NOTE: Only supports array sizes that have an implementation.
// FIXME: Provide a version for any array size.
pub unsafe fn shader_source<'a, A: SourceArray<'a>>(shader: &ShaderName, sources: &A) {
    // Scary, pointers not bound to 'a but should be. Perhaps we should
    // move this conversion into the trait.
    let mut pointers: A::RawSourceArray = std::mem::uninitialized();
    let mut lengths: A::RawLengthArray = std::mem::uninitialized();
    for (index, source) in sources.as_slice().iter().enumerate() {
        pointers.as_mut_slice()[index] = source.as_ptr();
        lengths.as_mut_slice()[index] = source.len() as i32;
    }
    gl::ShaderSource(
        shader.as_u32(),
        sources.len() as i32,
        pointers.as_ptr() as *const *const i8,
        lengths.as_ptr(),
    );
}

#[inline]
pub unsafe fn compile_shader(shader: &ShaderName) {
    gl::CompileShader(shader.as_u32());
}

#[inline]
pub unsafe fn get_shaderiv<P: GetShaderivParam>(
    shader: &ShaderName,
    pname: P,
    pvalue: &mut P::Value,
) {
    gl::GetShaderiv(
        shader.as_u32(),
        pname.as_u32(),
        pvalue as *mut P::Value as *mut i32,
    );
}

#[inline]
pub unsafe fn get_shaderiv_move<P: GetShaderivParam>(shader: &ShaderName, pname: P) -> P::Value {
    let mut pvalue: P::Value = ::std::mem::uninitialized();
    get_shaderiv(shader, pname, &mut pvalue);
    pvalue
}

#[inline]
pub unsafe fn get_shader_info_log(shader: &ShaderName, length: &mut i32, buffer: &mut [u8]) {
    gl::GetShaderInfoLog(
        shader.as_u32(),
        buffer.len() as i32,
        length,
        buffer.as_mut_ptr() as *mut i8,
    );
}

#[inline]
pub unsafe fn get_shader_info_log_move(shader: &ShaderName) -> Vec<u8> {
    let mut buffer = {
        let capacity = get_shaderiv_move(shader, INFO_LOG_LENGTH);
        assert!(capacity >= 0);
        Vec::with_capacity(capacity as usize)
    };
    let mut length = ::std::mem::uninitialized();
    get_shader_info_log(
        shader,
        &mut length,
        ::std::slice::from_raw_parts_mut(buffer.as_mut_ptr(), buffer.capacity()),
    );
    assert!(length >= 0 && length <= buffer.capacity() as i32);
    buffer.set_len(length as usize);
    buffer
}

// Program names.

#[inline]
pub unsafe fn create_program() -> Option<ProgramName> {
    ProgramName::from_raw(gl::CreateProgram())
}

#[inline]
pub unsafe fn delete_program_move(name: ProgramName) {
    gl::DeleteProgram(name.as_u32());
    ::std::mem::forget(name);
}

#[inline]
pub unsafe fn use_program(program: &ProgramName) {
    gl::UseProgram(program.as_u32());
}

#[inline]
pub unsafe fn attach_shader(program: &ProgramName, shader: &ShaderName) {
    gl::AttachShader(program.as_u32(), shader.as_u32());
}

#[inline]
pub unsafe fn link_program(program: &ProgramName) {
    gl::LinkProgram(program.as_u32());
}

#[inline]
pub unsafe fn get_programiv<P: GetProgramivParam>(
    program: &ProgramName,
    pname: P,
    pvalue: &mut P::Value,
) {
    gl::GetProgramiv(
        program.as_u32(),
        pname.as_u32(),
        pvalue as *mut _ as *mut i32,
    );
}

#[inline]
pub unsafe fn get_programiv_move<P: GetProgramivParam>(
    program: &ProgramName,
    pname: P,
) -> P::Value {
    let mut pvalue: P::Value = ::std::mem::uninitialized();
    get_programiv(program, pname, &mut pvalue);
    pvalue
}

#[inline]
pub unsafe fn get_program_info_log(program: &ProgramName, length: &mut i32, buffer: &mut [u8]) {
    gl::GetProgramInfoLog(
        program.as_u32(),
        buffer.len() as i32,
        length,
        buffer.as_mut_ptr() as *mut i8,
    );
}

#[inline]
pub unsafe fn get_program_info_log_move(program: &ProgramName) -> Vec<u8> {
    let mut buffer = {
        let capacity = get_programiv_move(program, INFO_LOG_LENGTH);
        assert!(capacity >= 0);
        Vec::with_capacity(capacity as usize)
    };
    let mut length = ::std::mem::uninitialized();
    get_program_info_log(
        program,
        &mut length,
        ::std::slice::from_raw_parts_mut(buffer.as_mut_ptr(), buffer.capacity()),
    );
    assert!(length >= 0 && length <= buffer.capacity() as i32);
    buffer.set_len(length as usize);
    buffer
}

#[inline]
pub unsafe fn get_attrib_location(
    program_name: &ProgramName,
    attrib_name: &CStr,
) -> Option<AttributeLocation> {
    AttributeLocation::from_raw(gl::GetAttribLocation(
        program_name.as_u32(),
        attrib_name.as_ptr(),
    ))
}

#[inline]
pub unsafe fn get_uniform_location<T>(
    program_name: &ProgramName,
    uniform_name: &CStr,
) -> Option<UniformLocation<T>> {
    UniformLocation::from_raw(gl::GetUniformLocation(
        program_name.as_u32(),
        uniform_name.as_ptr(),
    ))
}

// Texture names.

#[inline]
pub unsafe fn gen_textures(names: &mut [Option<TextureName>]) {
    gl::GenTextures(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn delete_textures(names: &mut [Option<TextureName>]) {
    gl::DeleteTextures(names.len() as i32, names.as_ptr() as *const u32);
}

#[inline]
pub unsafe fn gen_textures_move<A: Array<Option<TextureName>>>() -> A {
    let mut names: A = ::std::mem::uninitialized();
    gen_textures(names.as_mut_slice());
    names
}

#[inline]
pub unsafe fn delete_textures_move<A: Array<Option<TextureName>>>(mut names: A) {
    delete_textures(names.as_mut_slice());
    ::std::mem::forget(names);
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

// Buffer names.

#[inline]
pub unsafe fn gen_buffers(names: &mut [Option<BufferName>]) {
    gl::GenBuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn delete_buffers(names: &mut [Option<BufferName>]) {
    gl::DeleteBuffers(names.len() as i32, names.as_ptr() as *const u32);
}

#[inline]
pub unsafe fn gen_buffers_move<A: Array<Option<BufferName>>>() -> A {
    let mut names: A = ::std::mem::uninitialized();
    gen_buffers(names.as_mut_slice());
    names
}

#[inline]
pub unsafe fn delete_buffers_move<A: Array<Option<BufferName>>>(mut names: A) {
    delete_buffers(names.as_mut_slice());
    ::std::mem::forget(names);
}

#[inline]
pub unsafe fn bind_buffer(target: BufferTarget, name: &BufferName) {
    gl::BindBuffer(target as u32, name.as_u32());
}

// Vertex array names.

#[inline]
pub unsafe fn gen_vertex_arrays(names: &mut [Option<VertexArrayName>]) {
    gl::GenVertexArrays(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn delete_vertex_arrays(names: &mut [Option<VertexArrayName>]) {
    gl::DeleteVertexArrays(names.len() as i32, names.as_ptr() as *const u32);
}

#[inline]
pub unsafe fn gen_vertex_arrays_move<A: Array<Option<VertexArrayName>>>() -> A {
    let mut names: A = ::std::mem::uninitialized();
    gen_vertex_arrays(names.as_mut_slice());
    names
}

#[inline]
pub unsafe fn delete_vertex_arrays_move<A: Array<Option<VertexArrayName>>>(mut names: A) {
    delete_vertex_arrays(names.as_mut_slice());
    ::std::mem::forget(names);
}

#[inline]
pub unsafe fn bind_vertex_array(name: &VertexArrayName) {
    gl::BindVertexArray(name.as_u32());
}

// Framebuffer names.

#[inline]
pub unsafe fn gen_framebuffers(names: &mut [Option<FramebufferName>]) {
    gl::GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn delete_framebuffers(names: &mut [Option<FramebufferName>]) {
    gl::GenFramebuffers(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
pub unsafe fn gen_framebuffers_move<A: Array<Option<FramebufferName>>>() -> A {
    let mut names: A = ::std::mem::uninitialized();
    gen_framebuffers(names.as_mut_slice());
    names
}

#[inline]
pub unsafe fn delete_framebuffers_move<A: Array<Option<FramebufferName>>>(mut names: A) {
    delete_framebuffers(names.as_mut_slice());
    ::std::mem::forget(names);
}

#[inline]
pub unsafe fn bind_framebuffer<T>(target: FramebufferTarget, name: &T)
where
    T: MaybeDefaultFramebufferName,
{
    gl::BindFramebuffer(target.as_u32(), name.as_u32());
}

#[inline]
pub unsafe fn check_framebuffer_status(target: FramebufferTarget) -> Option<FramebufferStatus> {
    FramebufferStatus::from_raw(gl::CheckFramebufferStatus(target.as_u32()))
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

// Uniform setters.

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
        value.as_ptr(),
    );
}

#[inline]
pub unsafe fn uniform_matrix4fv(uniform_location: &UniformLocation<[[f32; 16]]>, major_axis: MajorAxis, value: &[[f32; 16]]) {
    gl::UniformMatrix4fv(
        uniform_location.as_i32(),
        value.len() as i32,
        major_axis as u8,
        value.as_ptr() as *const f32,
    );
}

macro_rules! impl_uniform_matrix {
    ($(($n:ident, $M:ident, $Flat:ty)),+ $(,)*) => {
        $(
            pub unsafe fn $n<M: $M>(loc: &UniformLocation<$Flat>, val: &M) {
                gl::UniformMatrix4fv(
                    loc.as_i32(),
                    1,
                    M::major_axis() as u8,
                    val.as_ref().as_ptr(),
                );
            }
        )+
    }
}

impl_uniform_matrix!(
    (uniform_matrix2f, Matrix2f, [f32; 4]),
    (uniform_matrix3f, Matrix3f, [f32; 9]),
    (uniform_matrix4f, Matrix4f, [f32; 16]),
    (uniform_matrix2x3f, Matrix2x3f, [f32; 6]),
    (uniform_matrix3x2f, Matrix3x2f, [f32; 6]),
    (uniform_matrix2x4f, Matrix2x4f, [f32; 8]),
    (uniform_matrix4x2f, Matrix4x2f, [f32; 8]),
    (uniform_matrix3x4f, Matrix3x4f, [f32; 12]),
    (uniform_matrix4x3f, Matrix4x3f, [f32; 12]),
);
