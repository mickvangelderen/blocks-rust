use assets::file_to_string;
use assets::Assets;
use cgmath::*;
use gl;
use glw;

use frustrum::Frustrum;

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
    mode_loc: Option<glw::UniformLocation<i32>>,
    frustrum_x0_loc: Option<glw::UniformLocation<f32>>,
    frustrum_x1_loc: Option<glw::UniformLocation<f32>>,
    frustrum_y0_loc: Option<glw::UniformLocation<f32>>,
    frustrum_y1_loc: Option<glw::UniformLocation<f32>>,
    frustrum_z0_loc: Option<glw::UniformLocation<f32>>,
    frustrum_z1_loc: Option<glw::UniformLocation<f32>>,
    mouse_pos_loc: Option<glw::UniformLocation<[f32; 2]>>,
    viewport_loc: Option<glw::UniformLocation<[f32; 2]>>,
    color_texture_name: &'a glw::TextureName,
    depth_stencil_texture_name: &'a glw::TextureName,
    vertex_array_name: glw::VertexArrayName,
    #[allow(unused)]
    vertex_buffer_name: glw::BufferName,
    #[allow(unused)]
    element_buffer_name: glw::BufferName,
}

impl<'a> PostRenderer<'a> {
    pub fn new(
        assets: &mut Assets,
        color_texture_name: &'a glw::TextureName,
        depth_stencil_texture_name: &'a glw::TextureName,
    ) -> Self {
        unsafe {
            let program_name = glw::ProgramName::new()
                .unwrap()
                .link(&[
                    glw::VertexShaderName::new()
                        .unwrap()
                        .compile(&[&file_to_string(assets.get_path("post_renderer.vert")).unwrap()])
                        .unwrap_or_else(|(_, err)| {
                            panic!("\npost_renderer.vert:\n{}", err);
                        })
                        .as_ref(),
                    glw::FragmentShaderName::new()
                        .unwrap()
                        .compile(&[&file_to_string(assets.get_path("post_renderer.frag")).unwrap()])
                        .unwrap_or_else(|(_, err)| {
                            panic!("\npost_renderer.frag:\n{}", err);
                        })
                        .as_ref(),
                ])
                .unwrap();

            let vertex_array_name =
                glw::VertexArrayName::new().expect("Failed to create vertex array.");

            let [vertex_buffer_name, element_buffer_name] = {
                let mut names: [Option<glw::BufferName>; 2] = ::std::mem::uninitialized();
                glw::gen_buffers(&mut names);
                [names[0].take().unwrap(), names[1].take().unwrap()]
            };

            let mode_loc;
            let frustrum_x0_loc;
            let frustrum_x1_loc;
            let frustrum_y0_loc;
            let frustrum_y1_loc;
            let frustrum_z0_loc;
            let frustrum_z1_loc;
            let viewport_loc;
            let mouse_pos_loc;

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

            mode_loc = glw::UniformLocation::<i32>::new(&program_name, static_cstr!("mode"));
            frustrum_x0_loc =
                glw::UniformLocation::<f32>::new(&program_name, static_cstr!("frustrum.x0"));
            frustrum_x1_loc =
                glw::UniformLocation::<f32>::new(&program_name, static_cstr!("frustrum.x1"));
            frustrum_y0_loc =
                glw::UniformLocation::<f32>::new(&program_name, static_cstr!("frustrum.y0"));
            frustrum_y1_loc =
                glw::UniformLocation::<f32>::new(&program_name, static_cstr!("frustrum.y1"));
            frustrum_z0_loc =
                glw::UniformLocation::<f32>::new(&program_name, static_cstr!("frustrum.z0"));
            frustrum_z1_loc =
                glw::UniformLocation::<f32>::new(&program_name, static_cstr!("frustrum.z1"));

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
            viewport_loc =
                glw::UniformLocation::<[f32; 2]>::new(&program_name, static_cstr!("viewport"));
            mouse_pos_loc =
                glw::UniformLocation::<[f32; 2]>::new(&program_name, static_cstr!("mouse_pos"));

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

            PostRenderer {
                program_name,
                mode_loc,
                frustrum_x0_loc,
                frustrum_x1_loc,
                frustrum_y0_loc,
                frustrum_y1_loc,
                frustrum_z0_loc,
                frustrum_z1_loc,
                mouse_pos_loc,
                viewport_loc,
                color_texture_name,
                depth_stencil_texture_name,
                vertex_array_name,
                vertex_buffer_name,
                element_buffer_name,
            }
        }
    }

    pub fn render(
        &self,
        mode: i32,
        frustrum: &Frustrum,
        viewport: &glw::Viewport,
        mouse: Vector2<f32>,
    ) {
        unsafe {
            glw::use_program(&self.program_name);

            if let Some(ref loc) = self.mode_loc {
                glw::uniform_1i(loc, mode);
            }

            if let Some(ref loc) = self.frustrum_x0_loc {
                glw::uniform_1f(loc, frustrum.x0);
            }

            if let Some(ref loc) = self.frustrum_x1_loc {
                glw::uniform_1f(loc, frustrum.x1);
            }

            if let Some(ref loc) = self.frustrum_y0_loc {
                glw::uniform_1f(loc, frustrum.y0);
            }

            if let Some(ref loc) = self.frustrum_y1_loc {
                glw::uniform_1f(loc, frustrum.y1);
            }

            if let Some(ref loc) = self.frustrum_z0_loc {
                glw::uniform_1f(loc, frustrum.z0);
            }

            if let Some(ref loc) = self.frustrum_z1_loc {
                glw::uniform_1f(loc, frustrum.z1);
            }

            if let Some(ref loc) = self.viewport_loc {
                glw::uniform_2f(loc, [viewport.width() as f32, viewport.height() as f32]);
            }

            if let Some(ref loc) = self.mouse_pos_loc {
                glw::uniform_2f(loc, [mouse.x, mouse.y]);
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

    pub unsafe fn delete(self) {
        let PostRenderer {
            vertex_buffer_name,
            element_buffer_name,
            ..
        } = self;
        let mut buffer_names = [Some(vertex_buffer_name), Some(element_buffer_name)];
        glw::delete_buffers(&mut buffer_names);
        ::std::mem::forget(buffer_names);
    }
}
