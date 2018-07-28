use assets;
use block::Block;
use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath_ext::*;
use chunk;
use chunk::Chunk;
use cube;
use gl;
use glw;
use image;

use glw::BufferNameArray;

pub struct ChunkRenderer {
    program_name: glw::LinkedProgramName,
    program_pos_from_wld_to_clp_space: glw::UniformLocation<[[f32; 4]; 4]>,
    texture_atlas_name: glw::TextureName,
    vertex_array_name: glw::VertexArrayName,
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

        let program_pos_from_wld_to_clp_space = unsafe {
            glw::UniformLocation::<[[f32; 4]; 4]>::new(
                &program_name,
                static_cstr!("pos_from_wld_to_clp_space"),
            ).unwrap()
        };

        let vertex_array_name = unsafe { glw::VertexArrayName::new().unwrap() };

        let [vertex_buffer_name, element_buffer_name, block_buffer_name] =
            unsafe { <[Option<glw::BufferName>; 3]>::new() };
        let vertex_buffer_name = vertex_buffer_name.unwrap();
        let element_buffer_name = element_buffer_name.unwrap();
        let block_buffer_name = block_buffer_name.unwrap();

        unsafe {
            glw::use_program(&program_name);
            let texture_atlas_loc =
                glw::UniformLocation::<i32>::new(&program_name, static_cstr!("texture_atlas"))
                    .unwrap();
            texture_atlas_loc.set(0);
        }

        unsafe {
            glw::bind_vertex_array(&vertex_array_name);

            // Set up vertex buffer.
            glw::bind_buffer(glw::ARRAY_BUFFER, &vertex_buffer_name);

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
            glw::bind_buffer(glw::ARRAY_BUFFER, &block_buffer_name);

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
            glw::bind_buffer(glw::ELEMENT_ARRAY_BUFFER, &element_buffer_name);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ::std::mem::size_of_val(&cube::ELEMENT_DATA) as isize,
                cube::ELEMENT_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );
        }

        let texture_atlas_name = unsafe {
            let name = glw::TextureName::new().unwrap();

            glw::bind_texture(glw::TEXTURE_2D_ARRAY, &name);

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

            {
                let img = image::open(assets::get_asset_path("stone_xyz.png")).unwrap();
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

            {
                let img = image::open(assets::get_asset_path("dirt_xyz.png")).unwrap();
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
            program_pos_from_wld_to_clp_space,
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
            glw::bind_buffer(glw::ARRAY_BUFFER, &self.block_buffer_name);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,                                                     // target
                0,                                                                    // offset
                ::std::mem::size_of::<[Block; chunk::CHUNK_TOTAL_BLOCKS]>() as isize, // size
                chunk.blocks.as_ptr() as *const ::std::os::raw::c_void,               // data
            );

            glw::use_program(&self.program_name);

            glw::bind_vertex_array(&self.vertex_array_name);
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
            self.program_pos_from_wld_to_clp_space
                .set(pos_from_wld_to_clp_space.as_matrix_ref());

            // TODO: use 3d texture and lookup in shader
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
