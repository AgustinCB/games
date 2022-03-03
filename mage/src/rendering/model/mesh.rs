use std::sync::Arc;

use itertools::multizip;
use nalgebra::{ArrayStorage, Matrix, U1, Vector2, Vector3};
use russimp::texture::TextureType;

use crate::rendering::opengl::DrawingMode;
use crate::rendering::opengl::program::Program;
use crate::rendering::opengl::texture::Texture;

fn flattened_vectors(vectors: &[Vector3<f32>]) -> Vec<f32> {
    vectors.iter()
        .map(|v| v.data.as_slice())
        .flatten()
        .cloned()
        .collect::<Vec<f32>>()
}

#[derive(Clone, Debug)]
pub enum TextureSource {
    File(String),
    Color(Vector3<f32>),
}

#[derive(Clone, Debug)]
pub struct TextureInfo {
    pub id: usize,
    pub texture_type: TextureType,
    pub source: TextureSource,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub bitangents: Option<Vec<Vector3<f32>>>,
    pub drawing_mode: DrawingMode,
    pub indices: Option<Vec<u32>>,
    pub normals: Option<Vec<Vector3<f32>>>,
    pub shininess: Option<f32>,
    pub tangents: Option<Vec<Vector3<f32>>>,
    pub textures: Option<Vec<TextureInfo>>,
    pub texture_coordinates: Option<Vec<Vector2<f32>>>,
    pub vertices: Vec<Vector3<f32>>,
}

impl Mesh {
    pub fn len(&self) -> usize {
        if let Some(indices) = &self.indices {
            indices.len()
        } else {
            self.vertices.len()
        }
    }

    pub fn set_program(&self, program: &Program, textures: &[Arc<Texture>]) {
        let mut diffuse_index = 0;
        let mut specular_index = 0;
        let mut normal_index = 0;
        let mut height_index = 0;
        let mut metallic_index = 0;
        let mut roughness_index = 0;
        let mut ao_index = 0;
        if let Some(infos) = &self.textures {
            for (texture, info) in textures.iter().zip(infos.iter()) {
                texture.bind(gl::TEXTURE0 + info.id as u32);
                let (texture_type, texture_index) = if info.texture_type == TextureType::Diffuse {
                    let index = diffuse_index;
                    diffuse_index += 1;
                    ("diffuse", index)
                } else if info.texture_type == TextureType::Specular {
                    let index = specular_index;
                    specular_index += 1;
                    ("specular", index)
                } else if info.texture_type == TextureType::Normals {
                    let index = normal_index;
                    normal_index += 1;
                    ("normal", index)
                } else if info.texture_type == TextureType::Height {
                    let index = height_index;
                    height_index += 1;
                    ("height", index)
                } else if info.texture_type == TextureType::Metalness {
                    let index = metallic_index;
                    metallic_index += 1;
                    ("metalness", index)
                } else if info.texture_type == TextureType::Roughness {
                    let index = roughness_index;
                    roughness_index += 1;
                    ("roughness", index)
                } else if info.texture_type == TextureType::AmbientOcclusion {
                    let index = ao_index;
                    ao_index += 1;
                    ("ao", index)
                } else {
                    panic!("Can't happen");
                };
                program.set_uniform_i1(&format!("material.{}{}", texture_type, texture_index), info.id as i32);
            }
        }
        program.set_uniform_i1("material.n_diffuse", diffuse_index);
        program.set_uniform_i1("material.n_specular", specular_index);
        program.set_uniform_i1("material.n_height", height_index);
        let shininess = self.shininess.clone().unwrap_or(64f32);
        program.set_uniform_f1("material.shininess", shininess);
    }

    pub fn flattened_data(&self) -> Vec<f32> {
        match (&self.normals, &self.texture_coordinates, &self.tangents, &self.bitangents) {
            (None, None, None, None) => {
                self.get_flattened_vertices()
            }
            (Some(normals), None, None, None) => {
                self.flatten_with_vertices(normals)
            }
            (None, Some(texture_coordinates), None, None) => {
                self.flatten_with_vertices(texture_coordinates)
            }
            (None, None, Some(tangents), None) => {
                self.flatten_with_vertices(tangents)
            }
            (None, None, None, Some(bitangents)) => {
                self.flatten_with_vertices(bitangents)
            }
            (Some(normals), Some(texture_coordinates), None, None) => {
                self.flatten_two_with_vertices(normals, texture_coordinates)
            }
            (Some(normals), None, Some(tangents), None) => {
                self.flatten_two_with_vertices(normals, tangents)
            }
            (Some(normals), None, None, Some(bitangents)) => {
                self.flatten_two_with_vertices(normals, bitangents)
            }
            (None, Some(texture_coordinates), Some(tangents), None) => {
                self.flatten_two_with_vertices(texture_coordinates, tangents)
            }
            (None, Some(texture_coordinates), None, Some(bitangents)) => {
                self.flatten_two_with_vertices(texture_coordinates, bitangents)
            }
            (None, None, Some(tangents), Some(bitangents)) => {
                self.flatten_two_with_vertices(tangents, bitangents)
            }
            (Some(normals), Some(texture_coordinates), Some(tangents), None) => {
                self.flatten_three_with_vertices(normals, texture_coordinates, tangents)
            }
            (Some(normals), Some(texture_coordinates), None, Some(bitangents)) => {
                self.flatten_three_with_vertices(normals, texture_coordinates, bitangents)
            }
            (Some(normals), None, Some(tangents), Some(bitangents)) => {
                self.flatten_three_with_vertices(normals, tangents, bitangents)
            }
            (None, Some(texture_coordinates), Some(tangents), Some(bitangents)) => {
                self.flatten_three_with_vertices(texture_coordinates, tangents, bitangents)
            }
            (Some(normals), Some(texture_coordinates), Some(tangents), Some(bitangents)) => {
                self.flatten_four_with_vertices(normals, texture_coordinates, tangents, bitangents)
            }
        }
    }

    pub fn vertex_info_size(&self) -> usize {
        let normals_size = if self.normals.is_some() {
            3
        } else {
            0
        };
        let textures_size = if self.texture_coordinates.is_some() {
            2
        } else {
            0
        };
        let tangents_size = if self.tangents.is_some() {
            3
        } else {
            0
        };
        let bitangents_size = if self.bitangents.is_some() {
            3
        } else {
            0
        };
        3 + normals_size + textures_size + tangents_size + bitangents_size
    }

    fn get_flattened_vertices(&self) -> Vec<f32> {
        flattened_vectors(&self.vertices)
    }

    fn flatten_two_with_vertices<U, V, const C: usize, const C1: usize>(
        &self,
        first: &[Matrix<f32, U, U1, ArrayStorage<f32, C, 1>>],
        second: &[Matrix<f32, V, U1, ArrayStorage<f32, C1, 1>>],
    ) -> Vec<f32> {
        multizip((self.vertices.iter(), first, second))
            .map(|(v, n, t)| {
                let mut d = v.data.as_slice().to_vec();
                d.extend(n.data.as_slice());
                d.extend(t.data.as_slice());
                d
            })
            .flatten()
            .collect::<Vec<f32>>()
    }

    fn flatten_three_with_vertices<U, V, W, const C: usize, const C1: usize, const C2: usize>(
        &self,
        first: &[Matrix<f32, U, U1, ArrayStorage<f32, C, 1>>],
        second: &[Matrix<f32, V, U1, ArrayStorage<f32, C1, 1>>],
        third: &[Matrix<f32, W, U1, ArrayStorage<f32, C2, 1>>],
    ) -> Vec<f32> {
        multizip((self.vertices.iter(), first, second, third))
            .map(|(v, f, s, t)| {
                let mut d = v.data.as_slice().to_vec();
                d.extend(f.data.as_slice());
                d.extend(s.data.as_slice());
                d.extend(t.data.as_slice());
                d
            })
            .flatten()
            .collect::<Vec<f32>>()
    }

    fn flatten_four_with_vertices<U, V, W, Y, const C: usize, const C1: usize, const C2: usize, const C3: usize>(
        &self,
        first: &[Matrix<f32, U, U1, ArrayStorage<f32, C, 1>>],
        second: &[Matrix<f32, V, U1, ArrayStorage<f32, C1, 1>>],
        third: &[Matrix<f32, W, U1, ArrayStorage<f32, C2, 1>>],
        forth: &[Matrix<f32, Y, U1, ArrayStorage<f32, C3, 1>>],
    ) -> Vec<f32> {
        multizip((self.vertices.iter(), first, second, third, forth))
            .map(|(v, f, s, t, ff)| {
                let mut d = v.data.as_slice().to_vec();
                d.extend(f.data.as_slice());
                d.extend(s.data.as_slice());
                d.extend(t.data.as_slice());
                d.extend(ff.data.as_slice());
                d
            })
            .flatten()
            .collect::<Vec<f32>>()
    }

    fn flatten_with_vertices<D, const S: usize>(
        &self,
        other: &[Matrix<f32, D, U1, ArrayStorage<f32, S, 1>>],
    ) -> Vec<f32> {
        self.vertices.iter()
            .zip(other)
            .map(|(v, n)| {
                let mut d = v.data.as_slice().to_vec();
                d.extend(n.data.as_slice());
                d
            })
            .flatten()
            .collect::<Vec<f32>>()
    }
}