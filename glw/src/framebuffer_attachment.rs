use gl;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FramebufferAttachment(u32);

impl FramebufferAttachment {
    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

pub const COLOR_ATTACHMENT0: FramebufferAttachment = FramebufferAttachment(gl::COLOR_ATTACHMENT0);
pub const DEPTH_ATTACHMENT: FramebufferAttachment = FramebufferAttachment(gl::DEPTH_ATTACHMENT);
pub const DEPTH_STENCIL_ATTACHMENT: FramebufferAttachment =
    FramebufferAttachment(gl::DEPTH_STENCIL_ATTACHMENT);
