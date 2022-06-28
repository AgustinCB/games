use nalgebra::{Vector2, Vector3};

use crate::rendering::model::mesh::{Mesh, TextureInfo};
use crate::rendering::opengl::DrawingMode;

const VERTICES: [Vector3<f32>; 36] = [
    Vector3::new(-1f32, -1f32, -1f32),
    Vector3::new(1f32, -1f32, -1f32),
    Vector3::new(1f32, 1f32, -1f32),
    Vector3::new(1f32, 1f32, -1f32),
    Vector3::new(-1f32, 1f32, -1f32),
    Vector3::new(-1f32, -1f32, -1f32),
    Vector3::new(-1f32, -1f32, 1f32),
    Vector3::new(1f32, -1f32, 1f32),
    Vector3::new(1f32, 1f32, 1f32),
    Vector3::new(1f32, 1f32, 1f32),
    Vector3::new(-1f32, 1f32, 1f32),
    Vector3::new(-1f32, -1f32, 1f32),
    Vector3::new(-1f32, 1f32, 1f32),
    Vector3::new(-1f32, 1f32, -1f32),
    Vector3::new(-1f32, -1f32, -1f32),
    Vector3::new(-1f32, -1f32, -1f32),
    Vector3::new(-1f32, -1f32, 1f32),
    Vector3::new(-1f32, 1f32, 1f32),
    Vector3::new(1f32, 1f32, 1f32),
    Vector3::new(1f32, 1f32, -1f32),
    Vector3::new(1f32, -1f32, -1f32),
    Vector3::new(1f32, -1f32, -1f32),
    Vector3::new(1f32, -1f32, 1f32),
    Vector3::new(1f32, 1f32, 1f32),
    Vector3::new(-1f32, -1f32, -1f32),
    Vector3::new(1f32, -1f32, -1f32),
    Vector3::new(1f32, -1f32, 1f32),
    Vector3::new(1f32, -1f32, 1f32),
    Vector3::new(-1f32, -1f32, 1f32),
    Vector3::new(-1f32, -1f32, -1f32),
    Vector3::new(-1f32, 1f32, -1f32),
    Vector3::new(1f32, 1f32, -1f32),
    Vector3::new(1f32, 1f32, 1f32),
    Vector3::new(1f32, 1f32, 1f32),
    Vector3::new(-1f32, 1f32, 1f32),
    Vector3::new(-1f32, 1f32, -1f32),
];

const NORMALS: [Vector3<f32>; 36] = [
    Vector3::new(0f32, 0f32, -1f32),
    Vector3::new(0f32, 0f32, -1f32),
    Vector3::new(0f32, 0f32, -1f32),
    Vector3::new(0f32, 0f32, -1f32),
    Vector3::new(0f32, 0f32, -1f32),
    Vector3::new(0f32, 0f32, -1f32),
    Vector3::new(0f32, 0f32, 1f32),
    Vector3::new(0f32, 0f32, 1f32),
    Vector3::new(0f32, 0f32, 1f32),
    Vector3::new(0f32, 0f32, 1f32),
    Vector3::new(0f32, 0f32, 1f32),
    Vector3::new(0f32, 0f32, 1f32),
    Vector3::new(-1f32, 0f32, 0f32),
    Vector3::new(-1f32, 0f32, 0f32),
    Vector3::new(-1f32, 0f32, 0f32),
    Vector3::new(-1f32, 0f32, 0f32),
    Vector3::new(-1f32, 0f32, 0f32),
    Vector3::new(-1f32, 0f32, 0f32),
    Vector3::new(1f32, 0f32, 0f32),
    Vector3::new(1f32, 0f32, 0f32),
    Vector3::new(1f32, 0f32, 0f32),
    Vector3::new(1f32, 0f32, 0f32),
    Vector3::new(1f32, 0f32, 0f32),
    Vector3::new(1f32, 0f32, 0f32),
    Vector3::new(0f32, -1f32, 0f32),
    Vector3::new(0f32, -1f32, 0f32),
    Vector3::new(0f32, -1f32, 0f32),
    Vector3::new(0f32, -1f32, 0f32),
    Vector3::new(0f32, -1f32, 0f32),
    Vector3::new(0f32, -1f32, 0f32),
    Vector3::new(0f32, 1f32, 0f32),
    Vector3::new(0f32, 1f32, 0f32),
    Vector3::new(0f32, 1f32, 0f32),
    Vector3::new(0f32, 1f32, 0f32),
    Vector3::new(0f32, 1f32, 0f32),
    Vector3::new(0f32, 1f32, 0f32),
];

const TEXTURE_COORDINATES: [Vector2<f32>; 36] = [
    Vector2::new(0f32, 0f32),
    Vector2::new(1f32, 0f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(0f32, 1f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(1f32, 0f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(0f32, 1f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(1f32, 0f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(0f32, 1f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(1f32, 0f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(0f32, 1f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(1f32, 0f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(0f32, 1f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(0f32, 0f32),
    Vector2::new(1f32, 0f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(1f32, 1f32),
    Vector2::new(0f32, 1f32),
    Vector2::new(0f32, 0f32),
];

pub fn cube(textures: Vec<TextureInfo>) -> Mesh {
    Mesh {
        bitangents: None,
        drawing_mode: DrawingMode::Triangles,
        indices: None,
        normals: Some(NORMALS.to_vec()),
        shininess: None,
        tangents: None,
        textures: Some(textures),
        texture_coordinates: Some(TEXTURE_COORDINATES.to_vec()),
        vertices: VERTICES.to_vec(),
    }
}

pub fn cuboid(hx: f32, hy: f32, hz: f32, textures: Vec<TextureInfo>) -> Mesh {
    let vertices = VERTICES
        .iter()
        .map(|v| Vector3::new(v.x * hx, v.y * hy, v.z * hz));
    Mesh {
        bitangents: None,
        drawing_mode: DrawingMode::Triangles,
        indices: None,
        normals: Some(NORMALS.to_vec()),
        shininess: None,
        tangents: None,
        textures: Some(textures),
        texture_coordinates: Some(TEXTURE_COORDINATES.to_vec()),
        vertices: vertices.collect(),
    }
}

pub fn rectangle(hx: f32, hy: f32, textures: Vec<TextureInfo>) -> Mesh {
    let vertices = VERTICES
        .iter()
        .take(6)
        .map(|v| Vector3::new(v.x * hx, v.y * hy, 0.0));

    Mesh {
        bitangents: None,
        drawing_mode: DrawingMode::Triangles,
        indices: None,
        normals: Some(NORMALS.iter().take(6).cloned().collect()),
        shininess: None,
        tangents: None,
        textures: Some(textures),
        texture_coordinates: Some(TEXTURE_COORDINATES.iter().take(6).cloned().collect()),
        vertices: vertices.collect(),
    }
}
