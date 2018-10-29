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
            let mut names: [Option<glw::VertexArrayName>; 1] = Default::default();
            glw::gen_vertex_arrays(&mut names);
            glw::delete_vertex_arrays(&mut names);
            ::std::mem::forget(names);
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
            let mut names: [Option<glw::VertexArrayName>; 1] = Default::default();
            glw::gen_vertex_arrays(&mut names);

            {
                let name: &glw::VertexArrayName = names[0].as_ref().unwrap();
                gl::BindVertexArray(name.as_u32());
                assert_eq!(gl::GetError(), gl::NO_ERROR);
            }

            glw::delete_vertex_arrays(&mut names);
            ::std::mem::forget(names);
        }
    }
}}
