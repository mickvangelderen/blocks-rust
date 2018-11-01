use assets::file_to_bytes;
use assets::Assets;
use block::Block;
use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath_ext::*;
use chunk;
use chunk::Chunk;
use cube;
use gl;
use glw;
use glw::prelude::*;
use image;
use program::*;
use shader::*;

pub struct ChunkRendererChanges {
    pub vert: bool,
    pub frag: bool,
    pub dirt: bool,
    pub stone: bool,
}

impl ChunkRendererChanges {
    pub fn new() -> Self {
        ChunkRendererChanges {
            vert: false,
            frag: false,
            dirt: false,
            stone: false,
        }
    }

    pub fn all() -> Self {
        ChunkRendererChanges {
            vert: true,
            frag: true,
            dirt: true,
            stone: true,
        }
    }
}

pub struct ChunkRenderer {
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader,
    program: Program,
    pos_from_wld_to_clp_space_loc: Option<glw::UniformLocation<[[f32; 4]; 4]>>,
    texture_atlas_name: glw::TextureName,
    vertex_array_name: glw::VertexArrayName,
    #[allow(unused)]
    vertex_buffer_name: glw::BufferName,
    #[allow(unused)]
    element_buffer_name: glw::BufferName,
    block_buffer_name: glw::BufferName,
}

impl ChunkRenderer {
    pub unsafe fn new(assets: &Assets) -> Self {
        let vertex_shader_name = glw::create_shader(glw::VERTEX_SHADER).unwrap();
        let fragment_shader_name = glw::create_shader(glw::FRAGMENT_SHADER).unwrap();
        let program_name = glw::create_program().unwrap();

        glw::attach_shader(&program_name, vertex_shader_name.as_ref());
        glw::attach_shader(&program_name, fragment_shader_name.as_ref());

        let [vertex_buffer_name, element_buffer_name, block_buffer_name] =
            glw::gen_buffers_move::<[_; 3]>().unwrap_all().unwrap();

        let [texture_atlas_name] = glw::gen_textures_move::<[_; 1]>().unwrap_all().unwrap();

        let [vertex_array_name] = glw::gen_vertex_arrays_move::<[_; 1]>()
            .unwrap_all()
            .unwrap();

        glw::bind_vertex_array(&vertex_array_name);

        // Set up vertex buffer
        {
            glw::bind_buffer(glw::ARRAY_BUFFER, &vertex_buffer_name);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                ::std::mem::size_of_val(&cube::VERTEX_DATA) as isize,
                cube::VERTEX_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );
        }

        // Set up element buffer
        {
            glw::bind_buffer(glw::ELEMENT_ARRAY_BUFFER, &element_buffer_name);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ::std::mem::size_of_val(&cube::ELEMENT_DATA) as isize,
                cube::ELEMENT_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );
        }

        // Set up block buffer
        {
            glw::bind_buffer(glw::ARRAY_BUFFER, &block_buffer_name);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                ::std::mem::size_of::<[Block; chunk::CHUNK_TOTAL_BLOCKS]>() as isize,
                ::std::ptr::null(),
                gl::STREAM_DRAW,
            );
        }

        {
            glw::bind_texture(glw::TEXTURE_2D_ARRAY, &texture_atlas_name);

            gl::TexStorage3D(
                gl::TEXTURE_2D_ARRAY, // target
                6,                    // levels
                gl::RGBA8,            // internal format
                32,                   // width
                32,                   // height
                2,                    // depth (layer count)
            );

            glw::tex_parameter_i(
                glw::TEXTURE_2D_ARRAY,
                glw::TEXTURE_MIN_FILTER,
                glw::LINEAR_MIPMAP_LINEAR,
            );
            glw::tex_parameter_i(glw::TEXTURE_2D_ARRAY, glw::TEXTURE_MAG_FILTER, glw::NEAREST);
            glw::tex_parameter_i(
                glw::TEXTURE_2D_ARRAY,
                glw::TEXTURE_WRAP_S,
                glw::CLAMP_TO_EDGE,
            );
            glw::tex_parameter_i(
                glw::TEXTURE_2D_ARRAY,
                glw::TEXTURE_WRAP_T,
                glw::CLAMP_TO_EDGE,
            );
        }

        let mut renderer = ChunkRenderer {
            vertex_shader: VertexShader::Uncompiled(vertex_shader_name),
            fragment_shader: FragmentShader::Uncompiled(fragment_shader_name),
            program: Program::Unlinked(program_name),
            pos_from_wld_to_clp_space_loc: None,
            texture_atlas_name,
            vertex_array_name,
            vertex_buffer_name,
            element_buffer_name,
            block_buffer_name,
        };

        renderer.update(assets, ChunkRendererChanges::all());

        renderer
    }

    pub unsafe fn update(&mut self, assets: &Assets, changes: ChunkRendererChanges) {
        if changes.vert {
            let file_path = &assets.chunk_renderer_vert;
            match file_to_bytes(&file_path) {
                Ok(source) => {
                    self.vertex_shader.compile(&[&source[..]]);

                    if let VertexShader::Uncompiled(ref name) = self.vertex_shader {
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

        if changes.frag {
            let file_path = &assets.chunk_renderer_frag;
            match file_to_bytes(&file_path) {
                Ok(source) => {
                    self.fragment_shader.compile(&[&source[..]]);

                    if let FragmentShader::Uncompiled(ref name) = self.fragment_shader {
                        let log = String::from_utf8(glw::get_shader_info_log_move(name.as_ref()))
                            .expect("Shader info log is not valid utf8.");
                        eprintln!("\n{}:\n{}", file_path.display(), log);
                    }
                }
                Err(err) => {
                    eprintln!("Failed to read {}: {}", file_path.display(), err);
                }
            }
        }

        if changes.vert || changes.frag {
            if let VertexShader::Compiled(_) = self.vertex_shader {
                if let FragmentShader::Compiled(_) = self.fragment_shader {
                    self.program.link();

                    match self.program {
                        Program::Unlinked(ref program_name) => {
                            let log =
                                String::from_utf8(glw::get_program_info_log_move(program_name))
                                    .expect("Program info log is not valid utf8.");
                            eprintln!("\nFailed to link program:\n{}", log);
                        }
                        Program::Linked(ref program_name) => {
                            // Update uniform locations.
                            self.pos_from_wld_to_clp_space_loc = glw::get_uniform_location(
                                &program_name,
                                static_cstr!("pos_from_wld_to_clp_space"),
                            );

                            // Bind the program.
                            glw::use_program(&program_name);

                            // Set texture sampler uniform.
                            match glw::get_uniform_location(
                                &program_name,
                                static_cstr!("texture_atlas"),
                            ) {
                                Some(ref texture_atlas_loc) => {
                                    glw::uniform_1i(texture_atlas_loc, 0);
                                }
                                None => {
                                    eprintln!("Could not find uniform \"texture_atlas\".");
                                }
                            }

                            // Set up the vertex array object.
                            {
                                glw::bind_vertex_array(&self.vertex_array_name);

                                // Set vertex position and texture position attributes.
                                {
                                    glw::bind_buffer(glw::ARRAY_BUFFER, &self.vertex_buffer_name);

                                    // Bind vertex position attribute.
                                    match glw::get_attrib_location(
                                        &program_name,
                                        static_cstr!("vs_ver_pos"),
                                    ) {
                                        Some(loc) => {
                                            gl::EnableVertexAttribArray(loc.as_u32());
                                            gl::VertexAttribPointer(
                                                loc.as_u32(),                                 // index
                                                3,         // size (component count)
                                                gl::FLOAT, // type (component type)
                                                gl::FALSE, // normalized
                                                ::std::mem::size_of::<cube::Vertex>() as i32, // stride
                                                0 as *const ::std::os::raw::c_void, // offset
                                            );
                                        }
                                        None => {
                                            eprintln!("Could not find vs_ver_pos attribute.");
                                        }
                                    }

                                    // Bind texture coordinate attribute.
                                    match glw::get_attrib_location(
                                        &program_name,
                                        static_cstr!("vs_tex_pos"),
                                    ) {
                                        Some(loc) => {
                                            gl::EnableVertexAttribArray(loc.as_u32());
                                            gl::VertexAttribPointer(
                                                loc.as_u32(),                                 // index
                                                2,         // size (component count)
                                                gl::FLOAT, // type (component type)
                                                gl::FALSE, // normalized
                                                ::std::mem::size_of::<cube::Vertex>() as i32, // stride
                                                ::std::mem::size_of::<Vector3<f32>>()
                                                    as *const ::std::os::raw::c_void, // offset
                                            );
                                        }
                                        None => {
                                            eprintln!("Could not find vs_tex_pos attribute.");
                                        }
                                    }
                                }

                                // Set block type attribute.
                                {
                                    glw::bind_buffer(glw::ARRAY_BUFFER, &self.block_buffer_name);

                                    // Bind block type attribute.
                                    match glw::get_attrib_location(
                                        &program_name,
                                        static_cstr!("vs_blk_type"),
                                    ) {
                                        Some(loc) => {
                                            gl::EnableVertexAttribArray(loc.as_u32());
                                            gl::VertexAttribIPointer(
                                                loc.as_u32(),                          // index
                                                ::std::mem::size_of::<Block>() as i32, // size (component count)
                                                gl::UNSIGNED_BYTE, // type (component type)
                                                ::std::mem::size_of::<Block>() as i32, // stride
                                                0 as *const ::std::os::raw::c_void, // offset
                                            );
                                            gl::VertexAttribDivisor(
                                                loc.as_u32(), // index
                                                1,            // advance every # instances
                                            );
                                        }
                                        None => {
                                            eprintln!("Could not find vs_blk_type attribute.");
                                        }
                                    }
                                }

                                // Bind the element array buffer.
                                glw::bind_buffer(
                                    glw::ELEMENT_ARRAY_BUFFER,
                                    &self.element_buffer_name,
                                );
                            }
                        }
                    }
                }
            }
        }

        if changes.dirt || changes.stone {
            glw::bind_texture(glw::TEXTURE_2D_ARRAY, &self.texture_atlas_name);
        }

        if changes.stone {
            let img = image::open(&assets.stone_xyz_png).unwrap();
            let img = img.flipv().to_rgba();
            assert_eq!(img.width(), 32);
            assert_eq!(img.height(), 32);
            gl::TexSubImage3D(
                gl::TEXTURE_2D_ARRAY,                          // target
                0,                                             // mipmap level
                0,                                             // xoffset
                0,                                             // yoffset,
                0,                                             // zoffset (slice),
                img.width() as i32,                            // width
                img.height() as i32,                           // height
                1,                                             // depth
                gl::RGBA,                                      // format
                gl::UNSIGNED_BYTE,                             // type
                img.as_ptr() as *const ::std::os::raw::c_void, // data
            );
        }

        if changes.dirt {
            let img = image::open(&assets.dirt_xyz_png).unwrap();
            let img = img.flipv().to_rgba();
            assert_eq!(img.width(), 32);
            assert_eq!(img.height(), 32);
            gl::TexSubImage3D(
                gl::TEXTURE_2D_ARRAY,                          // target
                0,                                             // mipmap level
                0,                                             // xoffset
                0,                                             // yoffset,
                1,                                             // zoffset (slice),
                img.width() as i32,                            // width
                img.height() as i32,                           // height
                1,                                             // depth
                gl::RGBA,                                      // format
                gl::UNSIGNED_BYTE,                             // type
                img.as_ptr() as *const ::std::os::raw::c_void, // data
            );
        }

        if changes.stone || changes.dirt {
            glw::generate_mipmap(glw::TEXTURE_2D_ARRAY);
        }
    }

    pub unsafe fn render(&mut self, pos_from_wld_to_clp_space: &Matrix4<f32>, chunk: &Chunk) {
        if let Program::Linked(ref program_name) = self.program {
            if let Some(ref pos_from_wld_to_clp_space_loc) = self.pos_from_wld_to_clp_space_loc {
                // Update block type buffer.
                glw::bind_buffer(glw::ARRAY_BUFFER, &self.block_buffer_name);
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,                                                     // target
                    0,                                                                    // offset
                    ::std::mem::size_of::<[Block; chunk::CHUNK_TOTAL_BLOCKS]>() as isize, // size
                    chunk.blocks.as_ptr() as *const ::std::os::raw::c_void,               // data
                );

                glw::use_program(&program_name);

                glw::bind_vertex_array(&self.vertex_array_name);

                // for (position, block) in chunk.blocks() {
                //     match block {
                //         Block::Void => continue,
                //         Block::Stone => {
                //             glw::active_texture(glw::TEXTURE0);
                //             glw::bind_texture(glw::TEXTURE_2D, &self.stone_texture_name);
                //         }
                //         Block::Dirt => {
                //             glw::active_texture(glw::TEXTURE0);
                //             glw::bind_texture(glw::TEXTURE_2D, &self.dirt_texture_name);
                //         }
                //     }

                //     let pos_from_obj_to_wld_space = Matrix4::from_translation(position);

                //     unsafe {
                //         let loc = gl::GetUniformLocation(
                //             self.program_name.as_u32(),
                //             gl_str!("pos_from_obj_to_clp_space"),
                //         );
                //         assert!(loc != -1, "failed to query uniform location");
                //         let pos_from_obj_to_clp_space =
                //             pos_from_wld_to_clp_space * pos_from_obj_to_wld_space;
                //         gl::UniformMatrix4fv(
                //             loc,                                // location
                //             1,                                  // count
                //             gl::FALSE,                          // row major
                //             pos_from_obj_to_clp_space.as_ptr(), // data
                //         );
                //     }

                //     unsafe {
                //         gl::DrawElements(
                //             gl::TRIANGLES,                      // mode
                //             12 * 3,                             // count
                //             gl::UNSIGNED_INT,                   // index type,
                //             0 as *const ::std::os::raw::c_void, // offset
                //         );
                //     }
                // }

                pos_from_wld_to_clp_space_loc.set(pos_from_wld_to_clp_space.as_matrix_ref());

                glw::active_texture(glw::TEXTURE0);
                glw::bind_texture(glw::TEXTURE_2D_ARRAY, &self.texture_atlas_name);

                gl::DrawElementsInstanced(
                    gl::TRIANGLES,                         // mode
                    (cube::ELEMENT_DATA.len() * 3) as i32, // count
                    gl::UNSIGNED_INT,                      // index type
                    0 as *const ::std::os::raw::c_void,    // offset
                    chunk::CHUNK_TOTAL_BLOCKS as i32,      // primitive count
                );
            }
        }
    }

    pub unsafe fn delete(self) {
        let ChunkRenderer {
            texture_atlas_name,
            vertex_array_name,
            vertex_buffer_name,
            element_buffer_name,
            block_buffer_name,
            ..
        } = self;
        {
            let mut names = [Some(texture_atlas_name)];
            glw::delete_textures(&mut names);
        }
        {
            let mut names = [Some(vertex_array_name)];
            glw::delete_vertex_arrays(&mut names);
        }
        {
            let mut names = [
                Some(vertex_buffer_name),
                Some(element_buffer_name),
                Some(block_buffer_name),
            ];
            glw::delete_buffers(&mut names);
        }
    }
}
