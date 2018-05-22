extern crate gl;

use super::name::Name;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ShaderKind {
    Compute = gl::COMPUTE_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
    Vertex = gl::VERTEX_SHADER,
    TesselationControl = gl::TESS_CONTROL_SHADER,
    TesselationEvaluation = gl::TESS_EVALUATION_SHADER,
}

#[derive(Debug)]
pub struct ShaderName(Name);

#[derive(Debug, Clone)]
pub struct CompilationFailed(String);

use std::error;
use std::fmt;

impl fmt::Display for CompilationFailed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for CompilationFailed {
    fn description(&self) -> &str {
        &self.0
    }
}

impl ShaderName {
    #[inline]
    fn new(kind: ShaderKind) -> Option<Self> {
        Name::new(unsafe { gl::CreateShader(kind as u32) }).map(ShaderName)
    }

    #[inline]
    pub unsafe fn as_u32(&self) -> u32 {
        self.0.get()
    }

    pub fn compile(self, sources: &[&str]) -> Result<CompiledShaderName, CompilationFailed> {
        let source_lengths: Vec<i32> = sources.iter().map(|source| source.len() as i32).collect();

        unsafe {
            gl::ShaderSource(
                self.as_u32(),
                sources.len() as i32,
                sources.as_ptr() as *const *const i8,
                source_lengths.as_ptr(),
            );
            gl::CompileShader(self.as_u32());
        }

        let status = unsafe {
            let mut status = ::std::mem::uninitialized();
            gl::GetShaderiv(self.as_u32(), gl::COMPILE_STATUS, &mut status);
            status
        };

        if status != (gl::TRUE as i32) {
            let capacity = unsafe {
                let mut capacity: i32 = ::std::mem::uninitialized();
                gl::GetShaderiv(self.as_u32(), gl::INFO_LOG_LENGTH, &mut capacity);
                assert!(capacity >= 0);
                capacity
            };

            let buffer = unsafe {
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

            Err(CompilationFailed(
                String::from_utf8(buffer).expect("Shader info log is not utf8"),
            ))
        } else {
            Ok(CompiledShaderName(self))
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

#[derive(Debug)]
pub struct CompiledShaderName(ShaderName);

impl CompiledShaderName {
    #[inline]
    pub unsafe fn as_u32(&self) -> u32 {
        self.0.as_u32()
    }
}

impl AsRef<ShaderName> for CompiledShaderName {
    #[inline]
    fn as_ref(&self) -> &ShaderName {
        &self.0
    }
}

macro_rules! impl_shader_kind {
    ($Kind:path, $KindShaderName:ident, $CompiledKindShaderName:ident) => {
        #[derive(Debug)]
        pub struct $KindShaderName(ShaderName);

        impl $KindShaderName {
            #[inline]
            pub fn new() -> Option<Self> {
                ShaderName::new($Kind).map($KindShaderName)
            }

            #[inline]
            pub unsafe fn as_u32(&self) -> u32 {
                self.0.as_u32()
            }

            #[inline]
            pub fn compile(
                self,
                sources: &[&str],
            ) -> Result<$CompiledKindShaderName, CompilationFailed> {
                self.0.compile(sources).map($CompiledKindShaderName)
            }
        }

        impl AsRef<ShaderName> for $KindShaderName {
            #[inline]
            fn as_ref(&self) -> &ShaderName {
                &self.0
            }
        }

        pub struct $CompiledKindShaderName(CompiledShaderName);

        impl AsRef<CompiledShaderName> for $CompiledKindShaderName {
            #[inline]
            fn as_ref(&self) -> &CompiledShaderName {
                &self.0
            }
        }

        impl AsRef<ShaderName> for $CompiledKindShaderName {
            #[inline]
            fn as_ref(&self) -> &ShaderName {
                self.0.as_ref()
            }
        }
    };
}

impl_shader_kind!(
    ShaderKind::Compute,
    ComputeShaderName,
    CompiledComputeShaderName
);
impl_shader_kind!(
    ShaderKind::Fragment,
    FragmentShaderName,
    CompiledFragmentShaderName
);
impl_shader_kind!(
    ShaderKind::Geometry,
    GeometryShaderName,
    CompiledGeometryShaderName
);
impl_shader_kind!(
    ShaderKind::Vertex,
    VertexShaderName,
    CompiledVertexShaderName
);
impl_shader_kind!(
    ShaderKind::TesselationControl,
    TesselationControlShaderName,
    CompiledTesselationControlShaderName
);
impl_shader_kind!(
    ShaderKind::TesselationEvaluation,
    TesselationEvaluationShaderName,
    CompiledTesselationEvaluationShaderName
);
