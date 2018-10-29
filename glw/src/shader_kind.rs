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

pub const COMPUTE: ShaderKind = ShaderKind::Compute;
pub const FRAGMENT: ShaderKind = ShaderKind::Fragment;
pub const GEOMETRY: ShaderKind = ShaderKind::Geometry;
pub const VERTEX: ShaderKind = ShaderKind::Vertex;
pub const TESSELATION_CONTROL: ShaderKind = ShaderKind::TesselationControl;
pub const TESSELATION_EVALUATION: ShaderKind = ShaderKind::TesselationEvaluation;
