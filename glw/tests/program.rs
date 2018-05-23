extern crate gl;
extern crate glutin;
extern crate glw;

mod support;

#[test]
fn create_a_program() {
    {
        let (_events_loop, _window) = support::build_display();

        let program_name = glw::ProgramName::new().unwrap();

        unsafe {
            assert_eq!(program_name.as_u32(), 1);
        }
    }

    {
        let (_events_loop, _window) = support::build_display();

        let program_name = glw::ProgramName::new().unwrap();

        unsafe {
            assert_eq!(program_name.as_u32(), 1);
        }
    }
}
