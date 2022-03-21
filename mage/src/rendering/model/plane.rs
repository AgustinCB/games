use nalgebra::{Vector2, Vector3};

use crate::rendering::model::mesh::{Mesh, TextureInfo};
use crate::rendering::opengl::DrawingMode;

const HORIZONTAL_VERTICES: [Vector3<f32>; 4] = [
    Vector3::new(1f32, 0f32, 1f32),
    Vector3::new(1f32, 0f32, -1f32),
    Vector3::new(-1f32, 0f32, -1f32),
    Vector3::new(-1f32, 0f32, 1f32),
];
const VERTICAL_VERTICES: [Vector3<f32>; 4] = [
    Vector3::new(1f32, 1f32, 0f32),
    Vector3::new(1f32, -1f32, 0f32),
    Vector3::new(-1f32, -1f32, 0f32),
    Vector3::new(-1f32, 1f32, 0f32),
];

const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

const HORIZONTAL_NORMALS: [Vector3<f32>; 4] = [Vector3::new(0f32, 1f32, 0f32); 4];
const VERTICAL_NORMALS: [Vector3<f32>; 4] = [Vector3::new(0f32, 0f32, 1f32); 4];

const TEXTURE_COORDINATES: [Vector2<f32>; 4] = [
    Vector2::new(1f32, 1f32),
    Vector2::new(1f32, 0f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(0f32, 1f32),
];

pub fn vertical_plane(textures: Vec<TextureInfo>) -> Mesh {
    plane(textures, &VERTICAL_NORMALS, &VERTICAL_VERTICES)
}

pub fn horizontal_plan(textures: Vec<TextureInfo>) -> Mesh {
    plane(textures, &HORIZONTAL_NORMALS, &HORIZONTAL_VERTICES)
}

fn plane(textures: Vec<TextureInfo>, normals: &[Vector3<f32>], vertices: &[Vector3<f32>]) -> Mesh {
    Mesh {
        bitangents: None,
        drawing_mode: DrawingMode::Triangles,
        indices: Some(INDICES.to_vec()),
        normals: Some(normals.to_vec()),
        shininess: None,
        tangents: None,
        textures: Some(textures),
        texture_coordinates: Some(TEXTURE_COORDINATES.to_vec()),
        vertices: vertices.to_vec(),
    }
}
