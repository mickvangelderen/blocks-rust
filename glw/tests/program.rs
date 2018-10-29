#[macro_use]
extern crate lazy_static;

extern crate gl;
extern crate glutin;
extern crate glw;

#[allow(unused)]
#[macro_use]
mod support;

serial_test!{fn create_a_program() {
    let (_events_loop, _window) = support::build_display();

    unsafe {
        let program_name = glw::create_program().unwrap();

        assert_eq!(program_name.as_u32(), 1);

        glw::delete_program(program_name);
    }
}}
