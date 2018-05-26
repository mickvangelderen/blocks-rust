extern crate gl;
extern crate glutin;
extern crate glw;

mod support;

#[test]
fn new_and_drop_dont_panic() {
    let (_events_loop, _window) = support::build_display();
    unsafe {
        let name = glw::VertexArrayName::new().unwrap();
        assert_eq!(name.as_u32(), 1);
    }
}
