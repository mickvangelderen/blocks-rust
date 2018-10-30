use gl;

pub trait GetProgramivParam {
    type Value;

    fn as_u32() -> u32;
}

/// params returns GL_TRUE if program is currently flagged for deletion,
/// and GL_FALSE otherwise.
pub struct DeleteStatus();

impl GetProgramivParam for DeleteStatus {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::DELETE_STATUS
    }
}

pub const DELETE_STATUS: DeleteStatus = DeleteStatus();

/// params returns GL_TRUE if the last link operation on program was
/// successful, and GL_FALSE otherwise.
pub struct LinkStatus();

impl GetProgramivParam for LinkStatus {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::LINK_STATUS
    }
}

pub const LINK_STATUS: LinkStatus = LinkStatus();

/// params returns GL_TRUE or if the last validation operation on
/// program was successful, and GL_FALSE otherwise.
pub struct ValidateStatus();

impl GetProgramivParam for ValidateStatus {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::VALIDATE_STATUS
    }
}

pub const VALIDATE_STATUS: ValidateStatus = ValidateStatus();

/// params returns the number of characters in the information log for
/// program including the null termination character (i.e., the size of
/// the character buffer required to store the information log). If
/// program has no information log, a value of 0 is returned.
pub struct InfoLogLength();

impl GetProgramivParam for InfoLogLength {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::INFO_LOG_LENGTH
    }
}

pub const INFO_LOG_LENGTH: InfoLogLength = InfoLogLength();

/// params returns the number of shader objects attached to program.
pub struct AttachedShaders();

impl GetProgramivParam for AttachedShaders {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::ATTACHED_SHADERS
    }
}

pub const ATTACHED_SHADERS: AttachedShaders = AttachedShaders();

/// params returns the number of active attribute atomic counter buffers
/// used by program.
pub struct ActiveAtomicCounterBuffers();

impl GetProgramivParam for ActiveAtomicCounterBuffers {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::ACTIVE_ATOMIC_COUNTER_BUFFERS
    }
}

pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: ActiveAtomicCounterBuffers = ActiveAtomicCounterBuffers();

/// params returns the number of active attribute variables for program.
pub struct ActiveAttributes();

impl GetProgramivParam for ActiveAttributes {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::ACTIVE_ATTRIBUTES
    }
}

pub const ACTIVE_ATTRIBUTES: ActiveAttributes = ActiveAttributes();

/// params returns the length of the longest active attribute name for
/// program, including the null termination character (i.e., the size of
/// the character buffer required to store the longest attribute name).
/// If no active attributes exist, 0 is returned.
pub struct ActiveAttributeMaxLength();

impl GetProgramivParam for ActiveAttributeMaxLength {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::ACTIVE_ATTRIBUTE_MAX_LENGTH
    }
}

pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: ActiveAttributeMaxLength = ActiveAttributeMaxLength();

/// params returns the number of active uniform variables for program.
pub struct ActiveUniforms();

impl GetProgramivParam for ActiveUniforms {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::ACTIVE_UNIFORMS
    }
}

pub const ACTIVE_UNIFORMS: ActiveUniforms = ActiveUniforms();

/// params returns the length of the longest active uniform variable
/// name for program, including the null termination character (i.e.,
/// the size of the character buffer required to store the longest
/// uniform variable name). If no active uniform variables exist, 0 is
/// returned.
pub struct ActiveUniformMaxLength();

impl GetProgramivParam for ActiveUniformMaxLength {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::ACTIVE_UNIFORM_MAX_LENGTH
    }
}

pub const ACTIVE_UNIFORM_MAX_LENGTH: ActiveUniformMaxLength = ActiveUniformMaxLength();

/// params returns the length of the program binary, in bytes that will
/// be returned by a call to glGetProgramBinary. When a progam's
/// GL_LINK_STATUS is GL_FALSE, its program binary length is zero.
pub struct ProgramBinaryLength();

impl GetProgramivParam for ProgramBinaryLength {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::PROGRAM_BINARY_LENGTH
    }
}

pub const PROGRAM_BINARY_LENGTH: ProgramBinaryLength = ProgramBinaryLength();

/// params returns an array of three integers containing the local work
/// group size of the compute program as specified by its input layout
/// qualifier(s). program must be the name of a program object that has
/// been previously linked successfully and contains a binary for the
/// compute shader stage.
pub struct ComputeWorkGroupSize();

impl GetProgramivParam for ComputeWorkGroupSize {
    type Value = [i32; 3];

    fn as_u32() -> u32 {
        gl::COMPUTE_WORK_GROUP_SIZE
    }
}

pub const COMPUTE_WORK_GROUP_SIZE: ComputeWorkGroupSize = ComputeWorkGroupSize();

/// params returns a symbolic constant indicating the buffer mode used
/// when transform feedback is active. This may be GL_SEPARATE_ATTRIBS
/// or GL_INTERLEAVED_ATTRIBS.
pub struct TransformFeedbackBufferMode();

impl GetProgramivParam for TransformFeedbackBufferMode {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::TRANSFORM_FEEDBACK_BUFFER_MODE
    }
}

pub const TRANSFORM_FEEDBACK_BUFFER_MODE: TransformFeedbackBufferMode = TransformFeedbackBufferMode();

/// params returns the number of varying variables to capture in
/// transform feedback mode for the program.
pub struct TransformFeedbackVaryings();

impl GetProgramivParam for TransformFeedbackVaryings {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::TRANSFORM_FEEDBACK_VARYINGS
    }
}

pub const TRANSFORM_FEEDBACK_VARYINGS: TransformFeedbackVaryings = TransformFeedbackVaryings();

/// params returns the length of the longest variable name to be used
/// for transform feedback, including the null-terminator.
pub struct TransformFeedbackVaryingMaxLength();

impl GetProgramivParam for TransformFeedbackVaryingMaxLength {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH
    }
}

pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: TransformFeedbackVaryingMaxLength = TransformFeedbackVaryingMaxLength();

/// params returns the maximum number of vertices that the geometry
/// shader in program will output.
pub struct GeometryVerticesOut();

impl GetProgramivParam for GeometryVerticesOut {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::GEOMETRY_VERTICES_OUT
    }
}

pub const GEOMETRY_VERTICES_OUT: GeometryVerticesOut = GeometryVerticesOut();

/// params returns a symbolic constant indicating the primitive type
/// accepted as input to the geometry shader contained in program.
pub struct GeometryInputType();

impl GetProgramivParam for GeometryInputType {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::GEOMETRY_INPUT_TYPE
    }
}

pub const GEOMETRY_INPUT_TYPE: GeometryInputType = GeometryInputType();

/// params returns a symbolic constant indicating the primitive type
/// that will be output by the geometry shader contained in program.
pub struct GeometryOutputType();

impl GetProgramivParam for GeometryOutputType {
    type Value = i32;

    fn as_u32() -> u32 {
        gl::GEOMETRY_OUTPUT_TYPE
    }
}

pub const GEOMETRY_OUTPUT_TYPE: GeometryOutputType = GeometryOutputType();
