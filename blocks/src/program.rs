use std::num::NonZeroU32;
use glw;
use shader::ShaderName;
use shader::Shader;

pub enum State {
    Unlinked = 1,
    Linked = 2,
}

#[repr(transparent)]
pub struct ProgramName(NonZeroU32);

impl ProgramName {
    unsafe fn link(&self, shaders: &[&ShaderName]) -> Result<(), String> {
        Ok(())
    }
}

pub struct Program {
    state: State,
    name: ProgramName,
}

impl Program {
    unsafe fn new(name: ProgramName) -> Self {
        Program {
            state: State::Unlinked,
            name,
        }
    }

    unsafe fn relink(&mut self, shaders: &[&ShaderName]) -> Result<(), String> {
        match self.name.link(shaders) {
            Ok(()) => {
                self.state = State::Linked;
                Ok(())
            },
            Err(err) => {
                self.state = State::Unlinked;
                Err(err)
            }
        }
    }
}
