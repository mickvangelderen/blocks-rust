use functions;
use gl;
use name::Name;

pub trait MaybeDefaultFramebufferName {
    unsafe fn as_u32(&self) -> u32;
}

pub struct DefaultFramebufferName();

pub const DEFAULT_FRAMEBUFFER_NAME: DefaultFramebufferName = DefaultFramebufferName();

impl MaybeDefaultFramebufferName for DefaultFramebufferName {
    #[inline]
    unsafe fn as_u32(&self) -> u32 {
        0
    }
}

pub struct FramebufferName(Name);

impl FramebufferName {
    #[inline]
    pub unsafe fn new() -> Option<Self> {
        let [name] = <[Option<Self>; 1]>::new();
        name
    }

    #[inline]
    pub unsafe fn as_u32(&self) -> u32 {
        self.0.get()
    }
}

impl Drop for FramebufferName {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.as_u32());
        }
    }
}

impl MaybeDefaultFramebufferName for FramebufferName {
    #[inline]
    unsafe fn as_u32(&self) -> u32 {
        FramebufferName::as_u32(self)
    }
}

pub trait FramebufferNameArray {
    unsafe fn new() -> Self;
}

macro_rules! impl_framebuffer_name_array {
    ($($N:expr)+) => {
        $(
            impl FramebufferNameArray for [Option<FramebufferName>; $N] {
                #[inline]
                unsafe fn new() -> Self {
                    let mut names: [Option<FramebufferName>; $N] = ::std::mem::uninitialized();
                    functions::gen_framebuffers(&mut names);
                    names
                }
            }
        )+
    }
}

impl_framebuffer_name_array! {
    0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}
