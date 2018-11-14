use gl;

#[repr(u8)]
pub enum MajorAxis {
    Col = gl::FALSE,
    Row = gl::TRUE,
}

pub trait Major<T>: AsRef<T> {
    fn major_axis() -> MajorAxis;
}

#[repr(transparent)]
pub struct RowMajorMatrix<T>(T);

#[repr(transparent)]
pub struct ColMajorMatrix<T>(T);

macro_rules! impl_matrices {
    ($(($M:ident, $Flat:ty, $Row:ty, $Col:ty)),+ $(,)*) => {
        $(
            pub trait $M: AsRef<$Flat> {
                fn major_axis() -> MajorAxis;
            }

            impl<'a> From<&'a $Row> for &'a RowMajorMatrix<$Row> {
                fn from(r: &$Row) -> Self {
                    unsafe {
                        &*(r as *const $Row as *const RowMajorMatrix<$Row>)
                    }
                }
            }

            impl AsRef<$Flat> for RowMajorMatrix<$Row> {
                fn as_ref(&self) -> &$Flat {
                    unsafe {
                        ::std::mem::transmute(&self.0)
                    }
                }
            }

            impl $M for RowMajorMatrix<$Row> {
                #[inline]
                fn major_axis() -> MajorAxis {
                    MajorAxis::Row
                }
            }

            impl<'a> From<&'a $Col> for &'a ColMajorMatrix<$Col> {
                fn from(r: &$Col) -> Self {
                    unsafe {
                        &*(r as *const $Col as *const ColMajorMatrix<$Col>)
                    }
                }
            }

            impl AsRef<$Flat> for ColMajorMatrix<$Col> {
                fn as_ref(&self) -> &$Flat {
                    unsafe {
                        ::std::mem::transmute(&self.0)
                    }
                }
            }

            impl $M for ColMajorMatrix<$Col> {
                #[inline]
                fn major_axis() -> MajorAxis {
                    MajorAxis::Col
                }
            }
        )+
    }
}

impl_matrices!(
    (Matrix2f, [f32;  4], [[f32; 2]; 2], [[f32; 2]; 2]),
    (Matrix3f, [f32;  9], [[f32; 3]; 3], [[f32; 3]; 3]),
    (Matrix4f, [f32; 16], [[f32; 4]; 4], [[f32; 4]; 4]),
    (Matrix2x3f, [f32; 6], [[f32; 2]; 3], [[f32; 3]; 2]),
    (Matrix3x2f, [f32; 6], [[f32; 3]; 2], [[f32; 2]; 3]),
    (Matrix2x4f, [f32; 8], [[f32; 2]; 4], [[f32; 4]; 2]),
    (Matrix4x2f, [f32; 8], [[f32; 4]; 2], [[f32; 2]; 4]),
    (Matrix3x4f, [f32; 12], [[f32; 3]; 4], [[f32; 4]; 3]),
    (Matrix4x3f, [f32; 12], [[f32; 4]; 3], [[f32; 3]; 4]),
);

#[test]
fn test() {
    let m: [[f32; 2]; 2] = [[1.1, 1.2], [2.1, 2.2]];
    let r: &RowMajorMatrix<[[f32; 2]; 2]> = (&m).into();
}
