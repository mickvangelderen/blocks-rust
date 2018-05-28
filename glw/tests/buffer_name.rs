#[macro_use]
extern crate lazy_static;

extern crate gl;
extern crate glutin;
extern crate glw;

#[allow(unused)]
#[macro_use]
mod support;

use glw::BufferNameArray;

serial_test!{fn create_array() {
    let (_events_loop, _window) = support::build_display();
    unsafe {
        let names = <[Option<glw::BufferName>; 32]>::new();
        for (i, n) in names.iter().enumerate() {
            assert_eq!(n.as_ref().unwrap().as_u32(), i as u32 + 1);
        }
    }
}}
