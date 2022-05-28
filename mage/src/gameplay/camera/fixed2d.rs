use crate::gameplay::camera::Camera;
use nalgebra::{Matrix4, Orthographic3, Point2, Point3, UnitVector3, Vector3};

pub struct Fixed2dCameraBuilder {
    from: Point2<f32>,
    to: Point2<f32>,
    front: Option<UnitVector3<f32>>,
    world_up: Option<UnitVector3<f32>>,
}

impl Fixed2dCameraBuilder {
    pub fn new(from: Point2<f32>, to: Point2<f32>) -> Fixed2dCameraBuilder {
        Fixed2dCameraBuilder {
            from,
            to,
            front: None,
            world_up: None,
        }
    }

    pub fn front(&mut self, front: UnitVector3<f32>) {
        self.front = Some(front);
    }

    pub fn world_up(&mut self, world_up: UnitVector3<f32>) {
        self.world_up = Some(world_up);
    }

    pub fn build(&self) -> Fixed2dCamera {
        let projection =
            Orthographic3::new(self.from.x, self.to.x, self.from.y, self.to.y, 0.1, 1000f32)
                .to_homogeneous();
        let front = self.front.unwrap_or(-Vector3::z_axis());
        let world_up = self.world_up.unwrap_or_else(Vector3::y_axis);
        let right = UnitVector3::new_normalize(front.cross(&world_up));
        let up = UnitVector3::new_normalize(right.cross(&front));
        let position = Vector3::new(0.0, 0.0, 0.0);
        let look_at_matrix = Matrix4::look_at_rh(
            &Point3::from(position),
            &Point3::from(position + front.into_inner()),
            &up,
        );
        Fixed2dCamera {
            look_at_matrix,
            projection,
            position,
        }
    }
}

pub struct Fixed2dCamera {
    look_at_matrix: Matrix4<f32>,
    position: Vector3<f32>,
    projection: Matrix4<f32>,
}

impl Camera for Fixed2dCamera {
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
