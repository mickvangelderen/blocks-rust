use param::*;

pub trait GetShaderivParam: Param {
    type Value;
}

impl GetShaderivParam for ShaderType {
    type Value = i32;
}

impl GetShaderivParam for DeleteStatus {
    type Value = i32;
}

impl GetShaderivParam for CompileStatus {
    type Value = i32;
}

impl GetShaderivParam for InfoLogLength {
    type Value = i32;
}

impl GetShaderivParam for ShaderSourceLength {
    type Value = i32;
}
