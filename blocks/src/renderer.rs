use assets::file_to_bytes;
use shader::*;
use std::path::Path;

pub unsafe fn recompile_and_log_vert<P: AsRef<Path>>(
    file_path: P,
    vertex_shader: &mut VertexShader,
) {
    let file_path = file_path.as_ref();
    match file_to_bytes(file_path) {
        Ok(source) => {
            vertex_shader.compile(&[&source[..]]);

            if let VertexShader::Uncompiled(ref name) = vertex_shader {
                let log = String::from_utf8(glw::get_shader_info_log_move(name.as_ref()))
                    .expect("Shader info log is not utf8.");
                eprintln!("\n{}:\n{}", file_path.display(), log);
            }
        }
        Err(err) => {
            eprintln!("Failed to read {}: {}", file_path.display(), err);
        }
    }
}

pub unsafe fn recompile_and_log_frag<P: AsRef<Path>>(
    file_path: P,
    fragment_shader: &mut FragmentShader,
) {
    let file_path = file_path.as_ref();
    match file_to_bytes(file_path) {
        Ok(source) => {
            fragment_shader.compile(&[&source[..]]);

            if let FragmentShader::Uncompiled(ref name) = fragment_shader {
                let log = String::from_utf8(glw::get_shader_info_log_move(name.as_ref()))
                    .expect("Shader info log is not utf8.");
                eprintln!("\n{}:\n{}", file_path.display(), log);
            }
        }
        Err(err) => {
            eprintln!("Failed to read {}: {}", file_path.display(), err);
        }
    }
}
