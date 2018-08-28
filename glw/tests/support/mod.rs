use gl;
use glutin;
use glutin::*;
use std::collections::HashMap;

pub const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 0));

pub fn build_display() -> (glutin::EventsLoop, glutin::GlWindow) {
    let events_loop = glutin::EventsLoop::new();
    let gl_window = glutin::GlWindow::new(
        glutin::WindowBuilder::new()
            .with_visibility(false)
            .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0)),
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

pub fn clear_errors() {
    let mut error_to_count: HashMap<u32, usize> = HashMap::new();
    for _ in 0..1000 {
        let error = unsafe { gl::GetError() };
        if error == gl::NO_ERROR {
            return;
        }
        error_to_count
            .entry(error)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    panic!("Unable to clear OpenGL errors {:?}", error_to_count);
}

// Work around tests being run in parallel by default.
// https://github.com/rust-lang/rust/issues/43155

use std::sync::Mutex;

lazy_static! {
    pub static ref SERIAL_TEST_MUTEX: Mutex<()> = Mutex::new(());
}

macro_rules! serial_test {
    (fn $name:ident() $body:block) => {
        #[test]
        fn $name() {
            let guard = $crate::support::SERIAL_TEST_MUTEX.lock().unwrap();
            if let Err(e) = std::panic::catch_unwind(|| $body) {
                drop(guard);
                std::panic::resume_unwind(e);
            }
        }
    };
}
