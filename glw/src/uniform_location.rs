use super::gl;
use super::program::LinkedProgramName;
use core::marker::PhantomData;
use std::ffi::CStr;
use std::num::NonZeroU32;

pub struct UniformLocation<T>(NonZeroU32, PhantomData<*const T>);

impl<T> UniformLocation<T> {
    pub unsafe fn new(program_name: &LinkedProgramName, identifier: &CStr) -> Option<Self> {
        let loc: i32 = gl::GetUniformLocation(program_name.as_u32(), identifier.as_ptr());
        assert!(loc >= -1);
        NonZeroU32::new((loc + 1) as u32).map(|n| UniformLocation(n, PhantomData))
    }

    pub unsafe fn as_i32(&self) -> i32 {
        self.0.get() as i32 - 1
    }
}

impl UniformLocation<i32> {
    /// Assumes the correct program is bound.
    pub unsafe fn set(&self, value: i32) {
        gl::Uniform1i(self.as_i32(), value);
    }
}

impl UniformLocation<[i32; 2]> {
    /// Assumes the correct program is bound.
    pub unsafe fn set(&self, value: [i32; 2]) {
        gl::Uniform2i(self.as_i32(), value[0], value[1]);
    }
}

impl UniformLocation<[i32; 3]> {
    /// Assumes the correct program is bound.
    pub unsafe fn set(&self, value: [i32; 3]) {
        gl::Uniform3i(self.as_i32(), value[0], value[1], value[2]);
    }
}

impl UniformLocation<[i32; 4]> {
    /// Assumes the correct program is bound.
    pub unsafe fn set(&self, value: [i32; 4]) {
        gl::Uniform4i(self.as_i32(), value[0], value[1], value[2], value[3]);
    }
}

pub trait MatrixRef<'a, T: 'a> {
    fn major_axis() -> MajorAxis;
    fn as_ref(&self) -> &'a T;
}

pub struct RowMatrixRef<'a, T: 'a>(&'a T);

impl<'a, T: 'a> RowMatrixRef<'a, T> {
    pub fn from(value: &'a T) -> Self {
        RowMatrixRef(value)
    }
}

impl<'a, T: 'a> MatrixRef<'a, T> for RowMatrixRef<'a, T> {
    fn major_axis() -> MajorAxis {
        MajorAxis::Row
    }

    fn as_ref(&self) -> &'a T {
        self.0
    }
}

pub struct ColMatrixRef<'a, T: 'a>(&'a T);

impl<'a, T: 'a> ColMatrixRef<'a, T> {
    pub fn from(value: &'a T) -> Self {
        ColMatrixRef(value)
    }
}

impl<'a, T: 'a> MatrixRef<'a, T> for ColMatrixRef<'a, T> {
    fn major_axis() -> MajorAxis {
        MajorAxis::Column
    }

    fn as_ref(&self) -> &'a T {
        self.0
    }
}

#[repr(u8)]
pub enum MajorAxis {
    Row = gl::TRUE,
    Column = gl::FALSE,
}

impl UniformLocation<[f32; 16]> {
    pub unsafe fn set<'a, R: MatrixRef<'a, [f32; 16]>>(&self, value: R) {
        gl::UniformMatrix4fv(
            self.as_i32(),  // location
            1,
            R::major_axis() as u8,  // row major
            value.as_ref().as_ptr(), // data
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
