use gl;
use super::*;

macro_rules! impl_enum_struct {
    ($T:ident, $V:ident) => {
        #[derive(Clone, Copy, Eq, PartialEq, Debug)]
        pub struct $T(());

        impl $T {
            #[inline]
            const fn as_u32() -> u32 {
                gl::$V
            }
        }

        pub const $V: $T = $T(());
    };
}

impl_enum_struct!(ComputeShader, COMPUTE_SHADER);
impl_enum_struct!(FragmentShader, FRAGMENT_SHADER);
impl_enum_struct!(GeometryShader, GEOMETRY_SHADER);
impl_enum_struct!(VertexShader, VERTEX_SHADER);
impl_enum_struct!(TessControlShader, TESS_CONTROL_SHADER);
impl_enum_struct!(TessEvaluationShader, TESS_EVALUATION_SHADER);

macro_rules! impl_shader_kinds {
    ($(($E:ident, $T:ident, $S:ident)),+ $(,)*) => {
        pub trait StaticShaderKind {
            type ShaderName: StaticShaderName;

            fn as_u32() -> u32;
        }

        $(
            impl StaticShaderKind for $T {
                type ShaderName = $S;

                fn as_u32() -> u32 {
                    Self::as_u32()
                }
            }
        )+

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(u32)]
        pub enum DynamicShaderKind {
            $(
                $E = $T::as_u32(),
            )+
        }

        impl DynamicShaderKind {
            #[inline]
            pub fn as_u32(&self) -> u32 {
                *self as u32
            }
        }

        pub trait ShaderKind {
            fn as_u32(&self) -> u32;
        }

        impl<T: StaticShaderKind> ShaderKind for T {
            fn as_u32(&self) -> u32 {
                T::as_u32()
            }
        }

        impl ShaderKind for DynamicShaderKind {
            fn as_u32(&self) -> u32 {
                self.as_u32()
            }
        }
    };
}

impl_shader_kinds! (
    (Compute, ComputeShader, ComputeShaderName),
    (Fragment, FragmentShader, FragmentShaderName),
    (Geometry, GeometryShader, GeometryShaderName),
    (Vertex, VertexShader, VertexShaderName),
    (TessControl, TessControlShader, TessControlShaderName),
    (TessEvaluation, TessEvaluationShader, TessEvaluationShaderName),
);
