use super::*;

impl_name!(ProgramName);

#[repr(transparent)]
pub struct LinkedProgramName(ProgramName);

impl ProgramName {
    pub unsafe fn link(
        self,
    ) -> Result<LinkedProgramName, (ProgramName, String)> {
        link_program(&self);

        let linked = get_programiv_move(&self, LINK_STATUS) != (gl::FALSE as i32);

        if linked {
            Ok(LinkedProgramName(self))
        } else {
            let log = String::from_utf8(get_program_info_log_move(&self))
                .expect("Program info log is not utf8.");

            Err((self, log))
        }
    }
}

impl From<LinkedProgramName> for ProgramName {
    fn from(name: LinkedProgramName) -> Self {
        name.0
    }
}

pub enum Program {
    Unlinked(ProgramName),
    Linked(LinkedProgramName),
}

impl Program {
    pub unsafe fn link(&mut self) -> Result<(), String> {
        use std::ptr;

        // Create a bitwise copy of self.
        let name = match ptr::read(self) {
            Program::Unlinked(name) => name,
            Program::Linked(name) => name.into(),
        };

        let (zelf, ret) = match name.link() {
            Ok(name) => (Program::Linked(name), Ok(())),
            Err((name, err)) => (Program::Unlinked(name), Err(err)),
        };

        ptr::write(self, zelf);
        ret
    }
}
