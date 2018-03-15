use block::Block;
use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::prelude::*;
use chunk::Chunk;
use gl;
use glw;
use cube;
use image;

pub struct ChunkRenderer {
    program_name: glw::LinkedProgramName,
    stone_texture_name: glw::TextureName,
    dirt_texture_name: glw::TextureName,
}

impl ChunkRenderer {
    pub fn new() -> Self {
        let program_name = glw::ProgramName::new()
            .unwrap()
            .link(&[
                glw::VertexShaderName::new()
                    .unwrap()
                    .compile(&[
                        &r#"
#version 400 core

uniform mat4 pos_from_obj_to_clp_space;

in vec3 vs_ver_pos;
in vec2 vs_tex_pos;

out vec2 fs_tex_pos;

void main() {
    gl_Position = pos_from_obj_to_clp_space*vec4(vs_ver_pos, 1.0);
    fs_tex_pos = vs_tex_pos;
}
"#,
                    ])
                    .unwrap()
                    .as_ref(),
                glw::FragmentShaderName::new()
                    .unwrap()
                    .compile(&[
                        &r#"
#version 400 core

in vec2 fs_tex_pos;

uniform sampler2D tex;

out vec4 color;

void main() {
    color = texture(tex, fs_tex_pos);
}
"#,
                    ])
                    .unwrap()
                    .as_ref(),
            ])
            .unwrap();

        let triangle_vertex_array_name = unsafe {
            let mut names: [u32; 1] = ::std::mem::uninitialized();
            gl::GenVertexArrays(names.len() as i32, names.as_mut_ptr());
            assert!(names[0] != 0, "Failed to create vertex array.");
            names[0]
        };

        let (triangle_vertex_buffer_name, triangle_element_buffer_name) = unsafe {
            let mut names: [u32; 2] = ::std::mem::uninitialized();
            gl::GenBuffers(names.len() as i32, names.as_mut_ptr());
            assert!(names[0] != 0, "Failed to create buffer.");
            assert!(names[1] != 0, "Failed to create buffer.");
            (names[0], names[1])
        };

        unsafe {
            gl::BindVertexArray(triangle_vertex_array_name);

            gl::BindBuffer(gl::ARRAY_BUFFER, triangle_vertex_buffer_name);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                ::std::mem::size_of_val(&cube::VERTEX_DATA) as isize,
                cube::VERTEX_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );

            let vs_ver_pos_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_ver_pos"));
            assert!(vs_ver_pos_loc != -1, "Couldn't find position attribute");
            gl::EnableVertexAttribArray(vs_ver_pos_loc as u32);
            gl::VertexAttribPointer(
                vs_ver_pos_loc as u32,                      // index
                3,                                          // size (component count)
                gl::FLOAT,                                  // type (component type)
                gl::FALSE,                                  // normalized
                ::std::mem::size_of::<cube::Vertex>() as i32, // stride
                0 as *const ::std::os::raw::c_void,           // offset
            );

            let vs_tex_pos_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_tex_pos"));
            assert!(vs_tex_pos_loc != -1, "Couldn't find color attribute");
            gl::EnableVertexAttribArray(vs_tex_pos_loc as u32);
            gl::VertexAttribPointer(
                vs_tex_pos_loc as u32,                                              // index
                2,                                          // size (component count)
                gl::FLOAT,                                  // type (component type)
                gl::FALSE,                                  // normalized
                ::std::mem::size_of::<cube::Vertex>() as i32, // stride
                ::std::mem::size_of::<Vector3<f32>>() as *const ::std::os::raw::c_void, // offset
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, triangle_element_buffer_name);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ::std::mem::size_of_val(&cube::ELEMENT_DATA) as isize,
                cube::ELEMENT_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );
        }

        let stone_texture_name: glw::TextureName = {
            let name = unsafe {
                let mut names: [Option<glw::TextureName>; 1] = ::std::mem::uninitialized();
                glw::gen_textures(&mut names);

                // Move all values out of the array and forget about the array.
                let name = ::std::mem::replace(&mut names[0], ::std::mem::uninitialized());
                ::std::mem::forget(names);

                name.unwrap()
            };

            glw::bind_texture(glw::TEXTURE_2D, &name);
            glw::tex_parameter_min_filter(glw::TEXTURE_2D, glw::LINEAR_MIPMAP_LINEAR);
            glw::tex_parameter_mag_filter(glw::TEXTURE_2D, glw::NEAREST);
            glw::tex_parameter_wrap_s(glw::TEXTURE_2D, glw::REPEAT);
            glw::tex_parameter_wrap_t(glw::TEXTURE_2D, glw::REPEAT);

            let img = image::open("assets/stone_xyz.png").unwrap();
            let img = img.flipv().to_rgba();
            unsafe {
                glw::tex_image_2d(
                    glw::TEXTURE_2D,
                    0, // mipmap level
                    gl::RGBA8 as i32,
                    img.width() as i32,
                    img.height() as i32,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    img.as_ptr() as *const ::std::os::raw::c_void,
                );
            }

            glw::generate_mipmap(glw::TEXTURE_2D);

            name
        };

        let dirt_texture_name: glw::TextureName = {
            let name = unsafe {
                let mut names: [Option<glw::TextureName>; 1] = ::std::mem::uninitialized();
                glw::gen_textures(&mut names);

                // Move all values out of the array and forget about the array.
                let name = ::std::mem::replace(&mut names[0], ::std::mem::uninitialized());
                ::std::mem::forget(names);

                name.unwrap()
            };

            glw::bind_texture(glw::TEXTURE_2D, &name);
            glw::tex_parameter_min_filter(glw::TEXTURE_2D, glw::LINEAR_MIPMAP_LINEAR);
            glw::tex_parameter_mag_filter(glw::TEXTURE_2D, glw::NEAREST);
            glw::tex_parameter_wrap_s(glw::TEXTURE_2D, glw::REPEAT);
            glw::tex_parameter_wrap_t(glw::TEXTURE_2D, glw::REPEAT);

            let img = image::open("assets/dirt_xyz.png").unwrap();
            let img = img.flipv().to_rgba();
            unsafe {
                glw::tex_image_2d(
                    glw::TEXTURE_2D,
                    0, // mipmap level
                    gl::RGBA8 as i32,
                    img.width() as i32,
                    img.height() as i32,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    img.as_ptr() as *const ::std::os::raw::c_void,
                );
            }

            glw::generate_mipmap(glw::TEXTURE_2D);

            name
        };

        ChunkRenderer {
            program_name,
            stone_texture_name,
            dirt_texture_name,
        }
    }

    pub fn render(&self, pos_from_wld_to_clp_space: &Matrix4<f32>, chunk: &Chunk) {
        glw::use_program(&self.program_name);

        for (position, block) in chunk.blocks() {
            match block {
                Block::Void => continue,
                Block::Stone => {
                    glw::active_texture(glw::TEXTURE0);
                    glw::bind_texture(glw::TEXTURE_2D, &self.stone_texture_name);
                }
                Block::Dirt => {
                    glw::active_texture(glw::TEXTURE0);
                    glw::bind_texture(glw::TEXTURE_2D, &self.dirt_texture_name);
                }
            }

            let pos_from_obj_to_wld_space = Matrix4::from_translation(position);

            unsafe {
                let loc = gl::GetUniformLocation(
                    self.program_name.as_u32(),
                    gl_str!("pos_from_obj_to_clp_space"),
                );
                let pos_from_obj_to_clp_space =
                    pos_from_wld_to_clp_space * pos_from_obj_to_wld_space;
                gl::UniformMatrix4fv(
                    loc,                                // location
                    1,                                  // count
                    gl::FALSE,                          // row major
                    pos_from_obj_to_clp_space.as_ptr(), // data
                );
            }

            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,      // mode
                    12 * 3,             // count
                    gl::UNSIGNED_INT,   // index type,
                    0 as *const ::std::os::raw::c_void, // offset
                );
            }
        }
    }
}
