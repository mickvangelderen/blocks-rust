use cgmath::*;
use gl;
use glw;

use glw::BufferNameArray;

struct Vertex {
    #[allow(unused)]
    ver_pos: Vector2<f32>,
    #[allow(unused)]
    tex_pos: Vector2<f32>,
}

static VERTEX_DATA: [Vertex; 4] = [
    // SE
    Vertex {
        ver_pos: Vector2 { x: -1.0, y: -1.0 },
        tex_pos: Vector2 { x: 0.0, y: 0.0 },
    },
    // SW
    Vertex {
        ver_pos: Vector2 { x: 1.0, y: -1.0 },
        tex_pos: Vector2 { x: 1.0, y: 0.0 },
    },
    // NE
    Vertex {
        ver_pos: Vector2 { x: -1.0, y: 1.0 },
        tex_pos: Vector2 { x: 0.0, y: 1.0 },
    },
    // NW
    Vertex {
        ver_pos: Vector2 { x: 1.0, y: 1.0 },
        tex_pos: Vector2 { x: 1.0, y: 1.0 },
    },
];

static ELEMENT_DATA: [u32; 4] = [0, 1, 2, 3];

pub struct PostRenderer<'a> {
    program_name: glw::LinkedProgramName,
    z_near_loc: Option<glw::UniformLocation<f32>>,
    z_far_loc: Option<glw::UniformLocation<f32>>,
    color_texture_name: &'a glw::TextureName,
    depth_stencil_texture_name: &'a glw::TextureName,
    vertex_array_name: glw::VertexArrayName,
    _vertex_buffer_name: glw::BufferName,
    _element_buffer_name: glw::BufferName,
}

impl<'a> PostRenderer<'a> {
    pub fn new(
        color_texture_name: &'a glw::TextureName,
        depth_stencil_texture_name: &'a glw::TextureName,
    ) -> Self {
        let program_name = glw::ProgramName::new()
            .unwrap()
            .link(&[
                glw::VertexShaderName::new()
                    .unwrap()
                    .compile(&[include_str!("post_renderer.vert")])
                    .unwrap_or_else(|err| {
                        panic!("\npost_renderer.vert:\n{}", err);
                    })
                    .as_ref(),
                glw::FragmentShaderName::new()
                    .unwrap()
                    .compile(&[include_str!("post_renderer.frag")])
                    .unwrap_or_else(|err| {
                        panic!("\npost_renderer.frag:\n{}", err);
                    })
                    .as_ref(),
            ])
            .unwrap();

        let vertex_array_name =
            unsafe { glw::VertexArrayName::new().expect("Failed to create vertex array.") };

        let [vertex_buffer_name, element_buffer_name] =
            unsafe { <[Option<glw::BufferName>; 2]>::new() };

        let vertex_buffer_name = vertex_buffer_name.unwrap();
        let element_buffer_name = element_buffer_name.unwrap();

        let z_near_loc;
        let z_far_loc;

        unsafe {
            glw::use_program(&program_name);

            if let Some(color_texture_loc) =
                glw::UniformLocation::new(&program_name, static_cstr!("color_texture"))
            {
                glw::uniform_1i(&color_texture_loc, 0);
            } else {
                println!("Warning: Couldn't find color_texture_loc");
            }

            if let Some(depth_stencil_texture_loc) =
                glw::UniformLocation::new(&program_name, static_cstr!("depth_stencil_texture"))
            {
                glw::uniform_1i(&depth_stencil_texture_loc, 1);
            } else {
                println!("Warning: Couldn't find depth_stencil_texture_loc");
            }

            z_near_loc = glw::UniformLocation::<f32>::new(&program_name, static_cstr!("z_near"));
            z_far_loc = glw::UniformLocation::<f32>::new(&program_name, static_cstr!("z_far"));

            glw::bind_vertex_array(&vertex_array_name);

            // Set up array buffer.
            glw::bind_buffer(glw::ARRAY_BUFFER, &vertex_buffer_name);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                ::std::mem::size_of_val(&VERTEX_DATA) as isize,
                VERTEX_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );

            let vs_ver_pos_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_ver_pos"));
            assert!(vs_ver_pos_loc != -1, "Couldn't find vs_ver_pos attribute");
            gl::EnableVertexAttribArray(vs_ver_pos_loc as u32);
            gl::VertexAttribPointer(
                vs_ver_pos_loc as u32,                  // index
                2,                                      // size (component count)
                gl::FLOAT,                              // type (component type)
                gl::FALSE,                              // normalized
                ::std::mem::size_of::<Vertex>() as i32, // stride
                0 as *const ::std::os::raw::c_void,     // offset
            );

            let vs_tex_pos_loc =
                gl::GetAttribLocation(program_name.as_u32(), gl_str!("vs_tex_pos"));
            assert!(vs_tex_pos_loc != -1, "Couldn't find vs_tex_pos attribute");
            gl::EnableVertexAttribArray(vs_tex_pos_loc as u32);
            gl::VertexAttribPointer(
                vs_tex_pos_loc as u32,                                                  // index
                2,                                      // size (component count)
                gl::FLOAT,                              // type (component type)
                gl::FALSE,                              // normalized
                ::std::mem::size_of::<Vertex>() as i32, // stride
                ::std::mem::size_of::<Vector2<f32>>() as *const ::std::os::raw::c_void, // offset
            );

            // Set up element buffer.
            glw::bind_buffer(glw::ELEMENT_ARRAY_BUFFER, &element_buffer_name);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ::std::mem::size_of_val(&ELEMENT_DATA) as isize,
                ELEMENT_DATA.as_ptr() as *const ::std::os::raw::c_void,
                gl::STATIC_DRAW,
            );
        }

        PostRenderer {
            program_name,
            z_near_loc,
            z_far_loc,
            color_texture_name,
            depth_stencil_texture_name,
            vertex_array_name,
            _vertex_buffer_name: vertex_buffer_name,
            _element_buffer_name: element_buffer_name,
        }
    }

    pub fn render(&self, z_near: f32, z_far: f32) {
        unsafe {
            glw::use_program(&self.program_name);

            if let Some(ref loc) = self.z_near_loc {
                glw::uniform_1f(loc, z_near);
            }

            if let Some(ref loc) = self.z_far_loc {
                glw::uniform_1f(loc, z_far);
            }

            glw::active_texture(glw::TEXTURE0);
            glw::bind_texture(glw::TEXTURE_2D, self.color_texture_name);

            glw::active_texture(glw::TEXTURE1);
            glw::bind_texture(glw::TEXTURE_2D, self.depth_stencil_texture_name);

            glw::bind_vertex_array(&self.vertex_array_name);

            gl::DrawElements(
                gl::TRIANGLE_STRIP,                 // mode
                ELEMENT_DATA.len() as i32,          // count
                gl::UNSIGNED_INT,                   // index type
                0 as *const ::std::os::raw::c_void, // offset
            );
        }
    }
}
