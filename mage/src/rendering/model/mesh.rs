use std::collections::HashMap;
use std::sync::Arc;

use itertools::multizip;
use nalgebra::{ArrayStorage, Matrix, Vector2, Vector3, Vector4, U1};
pub use russimp::texture::TextureType;

use crate::rendering::opengl::buffer::{Buffer, BufferType, BufferUsage};
use crate::rendering::opengl::program::Program;
use crate::rendering::opengl::texture::{Texture, TextureParameter, TextureParameterValue};
use crate::rendering::opengl::vertex_array::{DataType, VertexArray};
use crate::rendering::opengl::{draw_arrays, draw_elements, DrawingMode, OpenGlType};
use crate::resources::texture::TextureLoader;
use crate::MageError;

fn flattened_vectors(vectors: &[Vector3<f32>]) -> Vec<f32> {
    vectors
        .iter()
        .flat_map(|v| v.data.as_slice())
        .cloned()
        .collect::<Vec<f32>>()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TextureSource {
    File(String),
    Color(Vector3<u8>),
    ColoredFile(String, Vector4<u8>),
}

#[derive(Clone, Debug)]
pub struct TextureInfo {
    pub id: usize,
    pub texture_type: TextureType,
    pub source: TextureSource,
    pub parameters: HashMap<TextureParameter, TextureParameterValue>,
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
    pub fn len_vertices(&self) -> usize {
        if let Some(indices) = &self.indices {
            indices.len()
        } else {
            self.vertices.len()
        }
    }

    pub fn to_rendering_mesh(
        &self,
        loader: Arc<TextureLoader>,
    ) -> Result<RenderingMesh, MageError> {
        let size = self.size() as u32;
        let mut attribute = 0;
        let mut start = 0;
        let vertex_array = Arc::new(VertexArray::new());
        let array_buffer = Arc::new(Buffer::new(BufferType::Array));
        vertex_array.bind();
        array_buffer.bind();
        array_buffer.set_data(&self.flattened_data(), BufferUsage::StaticDraw);
        let element_buffer = if let Some(indices) = &self.indices {
            let element_buffer = Buffer::new(BufferType::ElementArray);
            element_buffer.bind();
            element_buffer.set_data(indices, BufferUsage::StaticDraw);
            Some(Arc::new(element_buffer))
        } else {
            None
        };
        VertexArray::set_vertex_attrib_with_padding::<f32>(
            DataType::Float,
            attribute,
            size,
            3,
            start,
            false,
        );
        start += 3;
        attribute += 1;
        if self.normals.is_some() {
            VertexArray::set_vertex_attrib_with_padding::<f32>(
                DataType::Float,
                attribute,
                size,
                3,
                start,
                false,
            );
            start += 3;
            attribute += 1;
        }
        if self.texture_coordinates.is_some() {
            VertexArray::set_vertex_attrib_with_padding::<f32>(
                DataType::Float,
                attribute,
                size,
                2,
                start,
                false,
            );
            start += 2;
            attribute += 1;
        }
        if self.tangents.is_some() {
            VertexArray::set_vertex_attrib_with_padding::<f32>(
                DataType::Float,
                attribute,
                size,
                3,
                start,
                false,
            );
            start += 3;
            attribute += 1;
        }
        if self.bitangents.is_some() {
            VertexArray::set_vertex_attrib_with_padding::<f32>(
                DataType::Float,
                attribute,
                size,
                3,
                start,
                false,
            );
        }
        let mut textures = vec![];

        if let Some(texture_infos) = &self.textures {
            for texture_info in texture_infos.iter() {
                textures.push(loader.load_texture_2d(texture_info)?);
            }
        }
        VertexArray::unbind();
        Ok(RenderingMesh {
            array_buffer,
            element_buffer,
            textures,
            vertex_array,
            mesh: self.clone(),
        })
    }

    pub fn size(&self) -> usize {
        3 + self.normals.as_ref().map(|_| 3).unwrap_or(0)
            + self.texture_coordinates.as_ref().map(|_| 2).unwrap_or(0)
            + self.tangents.as_ref().map(|_| 3).unwrap_or(0)
            + self.bitangents.as_ref().map(|_| 3).unwrap_or(0)
    }

    pub fn flattened_data(&self) -> Vec<f32> {
        match (
            &self.normals,
            &self.texture_coordinates,
            &self.tangents,
            &self.bitangents,
        ) {
            (None, None, None, None) => self.get_flattened_vertices(),
            (Some(normals), None, None, None) => self.flatten_with_vertices(normals),
            (None, Some(texture_coordinates), None, None) => {
                self.flatten_with_vertices(texture_coordinates)
            }
            (None, None, Some(tangents), None) => self.flatten_with_vertices(tangents),
            (None, None, None, Some(bitangents)) => self.flatten_with_vertices(bitangents),
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
        let normals_size = if self.normals.is_some() { 3 } else { 0 };
        let textures_size = if self.texture_coordinates.is_some() {
            2
        } else {
            0
        };
        let tangents_size = if self.tangents.is_some() { 3 } else { 0 };
        let bitangents_size = if self.bitangents.is_some() { 3 } else { 0 };
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
            .flat_map(|(v, n, t)| {
                let mut d = v.data.as_slice().to_vec();
                d.extend(n.data.as_slice());
                d.extend(t.data.as_slice());
                d
            })
            .collect::<Vec<f32>>()
    }

    fn flatten_three_with_vertices<U, V, W, const C: usize, const C1: usize, const C2: usize>(
        &self,
        first: &[Matrix<f32, U, U1, ArrayStorage<f32, C, 1>>],
        second: &[Matrix<f32, V, U1, ArrayStorage<f32, C1, 1>>],
        third: &[Matrix<f32, W, U1, ArrayStorage<f32, C2, 1>>],
    ) -> Vec<f32> {
        multizip((self.vertices.iter(), first, second, third))
            .flat_map(|(v, f, s, t)| {
                let mut d = v.data.as_slice().to_vec();
                d.extend(f.data.as_slice());
                d.extend(s.data.as_slice());
                d.extend(t.data.as_slice());
                d
            })
            .collect::<Vec<f32>>()
    }

    fn flatten_four_with_vertices<
        U,
        V,
        W,
        Y,
        const C: usize,
        const C1: usize,
        const C2: usize,
        const C3: usize,
    >(
        &self,
        first: &[Matrix<f32, U, U1, ArrayStorage<f32, C, 1>>],
        second: &[Matrix<f32, V, U1, ArrayStorage<f32, C1, 1>>],
        third: &[Matrix<f32, W, U1, ArrayStorage<f32, C2, 1>>],
        forth: &[Matrix<f32, Y, U1, ArrayStorage<f32, C3, 1>>],
    ) -> Vec<f32> {
        multizip((self.vertices.iter(), first, second, third, forth))
            .flat_map(|(v, f, s, t, ff)| {
                let mut d = v.data.as_slice().to_vec();
                d.extend(f.data.as_slice());
                d.extend(s.data.as_slice());
                d.extend(t.data.as_slice());
                d.extend(ff.data.as_slice());
                d
            })
            .collect::<Vec<f32>>()
    }

    fn flatten_with_vertices<D, const S: usize>(
        &self,
        other: &[Matrix<f32, D, U1, ArrayStorage<f32, S, 1>>],
    ) -> Vec<f32> {
        self.vertices
            .iter()
            .zip(other)
            .flat_map(|(v, n)| {
                let mut d = v.data.as_slice().to_vec();
                d.extend(n.data.as_slice());
                d
            })
            .collect::<Vec<f32>>()
    }
}

#[derive(Debug)]
pub struct RenderingMesh {
    pub array_buffer: Arc<Buffer>,
    pub element_buffer: Option<Arc<Buffer>>,
    pub vertex_array: Arc<VertexArray>,
    mesh: Mesh,
    textures: Vec<Arc<Texture>>,
}

impl RenderingMesh {
    pub fn draw(&self) {
        self.vertex_array.bind();
        if self.element_buffer.is_some() {
            draw_elements(
                self.mesh.drawing_mode,
                self.mesh.len_vertices() as u32,
                OpenGlType::UnsignedInt,
            );
        } else {
            draw_arrays(self.mesh.drawing_mode, self.mesh.len_vertices() as u32);
        }
        VertexArray::unbind();
    }

    pub fn attach_to_program(&self, program: &Program) {
        if let Some(infos) = &self.mesh.textures {
            for (texture, info) in self.textures.iter().zip(infos.iter()) {
                texture.bind(info.id as u32);
                let texture_type = match info.texture_type {
                    TextureType::Diffuse => "diffuse",
                    TextureType::Specular => "specular",
                    TextureType::Normals => "normal",
                    TextureType::Height => "height",
                    TextureType::Metalness => "metalness",
                    TextureType::Roughness => "roughness",
                    TextureType::AmbientOcclusion => "ao",
                    _ => panic!("Can't happen"),
                };
                program.set_uniform_i1(&format!("material.{}", texture_type), info.id as i32);
            }
        }
        let shininess = self.mesh.shininess.unwrap_or(64f32);
        program.set_uniform_f1("material.shininess", shininess);
    }

    pub fn clone_with_textures(
        &self,
        _textures: Vec<TextureInfo>,
        _texture_loader: &mut TextureLoader,
    ) {
        todo!();
    }
}
