#[macro_use]
extern crate lazy_static;

extern crate gl;
extern crate glutin;
extern crate glw;

#[allow(unused)]
#[macro_use]
mod support;

use glw::BufferName;
use glw::BufferNameArray;
use glw::OptionBufferNameArray;

serial_test!{fn gen_and_delete_buffers() {
    let (_events_loop, _window) = support::build_display();

    unsafe {
        let names = glw::gen_buffers_move::<[Option<BufferName>; 3]>().unwrap_all().unwrap();

        for (i, n) in names.iter().enumerate() {
            assert_eq!(n.as_u32(), i as u32 + 1);
        }

        glw::delete_buffers_move(names.wrap_all());
    }
}}
