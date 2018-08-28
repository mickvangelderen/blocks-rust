use std::num::NonZeroU32;
use std::mem::ManuallyDrop;

pub struct BufferName(NonZeroU32);

impl BufferName {
    #[inline]
    pub unsafe fn as_u32(&self) -> u32 {
        self.0.get()
    }
}

impl Drop for BufferName {
    #[inline]
    fn drop(&mut self) {
        if ::std::thread::panicking() == false {
            panic!("Forgot to free.");
        }
    }
}

pub struct Renderer {
    names: [BufferName; 2],
}

impl Renderer {
    pub fn new() -> Self {
        let names = unsafe {
            let mut names: ManuallyDrop<[Option<BufferName>; 2]> = ::std::mem::ManuallyDrop::new(::std::mem::uninitialized());
            gen_buffers(&mut *names);

            // Panic if any of the names is invalid.
            {
                names[0].as_ref().expect("Failed to acquire BufferName.");
                names[1].as_ref().expect("Failed to acquire BufferName.");
                assert_eq!(names.len(), 2);
            }

            // Remove the Option around each ManuallyDrop<BufferName>.
            ::std::mem::transmute::<
                ManuallyDrop<[Option<BufferName>; 2]>,
                [BufferName; 2]
            >(names)
        };

        Renderer {
            names,
        }
    }

    pub fn new2() -> Self {
        let names = unsafe {
            let mut names: ManuallyDrop<[Option<BufferName>; 2]> = ::std::mem::ManuallyDrop::new(::std::mem::uninitialized());
            gen_buffers(&mut *names);

            // NOTE: Generates code to drop names[1] if names[1] can't be unwrapped!
            [
                ::std::mem::replace(&mut names[0], ::std::mem::uninitialized()).expect("Failed to acquire BufferName."),
                ::std::mem::replace(&mut names[1], ::std::mem::uninitialized()).expect("Failed to acquire BufferName."),
            ]
        };

        Renderer {
            names,
        }
    }


    pub fn render(&self) {
        unsafe {
            do_something_with_buffer(self.vertex_buffer());
            do_something_with_buffer(self.element_buffer());
        }
    }

    #[inline]
    fn vertex_buffer(&self) -> &BufferName {
        &self.names[0]
    }

    #[inline]
    fn element_buffer(&self) -> &BufferName {
        &self.names[1]
    }
}

impl FreeMove for Renderer {
    #[inline]
    fn free(mut self) {
        unsafe {
            delete_buffers(&mut self.names);
        }
        ::std::mem::forget(self);
    }
}

impl Drop for Renderer {
    #[inline]
    fn drop(&mut self) {
        if ::std::thread::panicking() == false {
            panic!("Forgot to free.");
        }
    }
}

pub trait FreeMove {
    fn free(self);
}

pub trait FreeMut {
    fn free(&mut self);
}

impl<T: FreeMove> FreeMut for Option<T> {
    #[inline]
    fn free(&mut self) {
        if let Some(x) = self.take() {
            x.free();
        } else {
            panic!("Double free.");
        }
    }
}

extern {
    fn gen_buffers_ext(len: i32, ptr: *mut u32);
    fn delete_buffers_ext(len: i32, ptr: *mut u32);
    fn do_something_with_buffer_ext(name: u32);
    fn should_stop_ext() -> bool;
}

#[inline]
unsafe fn gen_buffers(names: &mut [Option<BufferName>]) {
    gen_buffers_ext(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

trait IsName {}
impl IsName for Option<ManuallyDrop<BufferName>> {}
impl IsName for Option<BufferName> {}
impl IsName for ManuallyDrop<BufferName> {}
impl IsName for BufferName {}

#[inline]
unsafe fn delete_buffers<T: IsName>(names: &mut [T]) {
    delete_buffers_ext(names.len() as i32, names.as_mut_ptr() as *mut u32);
}

#[inline]
unsafe fn do_something_with_buffer(name: &BufferName) {
    do_something_with_buffer_ext(name.as_u32());
}

#[inline]
fn should_stop() -> bool {
    unsafe {
        should_stop_ext()
    }
}

pub fn run() {
    let renderer = Renderer::new();

    while !should_stop() {
        renderer.render();
    }

    renderer.free();
}
