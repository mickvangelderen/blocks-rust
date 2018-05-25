extern crate gl;
extern crate glutin;
extern crate glw;

mod support;

use glw::BufferNameArray;

#[test]
fn create_array() {
    let (_events_loop, _window) = support::build_display();
    let names = <[Option<glw::BufferName>; 32]>::new();
    for (i, n) in names.iter().enumerate() {
        unsafe {
            assert_eq!(n.as_ref().unwrap().as_u32(), i as u32 + 1);
        }
    }
}
