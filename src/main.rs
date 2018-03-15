/*
render block world
flying camera
paint blocks
 */

/*
x, y, z
 */
#![feature(nonzero)]
#![feature(duration_extras)]

extern crate cgmath;
extern crate core;
extern crate gl;
extern crate glutin;
extern crate image;

pub mod block;
pub mod chunk;
#[macro_use]
pub mod glw;
pub mod cube;
pub mod rate_counter;

use block::Block;
use cgmath::*;
use chunk::CHUNK_SIDE_BLOCKS;
use chunk::CHUNK_TOTAL_BLOCKS;
use chunk::Chunk;
use glutin::GlContext;
use glw::camera::Camera;
use glw::camera::CameraUpdate;
use glw::program::ProgramName;
use glw::program::ProgramSlot;
use glw::shader::FragmentShaderName;
use glw::shader::VertexShaderName;
use glw::viewport::Viewport;
use std::{thread, time};

fn main() {
    let mut chunk = Chunk {
        blocks: [Block::Void; CHUNK_TOTAL_BLOCKS],
        position_indices: Vector3 { x: 0, y: 0, z: 0 },
    };

    for z in 0..CHUNK_SIDE_BLOCKS {
        for x in 0..CHUNK_SIDE_BLOCKS {
            *chunk.block_at_mut(x, 0, z) = Block::Stone;
        }
    }

    for z in 5..CHUNK_SIDE_BLOCKS {
        for x in 10..13 {
            *chunk.block_at_mut(x, 1, z) = Block::Dirt;
        }
    }

    *chunk.block_at_mut(5, 10, 1) = Block::Stone;
    *chunk.block_at_mut(5, 10, 2) = Block::Dirt;

    let mut viewport = Viewport::new(1024, 768);

    let mut events_loop = glutin::EventsLoop::new();
    let gl_window = glutin::GlWindow::new(
        glutin::WindowBuilder::new()
            .with_title("Hello, world!")
            .with_dimensions(viewport.width() as u32, viewport.height() as u32),
        glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 0)))
            .with_gl_profile(glutin::GlProfile::Core)
            .with_vsync(true),
        &events_loop,
    ).unwrap();

    let mut program_slot = ProgramSlot;

    unsafe {
        gl_window.make_current().unwrap();
    }

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let triangle_program_name = ProgramName::new()
        .unwrap()
        .link(&[
            VertexShaderName::new()
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
            FragmentShaderName::new()
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
        let mut names: [u32; 1] = std::mem::uninitialized();
        gl::GenVertexArrays(names.len() as i32, names.as_mut_ptr());
        assert!(names[0] != 0, "Failed to create vertex array.");
        names[0]
    };

    let (triangle_vertex_buffer_name, triangle_element_buffer_name) = unsafe {
        let mut names: [u32; 2] = std::mem::uninitialized();
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
            std::mem::size_of_val(&cube::VERTEX_DATA) as isize,
            cube::VERTEX_DATA.as_ptr() as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        let vs_ver_pos_loc =
            gl::GetAttribLocation(triangle_program_name.as_u32(), gl_str!("vs_ver_pos"));
        assert!(vs_ver_pos_loc != -1, "Couldn't find position attribute");
        gl::EnableVertexAttribArray(vs_ver_pos_loc as u32);
        gl::VertexAttribPointer(
            vs_ver_pos_loc as u32,                      // index
            3,                                          // size (component count)
            gl::FLOAT,                                  // type (component type)
            gl::FALSE,                                  // normalized
            std::mem::size_of::<cube::Vertex>() as i32, // stride
            0 as *const std::os::raw::c_void,           // offset
        );

        let vs_tex_pos_loc =
            gl::GetAttribLocation(triangle_program_name.as_u32(), gl_str!("vs_tex_pos"));
        assert!(vs_tex_pos_loc != -1, "Couldn't find color attribute");
        gl::EnableVertexAttribArray(vs_tex_pos_loc as u32);
        gl::VertexAttribPointer(
            vs_tex_pos_loc as u32,                                              // index
            2,                                          // size (component count)
            gl::FLOAT,                                  // type (component type)
            gl::FALSE,                                  // normalized
            std::mem::size_of::<cube::Vertex>() as i32, // stride
            std::mem::size_of::<Vector3<f32>>() as *const std::os::raw::c_void, // offset
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, triangle_element_buffer_name);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&cube::ELEMENT_DATA) as isize,
            cube::ELEMENT_DATA.as_ptr() as *const std::os::raw::c_void,
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
                img.as_ptr() as *const std::os::raw::c_void,
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
                img.as_ptr() as *const std::os::raw::c_void,
            );
        }

        glw::generate_mipmap(glw::TEXTURE_2D);

        name
    };

    let mut should_stop = false;
    let mut has_focus = false;
    let mut current_width = 0;
    let mut current_height = 0;
    let mut current_fullscreen = false;

    let mut input_forward = glutin::ElementState::Released;
    let mut input_backward = glutin::ElementState::Released;
    let mut input_left = glutin::ElementState::Released;
    let mut input_right = glutin::ElementState::Released;
    let mut input_up = glutin::ElementState::Released;
    let mut input_down = glutin::ElementState::Released;

    const DESIRED_UPS: f64 = 153.0;
    const DESIRED_FPS: f64 = 60.0;

    let simulation_start = time::Instant::now();

    let mut next_update = simulation_start;
    let mut next_render = simulation_start;

    let mut r = 0.9;
    let mut g = 0.8;
    let mut b = 0.7;

    let mut fps_counter = rate_counter::RateCounter::with_capacity(30);
    #[allow(unused_assignments)]
    let mut fps = std::f64::NAN;
    let mut ups_counter = rate_counter::RateCounter::with_capacity(30);
    let mut ups = std::f64::NAN;

    let mut camera = Camera {
        position: Vector3 {
            x: 0.0,
            y: 2.0,
            z: 5.0,
        },
        yaw: Rad(0.0),
        pitch: Rad(0.0),
        fovy: Rad::from(Deg(45.0)),
        positional_velocity: 2.0,
        angular_velocity: 0.2,
        zoom_velocity: 0.3,
    };

    while !should_stop {
        let now = time::Instant::now();

        while next_update < now {
            // {
            //     let elapsed = time::Instant::now().duration_since(simulation_start);
            //     let elapsed_us = elapsed.as_secs() * 1_000_000 + elapsed.subsec_micros() as u64;
            //     println!("u\t{}", elapsed_us);
            // }

            let mut new_width = current_width;
            let mut new_height = current_height;
            let mut new_fullscreen = current_fullscreen;
            let mut mouse_dx = 0.0;
            let mut mouse_dy = 0.0;
            let mut mouse_dscroll = 0.0;

            events_loop.poll_events(|event| {
                use glutin::Event::*;
                match event {
                    WindowEvent { event, .. } => {
                        use glutin::ElementState::*;
                        use glutin::WindowEvent::*;
                        match event {
                            Closed => should_stop = true,
                            Resized(width, height) => {
                                new_width = width;
                                new_height = height;
                            }
                            KeyboardInput { input, .. } => {
                                use glutin::VirtualKeyCode::*;
                                match input.virtual_keycode {
                                    Some(Escape) => {
                                        if input.state == Pressed && has_focus {
                                            should_stop = true;
                                        }
                                    }
                                    Some(F11) => {
                                        if input.state == Pressed && has_focus {
                                            new_fullscreen = !new_fullscreen;
                                        }
                                    }
                                    Some(W) => input_forward = input.state,
                                    Some(S) => input_backward = input.state,
                                    Some(A) => input_left = input.state,
                                    Some(D) => input_right = input.state,
                                    Some(Q) => input_up = input.state,
                                    Some(Z) => input_down = input.state,
                                    _ => (),
                                }
                            }
                            Focused(state) => {
                                has_focus = state;
                            }
                            _ => (),
                        }
                    }
                    DeviceEvent {
                        device_id, event, ..
                    } => {
                        use glutin::DeviceEvent::*;
                        match event {
                            Added => println!("Added device {:?}", device_id),
                            Removed => println!("Removed device {:?}", device_id),
                            Motion { axis, value } => match axis {
                                0 => mouse_dx += value,
                                1 => mouse_dy += value,
                                3 => mouse_dscroll += value,
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    _ => (),
                }
            });

            use glutin::ElementState::*;

            camera.update(&CameraUpdate {
                delta_time: 1.0 / DESIRED_UPS as f32,
                delta_position: Vector3 {
                    x: match input_left {
                        Pressed => -1.0,
                        Released => 0.0,
                    } + match input_right {
                        Pressed => 1.0,
                        Released => 0.0,
                    },
                    y: match input_up {
                        Pressed => 1.0,
                        Released => 0.0,
                    } + match input_down {
                        Pressed => -1.0,
                        Released => 0.0,
                    },
                    z: match input_forward {
                        Pressed => -1.0,
                        Released => 0.0,
                    } + match input_backward {
                        Pressed => 1.0,
                        Released => 0.0,
                    },
                },
                delta_yaw: Rad(mouse_dx as f32),
                delta_pitch: Rad(mouse_dy as f32),
                delta_scroll: mouse_dscroll as f32,
            });

            if input_forward == Pressed {
                r += 1.0 / DESIRED_UPS as f32;
                if r > 1.0 {
                    r = 1.0;
                }
            }
            if input_backward == Pressed {
                r -= 1.0 / DESIRED_UPS as f32;
                if r < 0.0 {
                    r = 0.0;
                }
            }
            if input_left == Pressed {
                g += 1.0 / DESIRED_UPS as f32;
                if g > 1.0 {
                    g = 1.0;
                }
            }
            if input_right == Pressed {
                g -= 1.0 / DESIRED_UPS as f32;
                if g < 0.0 {
                    g = 0.0;
                }
            }
            if input_up == Pressed {
                b += 1.0 / DESIRED_UPS as f32;
                if b > 1.0 {
                    b = 1.0;
                }
            }
            if input_down == Pressed {
                b -= 1.0 / DESIRED_UPS as f32;
                if b < 0.0 {
                    b = 0.0;
                }
            }

            // Update render buffer sizes and stuff.
            if new_fullscreen != current_fullscreen {
                current_fullscreen = new_fullscreen;
                if current_fullscreen {
                    gl_window.set_fullscreen(Some(gl_window.get_current_monitor()));
                } else {
                    gl_window.set_fullscreen(None);
                }
            }

            if new_width != current_width || new_height != current_height {
                current_width = new_width;
                current_height = new_height;
                gl_window.resize(current_width, current_height);
                viewport
                    .update()
                    .width(current_width as i32)
                    .height(current_height as i32);
            }

            next_update += time::Duration::from_nanos((1000_000_000f64 / DESIRED_UPS) as u64);
            ups = ups_counter.update();
        }

        // Don't put any updates after this or we will miss them.
        if next_update < next_render {
            thread::sleep(next_update - now);
            continue;
        }

        // Render.
        // {
        //     let elapsed = time::Instant::now().duration_since(simulation_start);
        //     let elapsed_us = elapsed.as_secs() * 1_000_000 + elapsed.subsec_micros() as u64;
        //     println!("r\t{}", elapsed_us);
        // }

        unsafe {
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
        }

        let pos_from_wld_to_cam_space = camera.pos_from_wld_to_cam_space();

        let pos_from_cam_to_clp_space = Matrix4::from(PerspectiveFov {
            fovy: Rad::from(camera.fovy),
            aspect: viewport.aspect(),
            near: 0.1,
            far: 100.0,
        });

        let pos_from_wld_to_clp_space = pos_from_cam_to_clp_space * pos_from_wld_to_cam_space;

        {
            let _ = program_slot.bind(&triangle_program_name);

            for (position, block) in chunk.blocks() {
                match block {
                    Block::Void => continue,
                    Block::Stone => {
                        glw::active_texture(glw::TEXTURE0);
                        glw::bind_texture(glw::TEXTURE_2D, &stone_texture_name);
                    }
                    Block::Dirt => {
                        glw::active_texture(glw::TEXTURE0);
                        glw::bind_texture(glw::TEXTURE_2D, &dirt_texture_name);
                    }
                }

                let pos_from_obj_to_wld_space = Matrix4::from_translation(position);

                unsafe {
                    let loc = gl::GetUniformLocation(
                        triangle_program_name.as_u32(),
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
                        gl::TRIANGLES,                    // mode
                        12 * 3,                           // count
                        gl::UNSIGNED_INT,                 // index type,
                        0 as *const std::os::raw::c_void, // offset
                    );
                }
            }
        }

        gl_window.swap_buffers().unwrap();

        fps = fps_counter.update();

        gl_window.set_title(&format!("blocks {:.1} FPS {:.1} UPS", fps, ups));

        next_render = time::Instant::now()
            + time::Duration::from_nanos((1000_000_000f64 / DESIRED_FPS) as u64);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
