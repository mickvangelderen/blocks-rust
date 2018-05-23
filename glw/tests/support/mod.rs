use super::gl;
use super::glutin;
use super::glutin::*;

pub const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 0));
pub const WINDOW_WIDTH: u32 = 1024;
pub const WINDOW_HEIGHT: u32 = 768;

pub fn build_display() -> (glutin::EventsLoop, glutin::GlWindow) {
    let events_loop = glutin::EventsLoop::new();
    let gl_window = glutin::GlWindow::new(
        glutin::WindowBuilder::new()
            .with_visibility(false)
            .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT),
        glutin::ContextBuilder::new()
            .with_gl_debug_flag(true)
            .with_gl(GL_VERSION)
            .with_gl_profile(glutin::GlProfile::Core),
        &events_loop,
    ).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    (events_loop, gl_window)
}
