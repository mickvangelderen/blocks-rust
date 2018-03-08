use core::nonzero::NonZero;
use std::marker::PhantomData;
use std::ffi::CStr;
use super::shader::CompiledShaderName;
use gl;

#[derive(Debug)]
pub struct ProgramName(NonZero<u32>);

impl ProgramName {
    pub fn new() -> Option<Self> {
        NonZero::new(unsafe { gl::CreateProgram() }).map(ProgramName)
    }

    pub unsafe fn as_u32(&self) -> u32 {
        (self.0).get()
    }

    pub fn link(self, shaders: &[&CompiledShaderName]) -> Result<LinkedProgramName, String> {
        for shader in shaders {
            unsafe {
                gl::AttachShader(self.as_u32(), shader.as_u32());
            }
        }

        unsafe {
            gl::LinkProgram(self.as_u32());
        }

        let status = unsafe {
            let mut status = ::std::mem::uninitialized();
            gl::GetProgramiv(self.as_u32(), gl::LINK_STATUS, &mut status);
            status
        };

        if status == (gl::TRUE as i32) {
            Ok(LinkedProgramName(self))
        } else {
            let capacity = unsafe {
                let mut capacity: i32 = ::std::mem::uninitialized();
                gl::GetProgramiv(self.as_u32(), gl::INFO_LOG_LENGTH, &mut capacity);
                assert!(capacity >= 0);
                capacity
            };

            let buffer = unsafe {
                let mut buffer: Vec<u8> = Vec::with_capacity(capacity as usize);
                let mut length: i32 = ::std::mem::uninitialized();
                gl::GetProgramInfoLog(
                    self.as_u32(),
                    capacity,
                    &mut length,
                    buffer.as_mut_ptr() as *mut i8,
                );
                assert!(length >= 0 && length <= capacity);
                buffer.set_len(length as usize);
                buffer
            };

            Err(String::from_utf8(buffer).expect("Program info log is not utf8"))
        }
    }
}

impl Drop for ProgramName {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.as_u32());
        }
    }
}

#[derive(Debug)]
pub struct LinkedProgramName(ProgramName);

impl LinkedProgramName {
    pub unsafe fn as_u32(&self) -> u32 {
        self.0.as_u32()
    }

    pub fn uniform_location(&self, name: &CStr) -> UniformLocation {
        let loc;
        unsafe {
            loc = gl::GetUniformLocation(self.as_u32(), name.as_ptr() as *const i8);
        }
        UniformLocation(loc, PhantomData)
    }
}

#[derive(Debug)]
pub struct UniformLocation<'p>(i32, PhantomData<&'p LinkedProgramName>);

impl<'p> UniformLocation<'p> {
    pub unsafe fn as_i32(&self) -> i32 {
        self.0
    }
}

#[derive(Debug)]
pub struct ProgramSlot;

impl ProgramSlot {
    pub fn bind<'s, 'p>(&'s mut self, program: &'p LinkedProgramName) -> BoundProgramName {
        unsafe {
            gl::UseProgram(program.as_u32());
        }
        BoundProgramName {
            slot: PhantomData,
            program: PhantomData,
        }
    }
}

#[derive(Debug)]
#[must_use = "The program is conceptually only bound for the lifetime of this object."]
pub struct BoundProgramName<'s, 'p> {
    slot: PhantomData<&'s mut ProgramSlot>,
    program: PhantomData<&'p LinkedProgramName>,
}
