use cgmath;
use glw;

pub trait Matrix4Ext<T> {
    fn as_matrix_ref(&self) -> glw::RowMatrixRef<&[[T; 4]; 4]>;
}

impl<T> Matrix4Ext<T> for cgmath::Matrix4<T> {
    #[inline]
    fn as_matrix_ref(&self) -> glw::RowMatrixRef<&[[T; 4]; 4]> {
        glw::RowMatrixRef(self.as_ref())
    }
}
