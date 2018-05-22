extern crate glutin;
extern crate gl;
extern crate glw;

use glutin::*;

#[test]
fn create_a_program() {
    let width: i32 = 256;
    let height: i32 = 256;
    let window = glutin::HeadlessRendererBuilder::new(width as u32, height as u32)
        .build()
        .unwrap();

    unsafe { window.make_current().expect("Couldn't make window current") };

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let program_name = glw::ProgramName::new().unwrap();
}
