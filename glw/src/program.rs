use gl;
use name::Name;
use shader::CompiledShaderName;

#[derive(Debug)]
pub struct ProgramName(Name);

impl ProgramName {
    pub fn new() -> Option<Self> {
        Name::new(unsafe { gl::CreateProgram() }).map(ProgramName)
    }

    pub unsafe fn as_u32(&self) -> u32 {
        (self.0).get()
    }

    pub unsafe fn link(self, shaders: &[&CompiledShaderName]) -> Result<LinkedProgramName, (ProgramName, String)> {
        for shader in shaders {
            gl::AttachShader(self.as_u32(), shader.as_u32());
        }

        gl::LinkProgram(self.as_u32());

        let status = {
            let mut status: i32 = ::std::mem::uninitialized();
            gl::GetProgramiv(self.as_u32(), gl::LINK_STATUS, &mut status);
            status
        };

        if status == (gl::TRUE as i32) {
            Ok(LinkedProgramName(self))
        } else {
            let capacity = {
                let mut capacity: i32 = ::std::mem::uninitialized();
                gl::GetProgramiv(self.as_u32(), gl::INFO_LOG_LENGTH, &mut capacity);
                assert!(capacity >= 0);
                capacity
            };

            let buffer = {
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

            Err((self, String::from_utf8(buffer).expect("Program info log is not utf8")))
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
}

// Permanently discard linked state.
impl From<LinkedProgramName> for ProgramName {
    fn from(name: LinkedProgramName) -> Self {
        name.0
    }
}

// use std::marker::PhantomData;

// #[derive(Debug)]
// pub struct ProgramSlot;

// impl ProgramSlot {
//     pub fn bind<'s, 'p>(&'s mut self, program: &'p LinkedProgramName) -> BoundProgramName {
//         unsafe {
//             use_program(program);
//         }
//         BoundProgramName {
//             slot: PhantomData,
//             program: PhantomData,
//         }
//     }
// }

// #[derive(Debug)]
// #[must_use = "The program is conceptually only bound for the lifetime of this object."]
// pub struct BoundProgramName<'s, 'p> {
//     slot: PhantomData<&'s mut ProgramSlot>,
//     program: PhantomData<&'p LinkedProgramName>,
// }
