/*
render block world
flying camera
paint blocks
 */

/*
x, y, z
 */
#![feature(duration_extras)]

extern crate cgmath;
extern crate core;
extern crate gl;
extern crate glutin;
#[macro_use]
extern crate glw;
extern crate image;

pub mod assets;
pub mod block;
pub mod camera;
pub mod cgmath_ext;
pub mod chunk;
pub mod chunk_renderer;
pub mod cube;
pub mod rate_counter;
pub mod text_renderer;

use block::Block;
use cgmath::*;
use chunk::Chunk;
use chunk::CHUNK_SIDE_BLOCKS;
use chunk::CHUNK_TOTAL_BLOCKS;
use chunk_renderer::ChunkRenderer;
use glutin::GlContext;
use std::{thread, time};
use text_renderer::TextRenderer;

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

    let mut viewport = glw::Viewport::new(1024, 768);

    let mut events_loop = glutin::EventsLoop::new();
    let gl_window = glutin::GlWindow::new(
        glutin::WindowBuilder::new()
            .with_title(format!("{} {}", env!("CARGO_PKG_NAME"), env!("GIT_HASH")))
            .with_dimensions(viewport.width() as u32, viewport.height() as u32),
        glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 0)))
            .with_gl_profile(glutin::GlProfile::Core)
            .with_vsync(true),
        // .with_multisampling(16),
        &events_loop,
    ).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let chunk_renderer = ChunkRenderer::new();
    let text_renderer = TextRenderer::new();

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

    let mut camera = camera::Camera {
        position: Vector3 {
            x: 4.0,
            y: 0.0,
            z: 10.0,
        },
        yaw: Rad(0.0),
        pitch: Rad(0.0),
        fovy: Rad::from(Deg(45.0)),
        positional_velocity: 2.0,
        angular_velocity: 0.2,
        zoom_velocity: 0.3,
    };

    let mut user_input = String::new();
    let mut font_size = 20.0;

    while !should_stop {
        let now = time::Instant::now();

        while next_update < now {
            let mut new_width = current_width;
            let mut new_height = current_height;
            let mut new_fullscreen = current_fullscreen;
            let mut mouse_dx = 0.0;
            let mut mouse_dy = 0.0;
            let mut mouse_dscroll = 0.0;

            events_loop.poll_events(|event| {
                use glutin::Event;
                match event {
                    Event::WindowEvent { event, .. } => {
                        use glutin::ElementState;
                        use glutin::WindowEvent;
                        match event {
                            WindowEvent::CloseRequested => should_stop = true,
                            WindowEvent::Resized(width, height) => {
                                new_width = width;
                                new_height = height;
                            }
                            WindowEvent::KeyboardInput { input, .. } => {
                                use glutin::VirtualKeyCode;
                                match input.virtual_keycode {
                                    Some(VirtualKeyCode::Escape) => {
                                        if input.state == ElementState::Pressed && has_focus {
                                            should_stop = true;
                                        }
                                    }
                                    Some(VirtualKeyCode::F11) => {
                                        if input.state == ElementState::Pressed && has_focus {
                                            new_fullscreen = !new_fullscreen;
                                        }
                                    }
                                    Some(VirtualKeyCode::W) => input_forward = input.state,
                                    Some(VirtualKeyCode::S) => input_backward = input.state,
                                    Some(VirtualKeyCode::A) => input_left = input.state,
                                    Some(VirtualKeyCode::D) => input_right = input.state,
                                    Some(VirtualKeyCode::Q) => input_up = input.state,
                                    Some(VirtualKeyCode::Z) => input_down = input.state,
                                    Some(VirtualKeyCode::Add) => {
                                        if input.state == ElementState::Pressed && has_focus {
                                            font_size += 1.0;
                                            if font_size > 200.0 {
                                                font_size = 200.0;
                                            }
                                        }
                                    },
                                    Some(VirtualKeyCode::Subtract) => {
                                        if input.state == ElementState::Pressed && has_focus {
                                            font_size -= 1.0;
                                            if font_size < 1.0 {
                                                font_size = 1.0;
                                            }
                                        }
                                    },
                                    _ => (),
                                }
                            }
                            WindowEvent::ReceivedCharacter(c) => match c {
                                '\u{8}' => {
                                    user_input.pop();
                                }
                                _ => {
                                    user_input.push(c);
                                }
                            },
                            WindowEvent::Focused(state) => {
                                has_focus = state;
                            }
                            _ => (),
                        }
                    }
                    Event::DeviceEvent {
                        device_id, event, ..
                    } => {
                        use glutin::DeviceEvent;
                        match event {
                            DeviceEvent::Added => println!("Added device {:?}", device_id),
                            DeviceEvent::Removed => println!("Removed device {:?}", device_id),
                            DeviceEvent::Motion { axis, value } => match axis {
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

            use glutin::ElementState;

            camera.update(&camera::CameraUpdate {
                delta_time: 1.0 / DESIRED_UPS as f32,
                delta_position: Vector3 {
                    x: match input_left {
                        ElementState::Pressed => -1.0,
                        ElementState::Released => 0.0,
                    } + match input_right {
                        ElementState::Pressed => 1.0,
                        ElementState::Released => 0.0,
                    },
                    y: match input_up {
                        ElementState::Pressed => 1.0,
                        ElementState::Released => 0.0,
                    } + match input_down {
                        ElementState::Pressed => -1.0,
                        ElementState::Released => 0.0,
                    },
                    z: match input_forward {
                        ElementState::Pressed => -1.0,
                        ElementState::Released => 0.0,
                    } + match input_backward {
                        ElementState::Pressed => 1.0,
                        ElementState::Released => 0.0,
                    },
                },
                delta_yaw: Rad(mouse_dx as f32),
                delta_pitch: Rad(mouse_dy as f32),
                delta_scroll: mouse_dscroll as f32,
            });

            if input_forward == ElementState::Pressed {
                r += 1.0 / DESIRED_UPS as f32;
                if r > 1.0 {
                    r = 1.0;
                }
            }
            if input_backward == ElementState::Pressed {
                r -= 1.0 / DESIRED_UPS as f32;
                if r < 0.0 {
                    r = 0.0;
                }
            }
            if input_left == ElementState::Pressed {
                g += 1.0 / DESIRED_UPS as f32;
                if g > 1.0 {
                    g = 1.0;
                }
            }
            if input_right == ElementState::Pressed {
                g -= 1.0 / DESIRED_UPS as f32;
                if g < 0.0 {
                    g = 0.0;
                }
            }
            if input_up == ElementState::Pressed {
                b += 1.0 / DESIRED_UPS as f32;
                if b > 1.0 {
                    b = 1.0;
                }
            }
            if input_down == ElementState::Pressed {
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

        unsafe {
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            // gl::Enable(gl::MULTISAMPLE);
        }

        // Render scene.
        let pos_from_wld_to_cam_space = camera.pos_from_wld_to_cam_space();

        let pos_from_cam_to_clp_space = Matrix4::from(PerspectiveFov {
            fovy: Rad::from(camera.fovy),
            aspect: viewport.aspect(),
            near: 0.1,
            far: 100.0,
        });

        let pos_from_wld_to_clp_space = pos_from_cam_to_clp_space * pos_from_wld_to_cam_space;

        chunk_renderer.render(&pos_from_wld_to_clp_space, &chunk);

        // Render ui
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }

        // obj
        let pos_from_wld_to_clp_space = Matrix4::from(cgmath::Ortho {
            left: 0.0,
            right: viewport.width() as f32,
            bottom: viewport.height() as f32,
            top: 0.0,
            near: -5.0,
            far: 5.0,
        });

        {
            let s = format!(
                "{} {}, {:.0} FPS, {:.0} UPS",
                env!("CARGO_PKG_NAME"),
                env!("GIT_HASH"),
                fps,
                ups
            );

            text_renderer.render(
                &pos_from_wld_to_clp_space,
                &s,
                font_size,
                &text_renderer::Rect::from_dims(
                    font_size,
                    font_size,
                    viewport.width() as f32 - font_size,
                    viewport.height() as f32 - font_size,
                ),
            );
        }

        text_renderer.render(
            &pos_from_wld_to_clp_space,
            &user_input,
            font_size,
            &text_renderer::Rect::from_dims(
                font_size,
                font_size*3.0,
                viewport.width() as f32 - font_size,
                viewport.height() as f32 - font_size,
            ),
        );

        gl_window.swap_buffers().unwrap();

        fps = fps_counter.update();

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
