use crate::rendering::model::mesh::{Mesh, TextureInfo};
use crate::rendering::opengl::DrawingMode;
use nalgebra::{Vector2, Vector3};
use num_traits::FloatConst;

const X_SEGMENT: usize = 128;
const Y_SEGMENT: usize = 128;

type ModelData = (Vec<Vector3<f32>>, Vec<Vector2<f32>>, Vec<Vector3<f32>>);

fn positions_uv_normals(radius: f32) -> ModelData {
    let mut positions = vec![];
    let mut uv = vec![];
    let mut normals = vec![];

    for x in 0..X_SEGMENT + 1 {
        for y in 0..Y_SEGMENT + 1 {
            let x_segment = x as f32 / X_SEGMENT as f32;
            let y_segment = y as f32 / Y_SEGMENT as f32;
            let x_pos =
                radius * (x_segment * 2f32 * f32::PI()).cos() * (y_segment * f32::PI()).sin();
            let y_pos = radius * (y_segment * f32::PI()).cos();
            let z_pos =
                radius * (x_segment * 2f32 * f32::PI()).sin() * (y_segment * f32::PI()).sin();
            positions.push(Vector3::new(x_pos, y_pos, z_pos));
            uv.push(Vector2::new(x_segment as f32, y_segment as f32));
            normals.push(Vector3::new(x_pos, y_pos, z_pos));
        }
    }
    (positions, uv, normals)
}

fn indices() -> Vec<u32> {
    let mut indices = vec![];
    let mut odd_row = false;

    for y in 0..Y_SEGMENT {
        if !odd_row {
            for x in 0..X_SEGMENT + 1 {
                indices.push(y as u32 * (X_SEGMENT as u32 + 1) + x as u32);
                indices.push((y as u32 + 1) * (X_SEGMENT as u32 + 1) + x as u32);
            }
        } else {
            for x in (0..X_SEGMENT + 1).rev() {
                indices.push((y as u32 + 1) * (X_SEGMENT as u32 + 1) + x as u32);
                indices.push(y as u32 * (X_SEGMENT as u32 + 1) + x as u32);
            }
        }
        odd_row = !odd_row;
    }

    indices
}

pub fn sphere(radius: f32, textures: Vec<TextureInfo>) -> Mesh {
    let indices = indices();
    let (vertices, uv, normals) = positions_uv_normals(radius);

    Mesh {
        vertices,
        drawing_mode: DrawingMode::TriangleStrip,
        normals: Some(normals),
        indices: Some(indices),
        tangents: None,
        bitangents: None,
        textures: Some(textures),
        texture_coordinates: Some(uv),
        shininess: None,
    }
}
