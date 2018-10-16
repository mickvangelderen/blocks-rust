#[macro_use]
extern crate lazy_static;

extern crate gl;
extern crate glutin;
extern crate glw;

#[allow(unused)]
#[macro_use]
mod support;

use glw::BufferName;

serial_test!{fn gen_and_delete_buffers() {
    let (_events_loop, _window) = support::build_display();

    unsafe {
        let mut names: [Option<BufferName>; 3] = Default::default();

        glw::gen_buffers(&mut names);

        for (i, n) in names.iter().enumerate() {
            assert_eq!(n.as_ref().unwrap().as_u32(), i as u32 + 1);
        }

        glw::delete_buffers(&mut names);

        for n in names.iter() {
            assert!(n.is_none());
        }
    }
}}
