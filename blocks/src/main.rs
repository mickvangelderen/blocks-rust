/*
render block world
flying camera
paint blocks
 */

/*
x, y, z
 */

extern crate cgmath;
extern crate core;
extern crate gl;
extern crate glutin;
#[macro_use]
extern crate glw;
extern crate image;
extern crate notify;

pub mod assets;
pub mod block;
pub mod camera;
pub mod cgmath_ext;
pub mod chunk;
pub mod chunk_renderer;
pub mod console;
pub mod cube;
pub mod frustrum;
pub mod post_renderer;
pub mod rate_counter;
pub mod text_renderer;

use block::Block;
use cgmath::*;
use chunk::Chunk;
use chunk::CHUNK_SIDE_BLOCKS;
use chunk::CHUNK_TOTAL_BLOCKS;
use chunk_renderer::ChunkRenderer;
use frustrum::Frustrum;
use glutin::GlContext;
use post_renderer::PostRenderer;
use std::env;
use std::path::PathBuf;
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
            .with_dimensions(glutin::dpi::LogicalSize::new(
                viewport.width() as f64,
                viewport.height() as f64,
            )),
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

    let mut assets = assets::Assets::new(
        env::var_os("BLOCKS_ASSET_DIR").map_or_else(|| PathBuf::from("assets"), PathBuf::from),
    );

    let mut chunk_renderer = ChunkRenderer::new(&mut assets);
    let mut text_renderer = TextRenderer::new(&mut assets);

    let mut should_stop = false;
    let mut window_has_focus = false;
    let mut console_has_focus = false;
    let mut dpi_factor = gl_window.window().get_hidpi_factor();
    let mut window_size = glutin::dpi::PhysicalSize::new(0.0, 0.0);
    let mut is_fullscreen = false;

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

    #[repr(i32)]
    #[derive(Eq, PartialEq, Copy, Clone)]
    enum RenderMode {
        Color = 0,
        Depth = 1,
        Debug = 2,
    }

    impl RenderMode {
        fn next(&self) -> RenderMode {
            match *self {
                RenderMode::Color => RenderMode::Depth,
                RenderMode::Depth => RenderMode::Debug,
                RenderMode::Debug => RenderMode::Color,
            }
        }
    }

    let mut render_mode = RenderMode::Color;

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
            y: 2.0,
            z: 10.0,
        },
        yaw: Rad::from(Deg(60.0)),
        pitch: Rad::from(Deg(10.0)),
        fovy: Rad::from(Deg(45.0)),
        positional_velocity: 2.0,
        angular_velocity: 0.2,
        zoom_velocity: 0.3,
    };

    let mut mouse_pos = cgmath::Vector2::<f32>::zero();

    let mut console = console::Console::new();
    let mut font_size = 20.0;

    let color_texture_name = unsafe {
        let name = glw::TextureName::new().unwrap();

        glw::bind_texture(glw::TEXTURE_2D, &name);

        glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_MIN_FILTER, glw::LINEAR);
        glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_MAG_FILTER, glw::LINEAR);
        glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_WRAP_S, glw::CLAMP_TO_EDGE);
        glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_WRAP_T, glw::CLAMP_TO_EDGE);

        glw::tex_image_2d(
            glw::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            viewport.width(),
            viewport.height(),
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );

        name
    };

    let depth_stencil_texture_name = unsafe {
        let name = glw::TextureName::new().unwrap();

        glw::bind_texture(glw::TEXTURE_2D, &name);

        glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_MIN_FILTER, glw::NEAREST);
        glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_MAG_FILTER, glw::NEAREST);
        glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_WRAP_S, glw::CLAMP_TO_EDGE);
        glw::tex_parameter_i(glw::TEXTURE_2D, glw::TEXTURE_WRAP_T, glw::CLAMP_TO_EDGE);

        glw::tex_image_2d(
            glw::TEXTURE_2D,
            0,
            gl::DEPTH24_STENCIL8 as i32,
            viewport.width(),
            viewport.height(),
            gl::DEPTH_STENCIL,
            gl::UNSIGNED_INT_24_8,
            std::ptr::null(),
        );

        name
    };

    let framebuffer_name = unsafe {
        let name = glw::FramebufferName::new().unwrap();

        glw::bind_framebuffer(glw::FRAMEBUFFER, &name);

        glw::framebuffer_texture_2d(
            glw::FRAMEBUFFER,
            glw::COLOR_ATTACHMENT0,
            glw::TEXTURE_2D,
            &color_texture_name,
            0,
        );

        glw::framebuffer_texture_2d(
            glw::FRAMEBUFFER,
            glw::DEPTH_STENCIL_ATTACHMENT,
            glw::TEXTURE_2D,
            &depth_stencil_texture_name,
            0,
        );

        let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);

        assert_eq!(
            status,
            gl::FRAMEBUFFER_COMPLETE,
            "Expected framebufer to be complete."
        );

        name
    };

    let post_renderer = PostRenderer::new(
        &mut assets,
        &color_texture_name,
        &depth_stencil_texture_name,
    );

    while !should_stop {
        let now = time::Instant::now();

        while next_update < now {
            let mut new_fullscreen = is_fullscreen;
            let mut new_window_size = window_size;
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
                            WindowEvent::HiDpiFactorChanged(new_dpi_factor) => {
                                dpi_factor = new_dpi_factor;
                            }
                            WindowEvent::Resized(size) => {
                                // NOTE: Assume order of events is
                                // conserved and dpi_factor is the
                                // current right value.
                                new_window_size = size.to_physical(dpi_factor);
                            }
                            WindowEvent::KeyboardInput { input, .. } => {
                                use glutin::VirtualKeyCode;
                                match input.virtual_keycode {
                                    Some(VirtualKeyCode::Escape) => {
                                        if input.state == ElementState::Pressed {
                                            if console_has_focus {
                                                console_has_focus = false;
                                            } else if window_has_focus {
                                                should_stop = true;
                                            }
                                        }
                                    }
                                    Some(VirtualKeyCode::F11) => {
                                        if input.state == ElementState::Pressed && window_has_focus
                                        {
                                            new_fullscreen = !new_fullscreen;
                                        }
                                    }
                                    Some(VirtualKeyCode::W) => input_forward = input.state,
                                    Some(VirtualKeyCode::S) => input_backward = input.state,
                                    Some(VirtualKeyCode::A) => input_left = input.state,
                                    Some(VirtualKeyCode::D) => input_right = input.state,
                                    Some(VirtualKeyCode::Q) => input_up = input.state,
                                    Some(VirtualKeyCode::Z) => input_down = input.state,
                                    Some(VirtualKeyCode::R) => {
                                        if input.state == ElementState::Pressed
                                            && window_has_focus
                                            && !console_has_focus
                                        {
                                            render_mode = render_mode.next();
                                        }
                                    }
                                    Some(VirtualKeyCode::Slash) | Some(VirtualKeyCode::Grave) => {
                                        if input.state == ElementState::Pressed
                                            && window_has_focus
                                            && !console_has_focus
                                        {
                                            console_has_focus = true;
                                        }
                                    }
                                    Some(VirtualKeyCode::Add) => {
                                        if input.state == ElementState::Pressed
                                            && window_has_focus
                                            && !console_has_focus
                                        {
                                            font_size += 1.0;
                                            if font_size > 200.0 {
                                                font_size = 200.0;
                                            }
                                        }
                                    }
                                    Some(VirtualKeyCode::Subtract) => {
                                        if input.state == ElementState::Pressed
                                            && window_has_focus
                                            && !console_has_focus
                                        {
                                            font_size -= 1.0;
                                            if font_size < 1.0 {
                                                font_size = 1.0;
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            WindowEvent::ReceivedCharacter(c) => {
                                if window_has_focus && console_has_focus {
                                    console.write(c);
                                }
                            }
                            WindowEvent::Focused(state) => {
                                window_has_focus = state;
                            }
                            WindowEvent::CursorMoved { position, .. } => {
                                let position = position.to_physical(dpi_factor);
                                mouse_pos.x = position.x as f32;
                                mouse_pos.y = position.y as f32;
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

            console.parse_commands(|command| {
                use console::Command;
                match command {
                    Command::Invalid(content) => {
                        println!("Invalid command {:?}", content);
                    }
                    Command::Quit => {
                        should_stop = true;
                    }
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
            if new_fullscreen != is_fullscreen {
                is_fullscreen = new_fullscreen;
                gl_window.set_fullscreen(if is_fullscreen {
                    Some(gl_window.get_current_monitor())
                } else {
                    None
                });
            }


            if new_window_size != window_size {
                window_size = new_window_size;

                gl_window.resize(window_size);

                unsafe {
                    viewport
                        .update()
                        .width(window_size.width.round() as i32)
                        .height(window_size.height.round() as i32);
                }

                // Update framebuffer texture sizes.
                unsafe {
                    glw::bind_texture(glw::TEXTURE_2D, &color_texture_name);
                    glw::tex_image_2d(
                        glw::TEXTURE_2D,
                        0,
                        gl::RGBA8 as i32,
                        viewport.width(),
                        viewport.height(),
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        std::ptr::null(),
                    );

                    glw::bind_texture(glw::TEXTURE_2D, &depth_stencil_texture_name);
                    glw::tex_image_2d(
                        glw::TEXTURE_2D,
                        0,
                        gl::DEPTH24_STENCIL8 as i32,
                        viewport.width(),
                        viewport.height(),
                        gl::DEPTH_STENCIL,
                        gl::UNSIGNED_INT_24_8,
                        std::ptr::null(),
                    );
                }
            }

            next_update += time::Duration::from_nanos((1000_000_000f64 / DESIRED_UPS) as u64);
            ups = ups_counter.update();
        }

        // Don't put any update code after this.
        if next_update < next_render {
            thread::sleep(next_update - now);
            continue;
        }

        unsafe {
            glw::bind_framebuffer(glw::FRAMEBUFFER, &framebuffer_name);

            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            // gl::Enable(gl::MULTISAMPLE);
        }

        // Render scene.
        let pos_from_wld_to_cam_space = camera.pos_from_wld_to_cam_space();

        let frustrum = {
            let z0 = 0.2;
            let dy = z0 * Rad::tan(Rad::from(camera.fovy) / 2.0);
            let dx = dy * viewport.aspect();
            Frustrum {
                x0: -dx,
                x1: dx,
                y0: -dy,
                y1: dy,
                z0,
                z1: 100.0,
            }
        };

        let pos_from_cam_to_clp_space = Matrix4::from(Perspective {
            left: frustrum.x0,
            right: frustrum.x1,
            bottom: frustrum.y0,
            top: frustrum.y1,
            near: frustrum.z0,
            far: frustrum.z1,
        });

        let pos_from_wld_to_clp_space = pos_from_cam_to_clp_space * pos_from_wld_to_cam_space;

        chunk_renderer.render(&pos_from_wld_to_clp_space, &chunk);

        // Render ui
        unsafe {
            glw::bind_framebuffer(glw::FRAMEBUFFER, &glw::DEFAULT_FRAMEBUFFER_NAME);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Disable(gl::DEPTH_TEST);
        }

        post_renderer.render(render_mode as i32, &frustrum, &viewport, mouse_pos);

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

        if console_has_focus {
            text_renderer.render(
                &pos_from_wld_to_clp_space,
                console.input(),
                font_size,
                &text_renderer::Rect::from_dims(
                    font_size,
                    font_size * 3.0,
                    viewport.width() as f32 - font_size,
                    viewport.height() as f32 - font_size,
                ),
            );
        }

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
