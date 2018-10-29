use core::marker::PhantomData;
use gl;
use num::NonMinusOneI32;

pub struct UniformLocation<T>(NonMinusOneI32, PhantomData<*const T>);

impl<T> UniformLocation<T> {
    pub unsafe fn from_raw(loc: i32) -> Option<Self> {
        NonMinusOneI32::new(loc).map(|n| UniformLocation(n, PhantomData))
    }

    pub unsafe fn as_i32(&self) -> i32 {
        self.0.get()
    }
}

impl UniformLocation<i32> {
    /// Assumes the correct program is bound.
    #[inline]
    pub unsafe fn set(&self, value: i32) {
        gl::Uniform1i(self.as_i32(), value);
    }
}

impl UniformLocation<[i32; 2]> {
    /// Assumes the correct program is bound.
    #[inline]
    pub unsafe fn set(&self, value: [i32; 2]) {
        gl::Uniform2i(self.as_i32(), value[0], value[1]);
    }
}

impl UniformLocation<[i32; 3]> {
    /// Assumes the correct program is bound.
    #[inline]
    pub unsafe fn set(&self, value: [i32; 3]) {
        gl::Uniform3i(self.as_i32(), value[0], value[1], value[2]);
    }
}

impl UniformLocation<[i32; 4]> {
    /// Assumes the correct program is bound.
    #[inline]
    pub unsafe fn set(&self, value: [i32; 4]) {
        gl::Uniform4i(self.as_i32(), value[0], value[1], value[2], value[3]);
    }
}

impl UniformLocation<f32> {
    /// Assumes the correct program is bound.
    #[inline]
    pub unsafe fn set(&self, value: f32) {
        gl::Uniform1f(self.as_i32(), value);
    }
}

impl UniformLocation<[f32; 2]> {
    /// Assumes the correct program is bound.
    #[inline]
    pub unsafe fn set(&self, value: [f32; 2]) {
        gl::Uniform2f(self.as_i32(), value[0], value[1]);
    }
}

impl UniformLocation<[f32; 3]> {
    /// Assumes the correct program is bound.
    #[inline]
    pub unsafe fn set(&self, value: [f32; 3]) {
        gl::Uniform3f(self.as_i32(), value[0], value[1], value[2]);
    }
}

impl UniformLocation<[f32; 4]> {
    /// Assumes the correct program is bound.
    #[inline]
    pub unsafe fn set(&self, value: [f32; 4]) {
        gl::Uniform4f(self.as_i32(), value[0], value[1], value[2], value[3]);
    }
}

pub trait MatrixRef<T> {
    fn major_axis() -> MajorAxis;
    fn into_inner(self) -> T;
}

pub struct RowMatrixRef<T>(pub T);

impl<T> MatrixRef<T> for RowMatrixRef<T> {
    fn major_axis() -> MajorAxis {
        MajorAxis::Row
    }

    fn into_inner(self) -> T {
        self.0
    }
}

pub struct ColMatrixRef<T>(pub T);

impl<T> MatrixRef<T> for ColMatrixRef<T> {
    fn major_axis() -> MajorAxis {
        MajorAxis::Column
    }

    fn into_inner(self) -> T {
        self.0
    }
}

pub enum MajorAxis {
    Row,
    Column,
}

impl MajorAxis {
    fn should_transpose(&self) -> u8 {
        match *self {
            MajorAxis::Row => gl::FALSE,
            MajorAxis::Column => gl::TRUE,
        }
    }
}

impl UniformLocation<[f32; 16]> {
    /// Single 4x4 matrix.
    #[inline]
    pub unsafe fn set<'a, R: MatrixRef<&'a [f32; 16]>>(&self, value: R) {
        let value = value.into_inner();
        gl::UniformMatrix4fv(
            self.as_i32(),
            1,
            R::major_axis().should_transpose(),
            value.as_ptr(),
        );
    }
}

impl UniformLocation<[[f32; 4]; 4]> {
    /// Single 4x4 matrix.
    #[inline]
    pub unsafe fn set<'a, R: MatrixRef<&'a [[f32; 4]; 4]>>(&self, value: R) {
        let value = value.into_inner();
        gl::UniformMatrix4fv(
            self.as_i32(),
            1,
            R::major_axis().should_transpose(),
            value.as_ptr() as *const f32,
        );
    }
}

impl UniformLocation<&'static [[f32; 16]]> {
    /// Array of 4x4 matrices.
    #[inline]
    pub unsafe fn set<'a, R: MatrixRef<&'a [[f32; 16]]>>(&self, value: R) {
        let value = value.into_inner();
        gl::UniformMatrix4fv(
            self.as_i32(),
            value.len() as i32,
            R::major_axis().should_transpose(),
            value.as_ptr() as *const f32,
        );
    }
}

impl UniformLocation<&'static [[f32; 4]; 4]> {
    /// Array of 4x4 matrices.
    #[inline]
    pub unsafe fn set<'a, R: MatrixRef<&'a [[f32; 4]; 4]>>(&self, value: R) {
        let value = value.into_inner();
        gl::UniformMatrix4fv(
            self.as_i32(),
            value.len() as i32,
            R::major_axis().should_transpose(),
            value.as_ptr() as *const f32,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_option_self_equals_size_of_u32() {
        use std::mem::size_of;
        assert_eq!(
            size_of::<Option<UniformLocation<[f32; 4]>>>(),
            size_of::<u32>()
        );
    }
}
