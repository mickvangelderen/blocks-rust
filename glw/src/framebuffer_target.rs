use gl;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum FramebufferTarget {
    DrawFramebuffer = gl::DRAW_FRAMEBUFFER,
    ReadFramebuffer = gl::READ_FRAMEBUFFER,
    Framebuffer = gl::FRAMEBUFFER,
}

pub const DRAW_FRAMEBUFFER: FramebufferTarget = FramebufferTarget::DrawFramebuffer;
pub const READ_FRAMEBUFFER: FramebufferTarget = FramebufferTarget::ReadFramebuffer;
pub const FRAMEBUFFER: FramebufferTarget = FramebufferTarget::Framebuffer;

impl FramebufferTarget {
    #[inline]
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}
