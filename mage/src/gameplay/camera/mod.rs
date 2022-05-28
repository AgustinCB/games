use nalgebra::{Matrix4, Vector3};

pub trait Camera {
    fn projection(&self) -> Matrix4<f32>;

    fn look_at_matrix(&self) -> Matrix4<f32>;

    fn position(&self) -> Vector3<f32>;
}

mod fixed;
pub use fixed::{FixedCamera, FixedCameraBuilder};

mod fixed2d;
pub use fixed2d::{Fixed2dCamera, Fixed2dCameraBuilder};
