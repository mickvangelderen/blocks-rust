impl_name!(ProgramName);

use super::*;

impl ProgramName {
    pub unsafe fn link(
        self,
        shaders: &[&ShaderName],
    ) -> Result<ProgramName, (ProgramName, String)> {
        for shader in shaders {
            attach_shader(&self, shader);
        }

        link_program(&self);

        let status = get_programiv_move(&self, LINK_STATUS);

        if status != (gl::FALSE as i32) {
            Ok(self)
        } else {
            let log = String::from_utf8(get_program_info_log_move(&self))
                .expect("Program info log is not utf8");

            Err((self, log))
        }
    }
}
