use assets::Assets;
use cgmath::*;
use gl;
use glw;
use glw::prelude::*;
use program::*;
use renderer;
use shader::*;

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

pub struct PostRendererChanges {
    pub vert: bool,
    pub frag: bool,
}

impl PostRendererChanges {
    pub fn new() -> Self {
        PostRendererChanges {
            vert: false,
            frag: false,
        }
    }

    pub fn all() -> Self {
        PostRendererChanges {
            vert: true,
            frag: true,
        }
    }
}

pub struct PostRenderer<'a> {
    program: Program,
    #[allow(unused)]
    vertex_shader: VertexShader,
    #[allow(unused)]
    fragment_shader: FragmentShader,
    vertex_array_name: glw::VertexArrayName,
    #[allow(unused)]
    vertex_buffer_name: glw::BufferName,
    #[allow(unused)]
    element_buffer_name: glw::BufferName,
    color_texture_name: glw::SmallRef<'a, glw::TextureName>,
    depth_stencil_texture_name: glw::SmallRef<'a, glw::TextureName>,
    mode_loc: Option<glw::UniformLocation<i32>>,
    frustrum_x0_loc: Option<glw::UniformLocation<f32>>,
    frustrum_x1_loc: Option<glw::UniformLocation<f32>>,
    frustrum_y0_loc: Option<glw::UniformLocation<f32>>,
    frustrum_y1_loc: Option<glw::UniformLocation<f32>>,
    frustrum_z0_loc: Option<glw::UniformLocation<f32>>,
    frustrum_z1_loc: Option<glw::UniformLocation<f32>>,
    mouse_pos_loc: Option<glw::UniformLocation<[f32; 2]>>,
    viewport_loc: Option<glw::UniformLocation<[f32; 2]>>,
}

impl<'a> PostRenderer<'a> {
    pub fn new(
        assets: &Assets,
        color_texture_name: &'a glw::TextureName,
        depth_stencil_texture_name: &'a glw::TextureName,
    ) -> Self {
        unsafe {
            let program_name = glw::create_program().unwrap();
            let vertex_shader_name = glw::create_shader(glw::VERTEX_SHADER).unwrap();
            let fragment_shader_name = glw::create_shader(glw::FRAGMENT_SHADER).unwrap();

            glw::attach_shader(&program_name, vertex_shader_name.as_ref());
            glw::attach_shader(&program_name, fragment_shader_name.as_ref());

            let [vertex_buffer_name, element_buffer_name] =
                glw::gen_buffers_move::<[_; 2]>().unwrap_all().unwrap();

            let [vertex_array_name] = glw::gen_vertex_arrays_move::<[_; 1]>()
                .unwrap_all()
                .unwrap();

            let mut r = PostRenderer {
                program: Program::Unlinked(program_name),
                vertex_shader: VertexShader::Uncompiled(vertex_shader_name),
                fragment_shader: FragmentShader::Uncompiled(fragment_shader_name),
                color_texture_name: glw::SmallRef::new(color_texture_name),
                depth_stencil_texture_name: glw::SmallRef::new(depth_stencil_texture_name),
                vertex_array_name,
                vertex_buffer_name,
                element_buffer_name,
                mode_loc: None,
                frustrum_x0_loc: None,
                frustrum_x1_loc: None,
                frustrum_y0_loc: None,
                frustrum_y1_loc: None,
                frustrum_z0_loc: None,
                frustrum_z1_loc: None,
                mouse_pos_loc: None,
                viewport_loc: None,
            };

            r.update(assets, PostRendererChanges::all());

            r
        }
    }

    pub unsafe fn update(&mut self, assets: &Assets, changes: PostRendererChanges) {
        if changes.vert {
            renderer::recompile_and_log_vert(&assets.post_renderer_vert, &mut self.vertex_shader);
        }

        if changes.frag {
            renderer::recompile_and_log_frag(&assets.post_renderer_frag, &mut self.fragment_shader);
        }

        if (changes.vert || changes.frag)
            && if let VertexShader::Compiled(_) = self.vertex_shader {
                true
            } else {
                false
            }
            && if let FragmentShader::Compiled(_) = self.fragment_shader {
                true
            } else {
                false
            } {
            self.program.link();

            match self.program {
                Program::Unlinked(ref program_name) => {
                    let log = String::from_utf8(glw::get_program_info_log_move(program_name))
                        .expect("Program info log is not valid utf8.");
                    eprintln!("\nFailed to link program:\n{}", log);
                }
                Program::Linked(ref program_name) => {
                    glw::use_program(&program_name);

                    #[inline]
                    unsafe fn get_uniform_location_logged<T>(
                        program_name: &glw::ProgramName,
                        location: &std::ffi::CStr,
                    ) -> Option<glw::UniformLocation<T>> {
                        let loc = glw::get_uniform_location(program_name, location);
                        if let None = loc {
                            eprintln!(
                                "post_renderer.rs: Could not find uniform location {:?}.",
                                location
                            );
                        }
                        loc
                    }

                    #[inline]
                    unsafe fn get_attrib_location_logged(
                        program_name: &glw::ProgramName,
                        location: &std::ffi::CStr,
                    ) -> Option<glw::AttributeLocation> {
                        let loc = glw::get_attrib_location(program_name, location);
                        if let None = loc {
                            eprintln!(
                                "post_renderer.rs: Could not find attribute location {:?}.",
                                location
                            );
                        }
                        loc
                    }

                    if let Some(ref loc) =
                        get_uniform_location_logged(&program_name, static_cstr!("color_texture"))
                    {
                        glw::uniform_1i(loc, 0);
                    }

                    if let Some(ref loc) = get_uniform_location_logged(
                        &program_name,
                        static_cstr!("depth_stencil_texture"),
                    ) {
                        glw::uniform_1i(loc, 1);
                    }

                    self.mode_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("mode"));
                    self.frustrum_x0_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("frustrum.x0"));
                    self.frustrum_x1_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("frustrum.x1"));
                    self.frustrum_y0_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("frustrum.y0"));
                    self.frustrum_y1_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("frustrum.y1"));
                    self.frustrum_z0_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("frustrum.z0"));
                    self.frustrum_z1_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("frustrum.z1"));
                    self.viewport_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("viewport"));
                    self.mouse_pos_loc =
                        get_uniform_location_logged(&program_name, static_cstr!("mouse_pos"));

                    glw::bind_vertex_array(&self.vertex_array_name);

                    // Set up vertex buffer.
                    glw::bind_buffer(glw::ARRAY_BUFFER, &self.vertex_buffer_name);

                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        ::std::mem::size_of_val(&VERTEX_DATA) as isize,
                        VERTEX_DATA.as_ptr() as *const ::std::os::raw::c_void,
                        gl::STATIC_DRAW,
                    );

                    if let Some(loc) =
                        get_attrib_location_logged(&program_name, static_cstr!("vs_ver_pos"))
                    {
                        gl::EnableVertexAttribArray(loc.as_u32());
                        gl::VertexAttribPointer(
                            loc.as_u32(),                           // index
                            2,                                      // size (component count)
                            gl::FLOAT,                              // type (component type)
                            gl::FALSE,                              // normalized
                            ::std::mem::size_of::<Vertex>() as i32, // stride
                            0 as *const ::std::os::raw::c_void,     // offset
                        );
                    }

                    if let Some(loc) =
                        get_attrib_location_logged(&program_name, static_cstr!("vs_tex_pos"))
                    {
                        gl::EnableVertexAttribArray(loc.as_u32());
                        gl::VertexAttribPointer(
                            loc.as_u32(),                                                           // index
                            2,                                      // size (component count)
                            gl::FLOAT,                              // type (component type)
                            gl::FALSE,                              // normalized
                            ::std::mem::size_of::<Vertex>() as i32, // stride
                            ::std::mem::size_of::<Vector2<f32>>() as *const ::std::os::raw::c_void, // offset
                        );
                    }

                    // Set up element buffer.
                    glw::bind_buffer(glw::ELEMENT_ARRAY_BUFFER, &self.element_buffer_name);

                    gl::BufferData(
                        gl::ELEMENT_ARRAY_BUFFER,
                        ::std::mem::size_of_val(&ELEMENT_DATA) as isize,
                        ELEMENT_DATA.as_ptr() as *const ::std::os::raw::c_void,
                        gl::STATIC_DRAW,
                    );
                }
            }
        }
    }

    pub unsafe fn render(
        &self,
        mode: i32,
        frustrum: &Frustrum,
        viewport: &glw::Viewport,
        mouse: Vector2<f32>,
    ) {
        if let Program::Linked(ref program_name) = self.program {
            glw::use_program(program_name);

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
            glw::bind_texture(glw::TEXTURE_2D, &*self.color_texture_name);

            glw::active_texture(glw::TEXTURE1);
            glw::bind_texture(glw::TEXTURE_2D, &*self.depth_stencil_texture_name);

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
            program,
            vertex_shader,
            fragment_shader,
            vertex_array_name,
            vertex_buffer_name,
            element_buffer_name,
            ..
        } = self;
        fragment_shader.delete();
        vertex_shader.delete();
        program.delete();
        glw::delete_vertex_arrays_move([vertex_array_name].wrap_all());
        glw::delete_buffers_move([vertex_buffer_name, element_buffer_name].wrap_all());
    }
}
