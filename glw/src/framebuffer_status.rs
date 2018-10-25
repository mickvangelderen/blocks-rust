use gl;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum FramebufferStatus {
    /// GL_COMPLETE is returned if the specified framebuffer is complete.
    Complete = gl::FRAMEBUFFER_COMPLETE,

    /// GL_FRAMEBUFFER_UNDEFINED is returned if the specified framebuffer is the
    /// default read or draw framebuffer, but the default framebuffer does not
    /// exist.
    Undefined = gl::FRAMEBUFFER_UNDEFINED,

    /// GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT is returned if any of the
    /// framebuffer attachment points are framebuffer incomplete.
    IncompleteAttachment = gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT,

    /// GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT is returned if the
    /// framebuffer does not have at least one image attached to it.
    IncompleteMissingAttachment = gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,

    /// GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER is returned if the value of
    /// GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE for any color
    /// attachment point(s) named by GL_DRAW_BUFFERi.
    IncompleteDrawBuffer = gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,

    /// GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER is returned if GL_READ_BUFFER is
    /// not GL_NONE and the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is
    /// GL_NONE for the color attachment point named by GL_READ_BUFFER.
    IncompleteReadBuffer = gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER,

    /// GL_FRAMEBUFFER_UNSUPPORTED is returned if the combination of internal
    /// formats of the attached images violates an implementation-dependent set
    /// of restrictions.
    Unsupported = gl::FRAMEBUFFER_UNSUPPORTED,

    /// GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE is returned if the value of
    /// GL_RENDERBUFFER_SAMPLES is not the same for all attached renderbuffers;
    /// if the value of GL_TEXTURE_SAMPLES is the not same for all attached
    /// textures; or, if the attached images are a mix of renderbuffers and
    /// textures, the value of GL_RENDERBUFFER_SAMPLES does not match the value
    /// of GL_TEXTURE_SAMPLES.

    /// GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE is also returned if the value of
    /// GL_TEXTURE_FIXED_SAMPLE_LOCATIONS is not the same for all attached
    /// textures; or, if the attached images are a mix of renderbuffers and
    /// textures, the value of GL_TEXTURE_FIXED_SAMPLE_LOCATIONS is not GL_TRUE
    /// for all attached textures.
    IncompleteMultisample = gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,

    /// GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS is returned if any framebuffer
    /// attachment is layered, and any populated attachment is not layered, or
    /// if all populated color attachments are not from textures of the same
    /// target.
    IncompleteLayerTargets = gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
}

pub const FRAMEBUFFER_COMPLETE: FramebufferStatus = FramebufferStatus::Complete;
pub const FRAMEBUFFER_UNDEFINED: FramebufferStatus = FramebufferStatus::Undefined;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: FramebufferStatus =
    FramebufferStatus::IncompleteAttachment;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: FramebufferStatus =
    FramebufferStatus::IncompleteMissingAttachment;
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: FramebufferStatus =
    FramebufferStatus::IncompleteDrawBuffer;
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: FramebufferStatus =
    FramebufferStatus::IncompleteReadBuffer;
pub const FRAMEBUFFER_UNSUPPORTED: FramebufferStatus = FramebufferStatus::Unsupported;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: FramebufferStatus =
    FramebufferStatus::IncompleteMultisample;

impl FramebufferStatus {
    #[inline]
    pub unsafe fn from_raw(value: u32) -> Option<Self> {
        ::std::mem::transmute(value)
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}
