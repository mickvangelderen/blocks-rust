use gl;

pub trait Param {
    fn as_u32(&self) -> u32;
}

pub struct ShaderType();

impl Param for ShaderType {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::SHADER_TYPE
    }
}

pub const SHADER_TYPE: ShaderType = ShaderType();

pub struct CompileStatus();

impl Param for CompileStatus {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::COMPILE_STATUS
    }
}

pub const COMPILE_STATUS: CompileStatus = CompileStatus();

pub struct ShaderSourceLength();

impl Param for ShaderSourceLength {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::SHADER_SOURCE_LENGTH
    }
}

pub const SHADER_SOURCE_LENGTH: ShaderSourceLength = ShaderSourceLength();

pub struct DeleteStatus();

impl Param for DeleteStatus {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::DELETE_STATUS
    }
}

pub const DELETE_STATUS: DeleteStatus = DeleteStatus();

pub struct LinkStatus();

impl Param for LinkStatus {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::LINK_STATUS
    }
}

pub const LINK_STATUS: LinkStatus = LinkStatus();

pub struct ValidateStatus();

impl Param for ValidateStatus {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::VALIDATE_STATUS
    }
}

pub const VALIDATE_STATUS: ValidateStatus = ValidateStatus();

pub struct InfoLogLength();

impl Param for InfoLogLength {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::INFO_LOG_LENGTH
    }
}

pub const INFO_LOG_LENGTH: InfoLogLength = InfoLogLength();

pub struct AttachedShaders();

impl Param for AttachedShaders {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::ATTACHED_SHADERS
    }
}

pub const ATTACHED_SHADERS: AttachedShaders = AttachedShaders();

pub struct ActiveAtomicCounterBuffers();

impl Param for ActiveAtomicCounterBuffers {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::ACTIVE_ATOMIC_COUNTER_BUFFERS
    }
}

pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: ActiveAtomicCounterBuffers = ActiveAtomicCounterBuffers();

pub struct ActiveAttributes();

impl Param for ActiveAttributes {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::ACTIVE_ATTRIBUTES
    }
}

pub const ACTIVE_ATTRIBUTES: ActiveAttributes = ActiveAttributes();

pub struct ActiveAttributeMaxLength();

impl Param for ActiveAttributeMaxLength {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::ACTIVE_ATTRIBUTE_MAX_LENGTH
    }
}

pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: ActiveAttributeMaxLength = ActiveAttributeMaxLength();

pub struct ActiveUniforms();

impl Param for ActiveUniforms {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::ACTIVE_UNIFORMS
    }
}

pub const ACTIVE_UNIFORMS: ActiveUniforms = ActiveUniforms();

pub struct ActiveUniformMaxLength();

impl Param for ActiveUniformMaxLength {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::ACTIVE_UNIFORM_MAX_LENGTH
    }
}

pub const ACTIVE_UNIFORM_MAX_LENGTH: ActiveUniformMaxLength = ActiveUniformMaxLength();

pub struct ProgramBinaryLength();

impl Param for ProgramBinaryLength {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::PROGRAM_BINARY_LENGTH
    }
}

pub const PROGRAM_BINARY_LENGTH: ProgramBinaryLength = ProgramBinaryLength();

pub struct ComputeWorkGroupSize();

impl Param for ComputeWorkGroupSize {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::COMPUTE_WORK_GROUP_SIZE
    }
}

pub const COMPUTE_WORK_GROUP_SIZE: ComputeWorkGroupSize = ComputeWorkGroupSize();

pub struct TransformFeedbackBufferMode();

impl Param for TransformFeedbackBufferMode {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::TRANSFORM_FEEDBACK_BUFFER_MODE
    }
}

pub const TRANSFORM_FEEDBACK_BUFFER_MODE: TransformFeedbackBufferMode =
    TransformFeedbackBufferMode();

pub struct TransformFeedbackVaryings();

impl Param for TransformFeedbackVaryings {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::TRANSFORM_FEEDBACK_VARYINGS
    }
}

pub const TRANSFORM_FEEDBACK_VARYINGS: TransformFeedbackVaryings = TransformFeedbackVaryings();

pub struct TransformFeedbackVaryingMaxLength();

impl Param for TransformFeedbackVaryingMaxLength {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH
    }
}

pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: TransformFeedbackVaryingMaxLength =
    TransformFeedbackVaryingMaxLength();

pub struct GeometryVerticesOut();

impl Param for GeometryVerticesOut {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::GEOMETRY_VERTICES_OUT
    }
}

pub const GEOMETRY_VERTICES_OUT: GeometryVerticesOut = GeometryVerticesOut();

pub struct GeometryInputType();

impl Param for GeometryInputType {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::GEOMETRY_INPUT_TYPE
    }
}

pub const GEOMETRY_INPUT_TYPE: GeometryInputType = GeometryInputType();

pub struct GeometryOutputType();

impl Param for GeometryOutputType {
    #[inline]
    fn as_u32(&self) -> u32 {
        gl::GEOMETRY_OUTPUT_TYPE
    }
}

pub const GEOMETRY_OUTPUT_TYPE: GeometryOutputType = GeometryOutputType();
