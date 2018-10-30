use param::*;

pub trait GetProgramivParam: Param {
    type Value;
}

impl GetProgramivParam for DeleteStatus {
    type Value = i32;
}

impl GetProgramivParam for LinkStatus {
    type Value = i32;
}

impl GetProgramivParam for ValidateStatus {
    type Value = i32;
}

impl GetProgramivParam for InfoLogLength {
    type Value = i32;
}

impl GetProgramivParam for AttachedShaders {
    type Value = i32;
}

impl GetProgramivParam for ActiveAtomicCounterBuffers {
    type Value = i32;
}

impl GetProgramivParam for ActiveAttributes {
    type Value = i32;
}

impl GetProgramivParam for ActiveAttributeMaxLength {
    type Value = i32;
}

impl GetProgramivParam for ActiveUniforms {
    type Value = i32;
}

impl GetProgramivParam for ActiveUniformMaxLength {
    type Value = i32;
}

impl GetProgramivParam for ProgramBinaryLength {
    type Value = i32;
}

impl GetProgramivParam for ComputeWorkGroupSize {
    type Value = [i32; 3];
}

impl GetProgramivParam for TransformFeedbackBufferMode {
    type Value = i32;
}

impl GetProgramivParam for TransformFeedbackVaryings {
    type Value = i32;
}

impl GetProgramivParam for TransformFeedbackVaryingMaxLength {
    type Value = i32;
}

impl GetProgramivParam for GeometryVerticesOut {
    type Value = i32;
}

impl GetProgramivParam for GeometryInputType {
    type Value = i32;
}

impl GetProgramivParam for GeometryOutputType {
    type Value = i32;
}
