use nalgebra::{Vector2, Vector3};

use crate::rendering::model::mesh::{Mesh, TextureInfo};
use crate::rendering::opengl::DrawingMode;

const VERTICES: [Vector3<f32>; 4] = [
    Vector3::new(1f32, 1f32, 0f32),
    Vector3::new(1f32, -1f32, 0f32),
    Vector3::new(-1f32, -1f32, 0f32),
    Vector3::new(-1f32, 1f32, 0f32),
];

const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

const NORMALS: [Vector3<f32>; 4] = [
    Vector3::new(0f32, 0f32, 1f32); 4
];

const TEXTURE_COORDINATES: [Vector2<f32>; 4] = [
    Vector2::new(1f32, 1f32),
    Vector2::new(1f32, 0f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(0f32, 1f32),
];

pub fn quad(textures: Vec<TextureInfo>) -> Mesh {
    Mesh {
        bitangents: None,
        drawing_mode: DrawingMode::Triangles,
        indices: Some(INDICES.to_vec()),
        normals: Some(NORMALS.to_vec()),
        shininess: None,
        tangents: None,
        textures: Some(textures),
        texture_coordinates: Some(TEXTURE_COORDINATES.to_vec()),
        vertices: VERTICES.to_vec(),
    }
}
