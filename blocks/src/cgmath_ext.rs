use cgmath;
use glw;

// NOTE: Maybe not the best name.
pub trait Matrix4Ext {
    fn as_matrix_ref(&self) -> &glw::ColMajorMatrix<[[f32; 4]; 4]>;
}

impl Matrix4Ext for cgmath::Matrix4<f32> {
    #[inline]
    fn as_matrix_ref(&self) -> &glw::ColMajorMatrix<[[f32; 4]; 4]> {
        AsRef::<[[f32; 4]; 4]>::as_ref(self).into()
    }
}
