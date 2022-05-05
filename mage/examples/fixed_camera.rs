use std::collections::HashMap;
use std::sync::Arc;

use hecs::World;
use include_dir::{Dir, include_dir};
use nalgebra::{Rotation, Translation3, Vector3, Vector4};
use russimp::texture::TextureType;

use mage::core::game::Game;
use mage::core::system::System;
use mage::gameplay::camera::{Camera, FixedCameraBuilder};
use mage::rendering::model::cube::cube;
use mage::rendering::model::mesh::{Mesh, TextureInfo, TextureSource};
use mage::rendering::opengl::buffer::{Buffer, BufferType, BufferUsage};
use mage::rendering::opengl::program::Program;
use mage::rendering::opengl::shader::ShaderType;
use mage::rendering::opengl::texture::{Texture, TextureParameter, TextureParameterValue};
use mage::rendering::opengl::vertex_array::{DataType, VertexArray};
use mage::rendering::opengl::{clear, enable, set_clear_color, DrawingBuffer, Feature};
use mage::resources::shader::ShaderLoader;
use mage::resources::texture::TextureLoader;
use mage::MageError;

static SHADER_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/examples/resources/shaders");

struct GameSystem {
    _array_buffer: Buffer,
    mesh: Mesh,
    program: Program,
    texture: Arc<Texture>,
    vertex_array: VertexArray,
}

impl GameSystem {
    fn new() -> Result<GameSystem, MageError> {
        let loader = ShaderLoader::new(&SHADER_DIR)?;
        let program = Program::new(
            loader.load(ShaderType::Vertex, "basic_vertex.glsl")?,
            loader.load(ShaderType::Fragment, "basic_fragment.glsl")?,
        )?;
        let cube = cube(vec![]);
        let vertex_array = VertexArray::new();
        let array_buffer = Buffer::new(BufferType::Array);
        vertex_array.bind();
        array_buffer.bind();
        array_buffer.set_data(&cube.flattened_data(), BufferUsage::StaticDraw);
        VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 0, 8, 3, 0, false);
        VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 1, 8, 3, 3, false);
        VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 2, 8, 2, 6, false);
        let camera = FixedCameraBuilder::new(800, 600, Vector3::new(0f32, 0f32, 3f32)).build();
        program.use_program();
        program.set_uniform_i1("texture1", 0);
        program.set_uniform_matrix4(
            "model",
            Rotation::from_axis_angle(&Vector3::x_axis(), -55f32.to_radians()).to_homogeneous(),
        );
        program.set_uniform_matrix4(
            "view",
            Translation3::new(0f32, 0f32, -3f32).to_homogeneous(),
        );
        program.set_uniform_matrix4("view", camera.look_at_matrix());
        program.set_uniform_matrix4("projection", camera.projection());

        let texture = load_texture(TextureInfo {
            id: 0,
            texture_type: TextureType::Diffuse,
            source: TextureSource::File(format!(
                "{}/examples/resources/container.jpg",
                env!("CARGO_MANIFEST_DIR")
            )),
            parameters: HashMap::from([
                (
                    TextureParameter::TextureWrapS,
                    TextureParameterValue::Repeat,
                ),
                (
                    TextureParameter::TextureWrapT,
                    TextureParameterValue::Repeat,
                ),
                (
                    TextureParameter::TextureMinFilter,
                    TextureParameterValue::LinearMipmapLinear,
                ),
                (
                    TextureParameter::TextureMagFilter,
                    TextureParameterValue::Linear,
                ),
            ]),
        })?;
        Ok(GameSystem {
            program,
            texture,
            vertex_array,
            _array_buffer: array_buffer,
            mesh: cube,
        })
    }
}

impl System for GameSystem {
    fn name(&self) -> &str {
        "Game System"
    }

    fn start(&self, _world: &mut World) -> Result<(), String> {
        enable(Feature::Depth);
        set_clear_color(Vector4::new(0.3, 0.3, 0.5, 1.0));
        Ok(())
    }

    fn early_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), String> {
        clear(&vec![DrawingBuffer::Color, DrawingBuffer::Depth]);
        Ok(())
    }

    fn update(&self, _world: &mut World, _delta_time: u64) -> Result<(), String> {
        self.texture.bind(0);
        self.program.use_program();
        self.vertex_array.bind();
        self.mesh.draw();
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), String> {
        Ok(())
    }
}

fn load_texture(texture_info: TextureInfo) -> Result<Arc<Texture>, MageError> {
    let mut loader = TextureLoader::new();
    loader.load_texture_2d(&texture_info)
}

pub fn main() {
    env_logger::init();
    let mut game = Game::new("Fixed camera", 800, 600).unwrap();
    game.play(vec![Box::new(GameSystem::new().unwrap())])
        .unwrap();
}
