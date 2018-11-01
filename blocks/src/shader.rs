use glw;
use glw::array::SourceArray;

macro_rules! impl_shaders {
    ($(($T:ident, $N:ident)),+ $(,)*) => {
        $(
            pub enum $T {
                Uncompiled(glw::$N),
                Compiled(glw::$N),
            }

            impl $T {
                #[inline]
                pub unsafe fn compile<'a, A: SourceArray<'a>>(&mut self, sources: &A) {
                    use std::ptr;

                    let name = match ptr::read(self) {
                        $T::Uncompiled(name) => name,
                        $T::Compiled(name) => name,
                    };

                    glw::shader_source(name.as_ref(), sources);

                    glw::compile_shader(name.as_ref());

                    let compiled = glw::get_shaderiv_move(name.as_ref(), glw::COMPILE_STATUS) != 0;

                    ptr::write(
                        self,
                        if compiled {
                            $T::Compiled(name)
                        } else {
                            $T::Uncompiled(name)
                        },
                    );
                }
            }
        )+
    }
}

impl_shaders!(
    (VertexShader, VertexShaderName),
    (FragmentShader, FragmentShaderName),
);

//         gl::ShaderSource(
//             self.as_u32(),
//             sources.len() as i32,
//             sources.as_ptr() as *const *const i8,
//             source_lengths.as_ptr(),
//         );

//         gl::CompileShader(self.as_u32());

//         let status = {
//             let mut status = ::std::mem::uninitialized();
//             gl::GetShaderiv(self.as_u32(), gl::COMPILE_STATUS, &mut status);
//             status
//         };

//         if status == (gl::TRUE as i32) {
//             Ok(CompiledShaderName(self))
//         } else {
//             let capacity = {
//                 let mut capacity: i32 = ::std::mem::uninitialized();
//                 gl::GetShaderiv(self.as_u32(), gl::INFO_LOG_LENGTH, &mut capacity);
//                 assert!(capacity >= 0);
//                 capacity
//             };

//             let buffer = {
//                 let mut buffer: Vec<u8> = Vec::with_capacity(capacity as usize);
//                 let mut length: i32 = ::std::mem::uninitialized();
//                 gl::GetShaderInfoLog(
//                     self.as_u32(),
//                     capacity,
//                     &mut length,
//                     buffer.as_mut_ptr() as *mut i8,
//                 );
//                 assert!(length >= 0 && length <= capacity);
//                 buffer.set_len(length as usize);
//                 buffer
//             };

//             Err((
//                 self,
//                 String::from_utf8(buffer).expect("Shader info log is not utf8"),
//             ))
//         }
//     }
// }

// impl Drop for ShaderName {
//     #[inline]
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteShader(self.as_u32());
//         }
//     }
// }

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
