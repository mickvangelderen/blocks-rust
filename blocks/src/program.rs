use glw;
use glw::ProgramName;

pub enum Program {
    Unlinked(ProgramName),
    Linked(ProgramName),
}

impl Program {
    #[inline]
    pub unsafe fn link(&mut self) {
        use std::ptr;

        // Create a bitwise copy of self.
        let name = match ptr::read(self) {
            Program::Unlinked(name) => name,
            Program::Linked(name) => name.into(),
        };

        glw::link_program(&name);

        let linked = glw::get_programiv_move(&name, glw::LINK_STATUS) != 0;

        ptr::write(
            self,
            if linked {
                Program::Linked(name)
            } else {
                Program::Unlinked(name)
            },
        );
    }

    pub unsafe fn log(&mut self) -> String {
        String::from_utf8(glw::get_program_info_log_move(self.as_ref()))
            .expect("Program info log is not valid utf8.")
    }

    pub unsafe fn delete(self) {
        match self {
            Program::Unlinked(program_name) => glw::delete_program_move(program_name),
            Program::Linked(program_name) => glw::delete_program_move(program_name),
        }
    }
}

impl AsRef<ProgramName> for Program {
    #[inline]
    fn as_ref(&self) -> &ProgramName {
        match *self {
            Program::Unlinked(ref name) => name,
            Program::Linked(ref name) => name,
        }
    }
}
