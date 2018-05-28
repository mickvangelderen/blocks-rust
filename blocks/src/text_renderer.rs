use cgmath::*;
use cgmath_ext::*;
use gl;
use glw;
use image;

use glw::BufferNameArray;

struct Vertex {
    #[allow(unused)]
    ver_pos: Vector3<f32>,
    #[allow(unused)]
    tex_pos: Vector2<f32>,
}

const VER_POS_OFF: f32 = 0.5;

// +-----------+ <- box
// | +-------+ <- glyph + padding
// | | +---+ <- glyph
// | | | G | | |  ... repeated GLYPH_PER_ROW times
// | | +---+ | |
// | +-------+ |
// +-----------+

// NOTE(mickvangelderen): Depends on the font texture we are using.
// Assuming rectangular glyphs and rectangular texture here.
const GLYPHS_PER_SIDE: u32 = 16;

// NOTE(mickvangelderen): Depends on the font texture we are using.
const TEX_POS_OFF: f32 = {
    const BOX: u32 = 128;
    const GLYPH: u32 = 64;
    const PAD: u32 = 4;
    const SPACE: f32 = (BOX - GLYPH - 2 * PAD) as f32 / 2.0;
    SPACE / BOX as f32
};

static VERTEX_DATA: [Vertex; 4] = [
    Vertex {
        ver_pos: Vector3 {
            x: -VER_POS_OFF,
            y: -VER_POS_OFF,
            z: 0.0,
        },
        tex_pos: Vector2 {
            x: TEX_POS_OFF / GLYPHS_PER_SIDE as f32,
            y: TEX_POS_OFF / GLYPHS_PER_SIDE as f32,
        },
    },
    Vertex {
        ver_pos: Vector3 {
            x: VER_POS_OFF,
            y: -VER_POS_OFF,
            z: 0.0,
        },
        tex_pos: Vector2 {
            x: (1.0 - TEX_POS_OFF) / GLYPHS_PER_SIDE as f32,
            y: TEX_POS_OFF / GLYPHS_PER_SIDE as f32,
        },
    },
    Vertex {
        ver_pos: Vector3 {
            x: -VER_POS_OFF,
            y: VER_POS_OFF,
            z: 0.0,
        },
        tex_pos: Vector2 {
            x: TEX_POS_OFF / GLYPHS_PER_SIDE as f32,
            y: (1.0 - TEX_POS_OFF) / GLYPHS_PER_SIDE as f32,
        },
    },
    Vertex {
        ver_pos: Vector3 {
            x: VER_POS_OFF,
            y: VER_POS_OFF,
            z: 0.0,
        },
        tex_pos: Vector2 {
            x: (1.0 - TEX_POS_OFF) / GLYPHS_PER_SIDE as f32,
            y: (1.0 - TEX_POS_OFF) / GLYPHS_PER_SIDE as f32,
        },
    },
];

struct Triangle(u32, u32, u32);

static ELEMENT_DATA: [Triangle; 2] = [Triangle(2, 0, 1), Triangle(1, 3, 2)];

#[derive(Debug)]
struct Character {
    value: u32,
    offset: Vector2<f32>,
}

pub struct TextRenderer {
    program_name: glw::LinkedProgramName,
    _program_font_texture_loc: glw::UniformLocation<i32>,
    program_pos_from_wld_to_clp_space: glw::UniformLocation<[[f32; 4]; 4]>,
    texture_name: glw::TextureName,
    vertex_array_name: glw::VertexArrayName,
    _vertex_buffer_name: glw::BufferName,
    _element_buffer_name: glw::BufferName,
    character_buffer_name: glw::BufferName,
}

impl TextRenderer {
    pub fn new() -> Self {
        let program_name = glw::ProgramName::new()
            .unwrap()
            .link(&[
                glw::VertexShaderName::new()
                    .unwrap()
                    .compile(&[include_str!("text_renderer.vert")])
                    .unwrap_or_else(|err| {
                        panic!("\ntext_renderer.vert:\n{}", err);
                    })
                    .as_ref(),
                glw::FragmentShaderName::new()
                    .unwrap()
                    .compile(&[include_str!("text_renderer.frag")])
                    .unwrap_or_else(|err| {
                        panic!("\ntext_renderer.frag:\n{}", err);
                    })
                    .as_ref(),
            ])
            .unwrap();

        let program_font_texture_loc = unsafe {
            glw::UniformLocation::<i32>::new(&program_name, static_cstr!("font_texture")).unwrap()
        };

        let program_pos_from_wld_to_clp_space = unsafe {
            glw::UniformLocation::<[[f32; 4]; 4]>::new(
                &program_name,
                static_cstr!("pos_from_wld_to_clp_space"),
            ).unwrap()
        };

        let vertex_array_name =
            unsafe { glw::VertexArrayName::new().expect("Failed to create vertex array.") };

        let [vertex_buffer_name, element_buffer_name, character_buffer_name] =
            unsafe { <[Option<glw::BufferName>; 3]>::new() };

        let vertex_buffer_name = vertex_buffer_name.unwrap();
        let element_buffer_name = element_buffer_name.unwrap();
        let character_buffer_name = character_buffer_name.unwrap();

        unsafe {
            glw::use_program(&program_name);

            program_font_texture_loc.set(0);

            glw::bind_vertex_array(&vertex_array_name);

            // Set up vertex buffer.
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_name.as_u32());

            gl::BufferData(
                gl::ARRAY_BUFFER,
                ::std::mem::size_of_val(&VERTEX_DATA) as isize,
                VERTEX_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );

            // Associate vertex positions.
            let vs_ver_pos_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_ver_pos"));
            assert!(vs_ver_pos_loc != -1, "Couldn't find position attribute");
            gl::EnableVertexAttribArray(vs_ver_pos_loc as u32);
            gl::VertexAttribPointer(
                vs_ver_pos_loc as u32,                  // index
                3,                                      // size (component count)
                gl::FLOAT,                              // type (component type)
                gl::FALSE,                              // normalized
                ::std::mem::size_of::<Vertex>() as i32, // stride
                0 as *const ::std::os::raw::c_void,     // offset
            );

            // Associate texture coordinates.
            let vs_tex_pos_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_tex_pos"));
            assert!(vs_tex_pos_loc != -1, "Couldn't find color attribute");
            gl::EnableVertexAttribArray(vs_tex_pos_loc as u32);
            gl::VertexAttribPointer(
                vs_tex_pos_loc as u32,                                                  // index
                2,                                      // size (component count)
                gl::FLOAT,                              // type (component type)
                gl::FALSE,                              // normalized
                ::std::mem::size_of::<Vertex>() as i32, // stride
                ::std::mem::size_of::<Vector3<f32>>() as *const ::std::os::raw::c_void, // offset
            );

            // Set up character buffer.
            gl::BindBuffer(gl::ARRAY_BUFFER, character_buffer_name.as_u32());

            // We create the data dynamically from text. Not sure how to deal with allocation here.
            // gl::BufferData(
            //     gl::ARRAY_BUFFER,
            //     ::std::mem::size_of_val(&CHARACTER_DATA) as isize,
            //     CHARACTER_DATA.as_ptr() as *const ::std::os::raw::c_void,
            //     gl::STATIC_DRAW,
            // );

            {
                let loc = gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_char"));
                assert!(loc != -1, "Couldn't find vs_char attribute location");
                gl::EnableVertexAttribArray(loc as u32);
                gl::VertexAttribIPointer(
                    loc as u32,                                // index
                    1,                                         // size (component count)
                    gl::UNSIGNED_INT,                          // type (component type)
                    ::std::mem::size_of::<Character>() as i32, // stride
                    0 as *const ::std::os::raw::c_void,        // offset
                );
                gl::VertexAttribDivisor(
                    loc as u32, // index
                    1,          // advance every # instances
                );
            }

            {
                let loc = gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_char_offset"));
                assert!(loc != -1, "Couldn't find vs_char_offset attribute location");
                gl::EnableVertexAttribArray(loc as u32);
                gl::VertexAttribPointer(
                    loc as u32,                                // index
                    2,                                         // size (component count)
                    gl::FLOAT,                                 // type (component type)
                    gl::FALSE,                                 // normalize
                    ::std::mem::size_of::<Character>() as i32, // stride
                    4 as *const ::std::os::raw::c_void,        // offset
                );
                gl::VertexAttribDivisor(
                    loc as u32, // index
                    1,          // advance every # instances
                );
            }

            // Associate and set up element array.
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_name.as_u32());
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ::std::mem::size_of_val(&ELEMENT_DATA) as isize,
                ELEMENT_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );
        }

        let texture_name: glw::TextureName = unsafe {
            let name = {
                let mut names: [Option<glw::TextureName>; 1] = ::std::mem::uninitialized();
                glw::gen_textures(&mut names);
                let [name] = names;
                name.unwrap()
            };

            glw::bind_texture(glw::TEXTURE_2D, &name);

            glw::tex_parameter_i(
                glw::TEXTURE_2D_ARRAY,
                glw::TEXTURE_MIN_FILTER,
                glw::LINEAR_MIPMAP_LINEAR,
            );
            glw::tex_parameter_i(
                glw::TEXTURE_2D_ARRAY,
                glw::TEXTURE_MAG_FILTER,
                glw::LINEAR_MIPMAP_LINEAR,
            );
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
                let img = image::open("../assets/font-padded-sdf.png").unwrap();
                let img = img.flipv().to_rgba();
                gl::TexImage2D(
                    gl::TEXTURE_2D,                                // target
                    0,                                             // mipmap level
                    gl::RGBA8 as i32,                              // internal format
                    img.width() as i32,                            // width
                    img.height() as i32,                           // height
                    0,                                             // border (must be 0)
                    gl::RGBA,                                      // format
                    gl::UNSIGNED_BYTE,                             // type
                    img.as_ptr() as *const ::std::os::raw::c_void, // data
                );
            }

            glw::generate_mipmap(glw::TEXTURE_2D);

            name
        };

        TextRenderer {
            program_name,
            _program_font_texture_loc: program_font_texture_loc,
            program_pos_from_wld_to_clp_space,
            texture_name,
            vertex_array_name,
            _vertex_buffer_name: vertex_buffer_name,
            _element_buffer_name: element_buffer_name,
            character_buffer_name,
        }
    }

    pub fn render(&self, pos_from_wld_to_clp_space: &Matrix4<f32>, text: &str) {
        // Construct character buffer from text.
        let character_data = {
            let bytes = text.as_bytes();
            let mut character_data: Vec<Character> = Vec::with_capacity(bytes.len());
            let mut offset = Vector2 { x: 0.0, y: 0.0 };
            for &byte in bytes {
                match byte {
                    b' ' => {
                        offset.x += 1.0;
                    }
                    b'\n' | b'\r' => {
                        offset.x = 0.0;
                        offset.y += -1.0;
                    }
                    _ => {
                        character_data.push(Character {
                            value: byte as u32,
                            offset: offset,
                        });
                        offset.x += 1.0;
                    }
                }
            }
            character_data
        };

        unsafe {
            glw::use_program(&self.program_name);

            glw::bind_buffer(glw::ARRAY_BUFFER, &self.character_buffer_name);

            gl::BufferData(
                gl::ARRAY_BUFFER,                                                     // target
                (::std::mem::size_of::<Character>() * character_data.len()) as isize, // size
                character_data.as_ptr() as *const ::std::os::raw::c_void,             // data
                gl::STREAM_DRAW,                                                      // usage
            );
        }

        unsafe {
            glw::bind_vertex_array(&self.vertex_array_name);

            self.program_pos_from_wld_to_clp_space
                .set(pos_from_wld_to_clp_space.as_matrix_ref());

            // TODO: use 3d texture and lookup in shader
            glw::active_texture(glw::TEXTURE0);
            glw::bind_texture(glw::TEXTURE_2D, &self.texture_name);

            gl::DrawElementsInstanced(
                gl::TRIANGLES,                      // mode
                (ELEMENT_DATA.len() * 3) as i32,    // count
                gl::UNSIGNED_INT,                   // index type
                0 as *const ::std::os::raw::c_void, // offset
                character_data.len() as i32,        // primitive count
            );
        }
    }
}
