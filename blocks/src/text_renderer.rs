use assets::file_to_bytes;
use assets::Assets;
use cgmath::*;
use cgmath_ext::*;
use gl;
use glw;
use glw::prelude::*;
use image;
use program::*;
use shader::*;

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
    const PAD: u32 = 5;
    const SPACE: f32 = (BOX - GLYPH - 2 * PAD) as f32 / 2.0;
    SPACE / BOX as f32
};

static VERTEX_DATA: [Vertex; 4] = [
    Vertex {
        ver_pos: Vector3 {
            x: -VER_POS_OFF,
            y: VER_POS_OFF,
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
            y: VER_POS_OFF,
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
            y: -VER_POS_OFF,
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
            y: -VER_POS_OFF,
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

#[derive(Debug)]
pub struct Rect {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

impl Rect {
    /// x1, y1 are exclusive.
    pub fn from_coords(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        assert!(x1 > x0);
        assert!(y1 > y0);
        Rect { x0, y0, x1, y1 }
    }

    pub fn from_dims(x0: f32, y0: f32, width: f32, height: f32) -> Self {
        assert!(width > 0.0);
        assert!(height > 0.0);
        Rect {
            x0,
            y0,
            x1: x0 + width,
            y1: y0 + height,
        }
    }
}

pub struct TextRenderer {
    program: Program,
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader,
    #[allow(unused)]
    program_font_texture_loc: Option<glw::UniformLocation<i32>>,
    program_pos_from_wld_to_clp_space_loc: Option<glw::UniformLocation<[f32; 16]>>,
    program_font_size_loc: Option<glw::UniformLocation<f32>>,
    texture_name: glw::TextureName,
    vertex_array_name: glw::VertexArrayName,
    #[allow(unused)]
    vertex_buffer_name: glw::BufferName,
    #[allow(unused)]
    element_buffer_name: glw::BufferName,
    character_buffer_name: glw::BufferName,
}

impl TextRenderer {
    pub unsafe fn new(assets: &Assets) -> Self {
        let program_name = glw::create_program().unwrap();
        let vertex_shader_name = glw::create_shader(glw::VERTEX_SHADER).unwrap();
        let fragment_shader_name = glw::create_shader(glw::FRAGMENT_SHADER).unwrap();

        glw::attach_shader(&program_name, vertex_shader_name.as_ref());
        glw::attach_shader(&program_name, fragment_shader_name.as_ref());

        let mut program = Program::Unlinked(program_name);
        let mut vertex_shader = VertexShader::Uncompiled(vertex_shader_name);
        let mut fragment_shader = FragmentShader::Uncompiled(fragment_shader_name);

        vertex_shader.compile(&[&file_to_bytes(&assets.text_renderer_vert).unwrap()[..]]);
        fragment_shader.compile(&[&file_to_bytes(&assets.text_renderer_vert).unwrap()[..]]);
        program.link();

        let [vertex_buffer_name, element_buffer_name, character_buffer_name] =
            glw::gen_buffers_move::<[_; 3]>().unwrap_all().unwrap();

        let [vertex_array_name] = glw::gen_vertex_arrays_move::<[_; 1]>()
            .unwrap_all()
            .unwrap();

        let texture_name: glw::TextureName = {
            let [name] = glw::gen_textures_move::<[_; 1]>().unwrap_all().unwrap();

            glw::bind_texture(glw::TEXTURE_2D, &name);

            glw::tex_parameter_i(
                glw::TEXTURE_2D,
                glw::TEXTURE_MIN_FILTER,
                glw::LINEAR_MIPMAP_LINEAR,
            );
            glw::tex_parameter_i(
                glw::TEXTURE_2D,
                glw::TEXTURE_MAG_FILTER,
                glw::LINEAR_MIPMAP_LINEAR,
            );
            glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_WRAP_S, glw::CLAMP_TO_EDGE);
            glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_WRAP_T, glw::CLAMP_TO_EDGE);

            {
                let img = image::open(&assets.font_padded_sdf_png).unwrap();
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

        let program_font_texture_loc;
        let program_pos_from_wld_to_clp_space_loc;
        let program_font_size_loc;

        if let Program::Linked(ref program_name) = program {
            // macro_rules! get_uniform_loc {
            //     ($type:ty, $identifier:tt) => {
            //         glw::get_uniform_location::<$type>(&program_name, static_cstr!($identifier))
            //             .unwrap_or_else(|| {
            //                 panic!("Failed to get uniform location {:?}", $identifier);
            //             })
            //     };
            // }

            program_font_texture_loc =
                glw::get_uniform_location(program_name, static_cstr!("font_texture"));

            program_pos_from_wld_to_clp_space_loc =
                glw::get_uniform_location(program_name, static_cstr!("pos_from_wld_to_clp_space"));

            program_font_size_loc =
                glw::get_uniform_location(program_name, static_cstr!("font_size"));

            glw::use_program(&program_name);

            if let Some(ref loc) = program_font_texture_loc {
                glw::uniform_1i(loc, 0);
            }

            glw::bind_vertex_array(&vertex_array_name);

            // Set up vertex buffer.
            glw::bind_buffer(glw::ARRAY_BUFFER, &vertex_buffer_name);

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
            glw::bind_buffer(glw::ARRAY_BUFFER, &character_buffer_name);

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
            glw::bind_buffer(glw::ELEMENT_ARRAY_BUFFER, &element_buffer_name);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ::std::mem::size_of_val(&ELEMENT_DATA) as isize,
                ELEMENT_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );
        } else {
            program_font_texture_loc = None;
            program_pos_from_wld_to_clp_space_loc = None;
            program_font_size_loc = None;
        }

        TextRenderer {
            program,
            vertex_shader,
            fragment_shader,
            program_font_texture_loc,
            program_pos_from_wld_to_clp_space_loc,
            program_font_size_loc,
            texture_name,
            vertex_array_name,
            vertex_buffer_name,
            element_buffer_name,
            character_buffer_name,
        }
    }

    pub unsafe fn render(
        &self,
        pos_from_wld_to_clp_space: &Matrix4<f32>,
        text: &str,
        font_size: f32,
        bounds: &Rect,
    ) {
        // Construct character buffer from text.
        let character_data = {
            let bytes = text.as_bytes();
            let mut character_data: Vec<Character> = Vec::with_capacity(bytes.len());
            let mut offset = Vector2 {
                x: bounds.x0,
                y: bounds.y0,
            };

            fn inc_x(offset: &mut Vector2<f32>, font_size: f32, bounds: &Rect) {
                offset.x += font_size;
                // Hard wrap.
                if offset.x >= bounds.x1 {
                    inc_y(offset, font_size, bounds);
                }
            }

            fn inc_y(offset: &mut Vector2<f32>, font_size: f32, bounds: &Rect) {
                offset.x = bounds.x0;
                offset.y += font_size;
            }

            for &byte in bytes {
                match byte {
                    b' ' => {
                        inc_x(&mut offset, font_size, bounds);
                    }
                    b'\n' | b'\r' => {
                        inc_y(&mut offset, font_size, bounds);
                    }
                    _ => {
                        // y might not be in bounds, check before adding.
                        if offset.y < bounds.y1 {
                            character_data.push(Character {
                                value: byte as u32,
                                offset: offset,
                            });
                        }
                        inc_x(&mut offset, font_size, bounds);
                    }
                }
            }
            character_data
        };

        if let Program::Linked(ref program_name) = self.program {
            glw::use_program(&program_name);

            // Update uniforms.
            if let Some(ref loc) = self.program_font_size_loc {
                glw::uniform_1f(loc, font_size);
            }

            if let Some(ref loc) = self.program_pos_from_wld_to_clp_space_loc {
                glw::uniform_matrix4f(loc, pos_from_wld_to_clp_space.as_matrix_ref());
            }

            // Update character buffer.
            glw::bind_buffer(glw::ARRAY_BUFFER, &self.character_buffer_name);

            gl::BufferData(
                gl::ARRAY_BUFFER,                                                     // target
                (::std::mem::size_of::<Character>() * character_data.len()) as isize, // size
                character_data.as_ptr() as *const ::std::os::raw::c_void,             // data
                gl::STREAM_DRAW,                                                      // usage
            );

            glw::bind_vertex_array(&self.vertex_array_name);

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

    pub unsafe fn delete(self) {
        let TextRenderer {
            texture_name,
            vertex_array_name,
            vertex_buffer_name,
            element_buffer_name,
            character_buffer_name,
            ..
        } = self;
        {
            let mut names = [Some(texture_name)];
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
                Some(character_buffer_name),
            ];
            glw::delete_buffers(&mut names);
        }
    }
}
