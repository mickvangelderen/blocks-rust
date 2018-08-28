#[macro_use]
extern crate lazy_static;

extern crate gl;
extern crate glutin;
extern crate glw;

#[allow(unused)]
#[macro_use]
mod support;

use glw::BufferName;
use std::mem::ManuallyDrop;

serial_test!{fn gen_and_delete_buffers() {
    let (_events_loop, _window) = support::build_display();

    unsafe {
        let mut names: ManuallyDrop<[Option<BufferName>; 3]> = ManuallyDrop::new(::std::mem::uninitialized());

        glw::gen_buffers(&mut *names);

        for (i, n) in names.iter().enumerate() {
            assert_eq!(n.as_ref().unwrap().as_u32(), i as u32 + 1);
        }

        glw::delete_buffers(&mut *names);
    }
}}

serial_test!{fn gen_and_delete_buffers_with_drop() {
    let (_events_loop, _window) = support::build_display();

    unsafe {
        let mut names: ManuallyDrop<[Option<BufferName>; 3]> = ManuallyDrop::new([None, None, None]);

        glw::gen_buffers_with_drop(&mut *names);

        for (i, n) in names.iter().enumerate() {
            assert_eq!(n.as_ref().unwrap().as_u32(), i as u32 + 1);
        }

        glw::delete_buffers_with_drop(&mut *names);

        for n in names.iter() {
            assert!(n.is_none());
        }
    }
}}

serial_test!{fn gen_and_delete_buffers_vec_with_drop() {
    let (_events_loop, _window) = support::build_display();

    unsafe {
        let mut names: ManuallyDrop<Vec<Option<BufferName>>> = ManuallyDrop::new(vec![None, None, None]);

        glw::gen_buffers_with_drop(&mut *names);

        for (i, n) in names.iter().enumerate() {
            assert_eq!(n.as_ref().unwrap().as_u32(), i as u32 + 1);
        }

        glw::delete_buffers_with_drop(&mut *names);

        for n in names.iter() {
            assert!(n.is_none());
        }
    }
}}
