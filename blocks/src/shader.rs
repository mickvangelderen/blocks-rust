use glw;
use std::num::NonZeroU32;

pub enum State {
    Uncompiled = 1,
    Compiled = 2,
}

#[repr(transparent)]
pub struct ShaderName(NonZeroU32);

impl ShaderName {
    unsafe fn compile(&self, sources: &[&str]) -> Result<(), String> {
        Ok(())
    }
}

pub struct Shader {
    state: State,
    name: ShaderName,
}

impl Shader {
    unsafe fn recompile(&mut self, sources: &[&str]) -> Result<(), String> {
        match self.name.compile(sources) {
            Ok(()) => {
                self.state = State::Compiled;
                Ok(())
            },
            Err(err) => {
                self.state = State::Uncompiled;
                Err(err)
            }
        }
    }
}

#[derive(Debug)]
pub struct ShaderName(NonZeroU32);

impl ShaderName {
    #[inline]
    fn new(kind: ShaderKind) -> Option<Self> {
        NonZeroU32::new(unsafe { gl::CreateShader(kind as u32) }).map(ShaderName)
    }

    #[inline]
    pub unsafe fn as_u32(&self) -> u32 {
        self.0.get()
    }

    pub unsafe fn compile(
        self,
        sources: &[&str],
    ) -> Result<CompiledShaderName, (ShaderName, String)> {
        // NOTE: Const generics please.
        let source_lengths: Vec<i32> = sources.iter().map(|source| source.len() as i32).collect();

        gl::ShaderSource(
            self.as_u32(),
            sources.len() as i32,
            sources.as_ptr() as *const *const i8,
            source_lengths.as_ptr(),
        );

        gl::CompileShader(self.as_u32());

        let status = {
            let mut status = ::std::mem::uninitialized();
            gl::GetShaderiv(self.as_u32(), gl::COMPILE_STATUS, &mut status);
            status
        };

        if status == (gl::TRUE as i32) {
            Ok(CompiledShaderName(self))
        } else {
            let capacity = {
                let mut capacity: i32 = ::std::mem::uninitialized();
                gl::GetShaderiv(self.as_u32(), gl::INFO_LOG_LENGTH, &mut capacity);
                assert!(capacity >= 0);
                capacity
            };

            let buffer = {
                let mut buffer: Vec<u8> = Vec::with_capacity(capacity as usize);
                let mut length: i32 = ::std::mem::uninitialized();
                gl::GetShaderInfoLog(
                    self.as_u32(),
                    capacity,
                    &mut length,
                    buffer.as_mut_ptr() as *mut i8,
                );
                assert!(length >= 0 && length <= capacity);
                buffer.set_len(length as usize);
                buffer
            };

            Err((
                self,
                String::from_utf8(buffer).expect("Shader info log is not utf8"),
            ))
        }
    }
}

impl Drop for ShaderName {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.as_u32());
        }
    }
}
// let source = file_to_string(&assets.chunk_renderer_vert).unwrap();
// let name: glw::VertexShaderName = match self.vertex_shader_name {
//     VertexShader::Uncompiled(ref mut name) => name.take(),
//     VertexShader::Compiled(ref mut name) => name.take().map(From::from),
// }
// .unwrap();

// self.vertex_shader_name = name
//     .compile(&[&source])
//     .map(|name| VertexShader::Compiled(Some(name)))
//     .unwrap_or_else(|(name, err)| {
//         eprintln!("\n{}:\n{}", assets.chunk_renderer_vert.display(), err);
//         VertexShader::Uncompiled(Some(name))
//     });
