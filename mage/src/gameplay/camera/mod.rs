use nalgebra::{Matrix4, Perspective3, Point3, UnitVector3, Vector3};

pub trait Camera {
    fn projection(&self) -> Matrix4<f32>;

    fn look_at_matrix(&self) -> Matrix4<f32>;

    fn position(&self) -> Vector3<f32>;
}

pub struct FixedCamera {
    look_at_matrix: Matrix4<f32>,
    position: Vector3<f32>,
    projection: Matrix4<f32>,
}

impl FixedCamera {
    pub fn new(width: u32, height: u32, position: Vector3<f32>) -> FixedCamera {
        FixedCamera::new_with_world_reference(
            width, height, 45f32, 0.1, 100f32, position, UnitVector3::new_normalize(Vector3::new(0f32, 0f32, -1f32)), Vector3::y_axis(),
        )
    }

    pub fn new_with_world_reference(
        width: u32,
        height: u32,
        fov: f32,
        close_plane: f32,
        far_plane: f32,
        position: Vector3<f32>,
        front: UnitVector3<f32>,
        world_up: UnitVector3<f32>,
    ) -> FixedCamera {
        let projection = Perspective3::new(
            width as f32 / height as f32,
            fov.to_radians(),
            close_plane,
            far_plane,
        ).to_homogeneous();
        let right = UnitVector3::new_normalize(front.cross(&world_up));
        let up = UnitVector3::new_normalize(right.cross(&front));
        let look_at_matrix = Matrix4::look_at_rh(
            &Point3::from(position),
            &Point3::from(position + front.into_inner()),
            &up,
        );
        FixedCamera {
            look_at_matrix,
            position,
            projection,
        }
    }
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
