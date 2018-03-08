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

pub mod block;
pub mod chunk;
#[macro_use]
pub mod glw;
pub mod cube;

use block::Block;
use cgmath::*;
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
    };

    for y in 0..32 {
        for x in 0..32 {
            *chunk.block_at_mut(x, y, 0) = Block::Rock;
        }
    }

    *chunk.block_at_mut(5, 10, 1) = Block::Rock;

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
#version 330 core

uniform mat4 pos_from_obj_to_clp_space;

in vec3 vs_position;
in vec3 vs_color;

out vec3 fs_color;

void main() {
    gl_Position = pos_from_obj_to_clp_space*vec4(vs_position, 1.0);
    fs_color = vs_color;
}
"#,
                ])
                .unwrap()
                .as_ref(),
            FragmentShaderName::new()
                .unwrap()
                .compile(&[
                    &r#"
#version 330 core

in vec3 fs_color;

out vec4 color;

void main() {
    color = vec4(fs_color, 1.0);
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

        let position_loc =
            gl::GetAttribLocation(triangle_program_name.as_u32(), gl_str!("vs_position"));
        assert!(position_loc != -1, "Couldn't find position attribute");
        gl::EnableVertexAttribArray(position_loc as u32);
        gl::VertexAttribPointer(
            position_loc as u32,                  // index
            3,                                    // size (component count)
            gl::FLOAT,                            // type (component type)
            gl::FALSE,                            // normalized
            std::mem::size_of::<cube::Vertex>() as i32, // stride
            0 as *const std::os::raw::c_void,     // offset
        );

        let color_loc = gl::GetAttribLocation(triangle_program_name.as_u32(), gl_str!("vs_color"));
        assert!(color_loc != -1, "Couldn't find color attribute");
        gl::EnableVertexAttribArray(color_loc as u32);
        gl::VertexAttribPointer(
            color_loc as u32,                                                   // index
            3,                                    // size (component count)
            gl::FLOAT,                            // type (component type)
            gl::FALSE,                            // normalized
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

    let mut frame_count = 0;
    let mut frame_count_start = simulation_start;
    let mut update_count: u64 = 0;

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

            update_count += 1;
            next_update += time::Duration::from_nanos((1000_000_000f64 / DESIRED_UPS) as u64);
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

            for i in 0..4 {
                let pos_from_obj_to_wld_space = Matrix4::from_translation(Vector3 {
                    x: 0.0,
                    y: i as f32 * 1.0,
                    z: i as f32 * 2.0,
                });

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

        frame_count += 1;

        if frame_count == 20 {
            // Calculate fps.
            let now = time::Instant::now();
            let elapsed = now - frame_count_start;
            let fps = 20.0 * 1_000_000.0
                / (elapsed.as_secs() * 1_000_000 + elapsed.subsec_micros() as u64) as f64;

            // Update window title.
            gl_window.set_title(&format!("blocks {:.1} FPS", fps));

            // Reset.
            frame_count = 0;
            frame_count_start = now;
        }

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
