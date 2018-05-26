use block::Block;
use cgmath::prelude::*;
use cgmath::Matrix4;
use cgmath::Vector3;
use chunk;
use chunk::Chunk;
use cube;
use gl;
use glw;
use image;

use glw::BufferNameArray;

pub struct ChunkRenderer {
    program_name: glw::LinkedProgramName,
    texture_atlas_name: glw::TextureName,
    vertex_array_name: u32,
    _vertex_buffer_name: glw::BufferName,
    _element_buffer_name: glw::BufferName,
    block_buffer_name: glw::BufferName,
}

impl ChunkRenderer {
    pub fn new() -> Self {
        let program_name = glw::ProgramName::new()
            .unwrap()
            .link(&[
                glw::VertexShaderName::new()
                    .unwrap()
                    .compile(&[include_str!("chunk_renderer.vert")])
                    .unwrap_or_else(|err| {
                        panic!("\nchunk_renderer.vert:\n{}", err);
                    })
                    .as_ref(),
                glw::FragmentShaderName::new()
                    .unwrap()
                    .compile(&[include_str!("chunk_renderer.frag")])
                    .unwrap_or_else(|err| {
                        panic!("\nchunk_renderer.frag:\n{}", err);
                    })
                    .as_ref(),
            ])
            .unwrap();

        let vertex_array_name = unsafe {
            let mut names: [u32; 1] = ::std::mem::uninitialized();
            gl::GenVertexArrays(names.len() as i32, names.as_mut_ptr());
            assert!(names[0] != 0, "Failed to create vertex array.");
            names[0]
        };

        let [vertex_buffer_name, element_buffer_name, block_buffer_name] =
            <[Option<glw::BufferName>; 3]>::new();
        let vertex_buffer_name = vertex_buffer_name.unwrap();
        let element_buffer_name = element_buffer_name.unwrap();
        let block_buffer_name = block_buffer_name.unwrap();

        unsafe {
            glw::use_program(&program_name);
            let texture_atlas_loc: i32 =
                gl::GetUniformLocation(program_name.as_u32(), gl_str!("texture_atlas"));
            gl::Uniform1i(texture_atlas_loc, 0);
        }

        unsafe {
            gl::BindVertexArray(vertex_array_name);

            // Set up vertex buffer.
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_name.as_u32());

            gl::BufferData(
                gl::ARRAY_BUFFER,
                ::std::mem::size_of_val(&cube::VERTEX_DATA) as isize,
                cube::VERTEX_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );

            // Associate vertex positions.
            let vs_ver_pos_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_ver_pos"));
            assert!(vs_ver_pos_loc != -1, "Couldn't find position attribute");
            gl::EnableVertexAttribArray(vs_ver_pos_loc as u32);
            gl::VertexAttribPointer(
                vs_ver_pos_loc as u32,                        // index
                3,                                            // size (component count)
                gl::FLOAT,                                    // type (component type)
                gl::FALSE,                                    // normalized
                ::std::mem::size_of::<cube::Vertex>() as i32, // stride
                0 as *const ::std::os::raw::c_void,           // offset
            );

            // Associate texture coordinates.
            let vs_tex_pos_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_tex_pos"));
            assert!(vs_tex_pos_loc != -1, "Couldn't find color attribute");
            gl::EnableVertexAttribArray(vs_tex_pos_loc as u32);
            gl::VertexAttribPointer(
                vs_tex_pos_loc as u32,                                                  // index
                2,                                            // size (component count)
                gl::FLOAT,                                    // type (component type)
                gl::FALSE,                                    // normalized
                ::std::mem::size_of::<cube::Vertex>() as i32, // stride
                ::std::mem::size_of::<Vector3<f32>>() as *const ::std::os::raw::c_void, // offset
            );

            // Set up block type buffer.
            gl::BindBuffer(gl::ARRAY_BUFFER, block_buffer_name.as_u32());

            gl::BufferData(
                gl::ARRAY_BUFFER,
                ::std::mem::size_of::<[Block; chunk::CHUNK_TOTAL_BLOCKS]>() as isize,
                ::std::ptr::null(),
                gl::STREAM_DRAW,
            );

            // Associate block type buffer with vertex attribute.
            let vs_blk_type_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_blk_type"));
            assert!(
                vs_blk_type_loc != -1,
                "Couldn't find vs_blk_type attribute location"
            );
            gl::EnableVertexAttribArray(vs_blk_type_loc as u32);
            gl::VertexAttribIPointer(
                vs_blk_type_loc as u32,                // index
                ::std::mem::size_of::<Block>() as i32, // size (component count)
                gl::UNSIGNED_BYTE,                     // type (component type)
                ::std::mem::size_of::<Block>() as i32, // stride
                0 as *const ::std::os::raw::c_void,    // offset
            );
            gl::VertexAttribDivisor(
                vs_blk_type_loc as u32, // index
                1,                      // advance every # instances
            );

            // Associate and set up element array.
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_name.as_u32());
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ::std::mem::size_of_val(&cube::ELEMENT_DATA) as isize,
                cube::ELEMENT_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );
        }

        let texture_atlas_name: glw::TextureName = {
            let name = unsafe {
                let mut names: [Option<glw::TextureName>; 1] = ::std::mem::uninitialized();
                glw::gen_textures(&mut names);

                // Move all values out of the array and forget about the array.
                let name = ::std::mem::replace(&mut names[0], ::std::mem::uninitialized());
                ::std::mem::forget(names);

                name.unwrap()
            };

            glw::bind_texture(glw::TEXTURE_2D_ARRAY, &name);
            unsafe {
                gl::TexStorage3D(
                    gl::TEXTURE_2D_ARRAY, // target
                    6,                    // levels
                    gl::RGBA8,            // internal format
                    32,                   // width
                    32,                   // height
                    2,                    // depth (layer count)
                );
            }

            glw::tex_parameter_min_filter(glw::TEXTURE_2D_ARRAY, glw::LINEAR_MIPMAP_LINEAR);
            glw::tex_parameter_mag_filter(glw::TEXTURE_2D_ARRAY, glw::NEAREST);
            glw::tex_parameter_wrap_s(glw::TEXTURE_2D_ARRAY, glw::CLAMP_TO_EDGE);
            glw::tex_parameter_wrap_t(glw::TEXTURE_2D_ARRAY, glw::CLAMP_TO_EDGE);

            unsafe {
                let img = image::open("../assets/stone_xyz.png").unwrap();
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

            unsafe {
                let img = image::open("../assets/dirt_xyz.png").unwrap();
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

            glw::generate_mipmap(glw::TEXTURE_2D_ARRAY);

            name
        };

        ChunkRenderer {
            program_name,
            texture_atlas_name,
            vertex_array_name,
            _vertex_buffer_name: vertex_buffer_name,
            _element_buffer_name: element_buffer_name,
            block_buffer_name,
        }
    }

    pub fn render(&self, pos_from_wld_to_clp_space: &Matrix4<f32>, chunk: &Chunk) {
        unsafe {
            // Update block type buffer.
            gl::BindBuffer(gl::ARRAY_BUFFER, self.block_buffer_name.as_u32());
            gl::BufferSubData(
                gl::ARRAY_BUFFER,                                                     // target
                0,                                                                    // offset
                ::std::mem::size_of::<[Block; chunk::CHUNK_TOTAL_BLOCKS]>() as isize, // size
                chunk.blocks.as_ptr() as *const ::std::os::raw::c_void,               // data
            );
        }

        glw::use_program(&self.program_name);

        unsafe {
            gl::BindVertexArray(self.vertex_array_name);
        }

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

        unsafe {
            {
                // Set pos_from_wld_to_clp_space.
                let loc = gl::GetUniformLocation(
                    self.program_name.as_u32(),
                    gl_str!("pos_from_wld_to_clp_space"),
                );
                assert!(loc != -1, "failed to query uniform location");
                gl::UniformMatrix4fv(
                    loc,                                // location
                    1,                                  // count
                    gl::FALSE,                          // row major
                    pos_from_wld_to_clp_space.as_ptr(), // data
                );
            }

            // TODO: use 3d texture and lookup in shader
            glw::active_texture(glw::TEXTURE0);
            glw::bind_texture(glw::TEXTURE_2D, &self.texture_atlas_name);

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
