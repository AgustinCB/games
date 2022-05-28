use crate::gameplay::camera::Camera;
use nalgebra::{Matrix4, Perspective3, Point3, UnitVector3, Vector3};

pub struct FixedCameraBuilder {
    width: u32,
    height: u32,
    fov: Option<f32>,
    close_plane: Option<f32>,
    far_plane: Option<f32>,
    position: Vector3<f32>,
    front: Option<UnitVector3<f32>>,
    world_up: Option<UnitVector3<f32>>,
}

impl FixedCameraBuilder {
    pub fn new(width: u32, height: u32, position: Vector3<f32>) -> FixedCameraBuilder {
        FixedCameraBuilder {
            height,
            position,
            width,
            fov: None,
            close_plane: None,
            far_plane: None,
            front: None,
            world_up: None,
        }
    }

    pub fn fov(&mut self, fov: f32) {
        self.fov = Some(fov);
    }

    pub fn close_plane(&mut self, close_plane: f32) {
        self.close_plane = Some(close_plane);
    }

    pub fn far_plane(&mut self, far_plane: f32) {
        self.far_plane = Some(far_plane);
    }

    pub fn front(&mut self, front: UnitVector3<f32>) {
        self.front = Some(front);
    }

    pub fn world_up(&mut self, world_up: UnitVector3<f32>) {
        self.world_up = Some(world_up);
    }

    pub fn build(&self) -> FixedCamera {
        let projection = Perspective3::new(
            self.width as f32 / self.height as f32,
            self.fov.unwrap_or(45f32).to_radians(),
            self.close_plane.unwrap_or(0.0001),
            self.far_plane.unwrap_or(100f32),
        )
        .to_homogeneous();
        //let aspect = self.width as f32 / self.height as f32;
        //let projection = Scale3::new(0.12, 0.16, 0.1).to_homogeneous();
        let front = self.front.unwrap_or(-Vector3::z_axis());
        let world_up = self.world_up.unwrap_or_else(Vector3::y_axis);
        let right = UnitVector3::new_normalize(front.cross(&world_up));
        let up = UnitVector3::new_normalize(right.cross(&front));
        let look_at_matrix = Matrix4::look_at_rh(
            &Point3::from(self.position),
            &Point3::from(self.position + front.into_inner()),
            &up,
        );
        FixedCamera {
            look_at_matrix,
            projection,
            position: self.position,
        }
    }
}

pub struct FixedCamera {
    look_at_matrix: Matrix4<f32>,
    position: Vector3<f32>,
    projection: Matrix4<f32>,
}

impl Camera for FixedCamera {
    fn projection(&self) -> Matrix4<f32> {
        self.projection
    }

    fn look_at_matrix(&self) -> Matrix4<f32> {
        self.look_at_matrix
    }

    fn position(&self) -> Vector3<f32> {
        self.position
    }
}
