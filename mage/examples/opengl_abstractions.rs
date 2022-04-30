use std::collections::HashMap;
use std::sync::Arc;

use hecs::World;
use nalgebra::Vector4;
use russimp::texture::TextureType;

use mage::core::game::Game;
use mage::core::system::System;
use mage::rendering::model::mesh::{TextureInfo, TextureSource};
use mage::rendering::model::plane::vertical_plane;
use mage::rendering::opengl::buffer::{Buffer, BufferType, BufferUsage};
use mage::rendering::opengl::program::Program;
use mage::rendering::opengl::shader::{Shader, ShaderType};
use mage::rendering::opengl::texture::{Texture, TextureParameter, TextureParameterValue};
use mage::rendering::opengl::vertex_array::{DataType, VertexArray};
use mage::rendering::opengl::{
    clear, draw, set_clear_color, DrawingBuffer, DrawingMode, OpenGlType,
};
use mage::resources::texture::TextureLoader;
use mage::MageError;

const VERTEX_SHADER: &'static str = "#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;

void main()
{
    vec3 position = aPos * 0.5;
    gl_Position = vec4(position, 1.0);
    ourColor = vec3(aTexCoord, 1.0);
    TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}";

const FRAGMENT_SHADER: &'static str = "#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D texture1;

void main()
{
    FragColor = texture(texture1, TexCoord) * vec4(ourColor, 1);
}";

struct GameSystem {
    program: Program,
    texture: Arc<Texture>,
    vertex_array: VertexArray,
}

impl System for GameSystem {
    fn name(&self) -> &str {
        "Game System"
    }

    fn start(&self, _world: &mut World) -> Result<(), String> {
        set_clear_color(Vector4::new(0.3, 0.3, 0.5, 1.0));
        Ok(())
    }

    fn early_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), String> {
        clear(&vec![DrawingBuffer::Color]);
        Ok(())
    }

    fn update(&self, _world: &mut World, _delta_time: u64) -> Result<(), String> {
        self.texture.bind(0);
        self.program.use_program();
        self.vertex_array.bind();
        draw(DrawingMode::Triangles, 6, OpenGlType::UnsignedInt);
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
    let mut game = Game::new("Opengl abstractions", 800, 600).unwrap();
    let program = Program::new(
        Shader::new(ShaderType::Vertex, VERTEX_SHADER).unwrap(),
        Shader::new(ShaderType::Fragment, FRAGMENT_SHADER).unwrap(),
    )
    .unwrap();
    let quad = vertical_plane(vec![]);
    let vertex_array = VertexArray::new();
    let array_buffer = Buffer::new(BufferType::Array);
    let element_buffer = Buffer::new(BufferType::ElementArray);
    vertex_array.bind();
    array_buffer.bind();
    array_buffer.set_data(&quad.flattened_data(), BufferUsage::StaticDraw);
    element_buffer.bind();
    element_buffer.set_data(&quad.indices.clone().unwrap(), BufferUsage::StaticDraw);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 0, 8, 3, 0, false);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 1, 8, 3, 3, false);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 2, 8, 2, 6, false);

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
    })
    .unwrap();
    let game_system = GameSystem {
        program,
        texture,
        vertex_array,
    };
    game.play(vec![Box::new(game_system)]).unwrap();
}
