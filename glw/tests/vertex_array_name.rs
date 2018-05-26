#[macro_use]
extern crate lazy_static;

extern crate gl;
extern crate glutin;
extern crate glw;

#[allow(unused)]
#[macro_use]
mod support;

serial_test!{fn new_and_drop_dont_panic() {
    let (_events_loop, _window) = support::build_display();

    unsafe {
        support::clear_errors();

        {
            let name = glw::VertexArrayName::new().unwrap();
            std::mem::drop(name);
        }

        assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
}}

serial_test!{fn can_bind() {
    let (_events_loop, _window) = support::build_display();

    unsafe {
        support::clear_errors();

        {
            gl::BindVertexArray(1);
            assert_eq!(gl::GetError(), gl::INVALID_OPERATION);
        }

        {
            let name = glw::VertexArrayName::new().unwrap();
            gl::BindVertexArray(name.as_u32());
            assert_eq!(gl::GetError(), gl::NO_ERROR);
        }
    }
}}
