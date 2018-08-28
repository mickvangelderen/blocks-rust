use gl;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum BufferTarget {
    ArrayBuffer = gl::ARRAY_BUFFER,
    AtomicCounterBuffer = gl::ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = gl::COPY_READ_BUFFER,
    CopyWriteBuffer = gl::COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = gl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = gl::DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = gl::PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,
    QueryBuffer = gl::QUERY_BUFFER,
    ShaderStorageBuffer = gl::SHADER_STORAGE_BUFFER,
    TextureBuffer = gl::TEXTURE_BUFFER,
    TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = gl::UNIFORM_BUFFER,
}

pub const ARRAY_BUFFER: BufferTarget = BufferTarget::ArrayBuffer;
pub const ATOMIC_COUNTER_BUFFER: BufferTarget = BufferTarget::AtomicCounterBuffer;
pub const COPY_READ_BUFFER: BufferTarget = BufferTarget::CopyReadBuffer;
pub const COPY_WRITE_BUFFER: BufferTarget = BufferTarget::CopyWriteBuffer;
pub const DISPATCH_INDIRECT_BUFFER: BufferTarget = BufferTarget::DispatchIndirectBuffer;
pub const DRAW_INDIRECT_BUFFER: BufferTarget = BufferTarget::DrawIndirectBuffer;
pub const ELEMENT_ARRAY_BUFFER: BufferTarget = BufferTarget::ElementArrayBuffer;
pub const PIXEL_PACK_BUFFER: BufferTarget = BufferTarget::PixelPackBuffer;
pub const PIXEL_UNPACK_BUFFER: BufferTarget = BufferTarget::PixelUnpackBuffer;
pub const QUERY_BUFFER: BufferTarget = BufferTarget::QueryBuffer;
pub const SHADER_STORAGE_BUFFER: BufferTarget = BufferTarget::ShaderStorageBuffer;
pub const TEXTURE_BUFFER: BufferTarget = BufferTarget::TextureBuffer;
pub const TRANSFORM_FEEDBACK_BUFFER: BufferTarget = BufferTarget::TransformFeedbackBuffer;
pub const UNIFORM_BUFFER: BufferTarget = BufferTarget::UniformBuffer;

impl BufferTarget {
    #[inline]
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}
