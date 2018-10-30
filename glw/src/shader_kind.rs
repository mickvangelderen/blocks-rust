use gl;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum ShaderKind {
    Compute = gl::COMPUTE_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
    Vertex = gl::VERTEX_SHADER,
    TesselationControl = gl::TESS_CONTROL_SHADER,
    TesselationEvaluation = gl::TESS_EVALUATION_SHADER,
}

impl ShaderKind {
    #[inline]
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

pub const COMPUTE_SHADER: ShaderKind = ShaderKind::Compute;
pub const FRAGMENT_SHADER: ShaderKind = ShaderKind::Fragment;
pub const GEOMETRY_SHADER: ShaderKind = ShaderKind::Geometry;
pub const VERTEX_SHADER: ShaderKind = ShaderKind::Vertex;
pub const TESS_CONTROL_SHADER: ShaderKind = ShaderKind::TesselationControl;
pub const TESS_EVALUATION_SHADER: ShaderKind = ShaderKind::TesselationEvaluation;
