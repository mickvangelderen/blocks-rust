use super::shader_kind::*;

impl_name!(ShaderName);

pub trait StaticShaderName: Sized {
    type ShaderKind: StaticShaderKind;

    fn as_u32(&self) -> u32;
    unsafe fn from_raw(name: u32) -> Option<Self>;
}

macro_rules! impl_shader_kind {
    ($K:path, $T:ident,) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $T(ShaderName);

        impl $T {
            #[inline]
            pub fn as_u32(&self) -> u32 {
                self.0.as_u32()
            }

            #[inline]
            unsafe fn from_raw(name: u32) -> Option<Self> {
                ShaderName::from_raw(name).map($T)
            }
        }

        impl StaticShaderName for $T {
            type ShaderKind = $K;

            #[inline]
            fn as_u32(&self) -> u32 {
                self.as_u32()
            }

            #[inline]
            unsafe fn from_raw(name: u32) -> Option<Self> {
                Self::from_raw(name)
            }
        }

        // Temporarily discard the kind.
        impl AsRef<ShaderName> for $T {
            #[inline]
            fn as_ref(&self) -> &ShaderName {
                &self.0
            }
        }
    };
}

impl_shader_kind!(
    ComputeShader,
    ComputeShaderName,
);
impl_shader_kind!(
    FragmentShader,
    FragmentShaderName,
);
impl_shader_kind!(
    GeometryShader,
    GeometryShaderName,
);
impl_shader_kind!(
    VertexShader,
    VertexShaderName,
);
impl_shader_kind!(
    TessControlShader,
    TessControlShaderName,
);
impl_shader_kind!(
    TessEvaluationShader,
    TessEvaluationShaderName,
);
