use std::collections::HashMap;

use hecs::World;
use nalgebra::Vector4;
use russimp::texture::TextureType;

use mage::core::game::GameBuilder;
use mage::rendering::engine::Engine;
use mage::rendering::model::mesh::{RenderingMesh, TextureInfo, TextureSource};
use mage::rendering::model::plane::vertical_plane;
use mage::rendering::opengl::program::Program;
use mage::rendering::opengl::shader::{Shader, ShaderType};
use mage::rendering::opengl::texture::{TextureParameter, TextureParameterValue};
use mage::rendering::opengl::{clear, set_clear_color, DrawingBuffer};
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

struct GameEngine {
    program: Program,
    rendering_mesh: RenderingMesh,
}

impl GameEngine {
    fn new() -> Result<GameEngine, MageError> {
        let program = Program::new(
            Shader::new(ShaderType::Vertex, VERTEX_SHADER).unwrap(),
            Shader::new(ShaderType::Fragment, FRAGMENT_SHADER).unwrap(),
        )
        .unwrap();
        let quad = vertical_plane(vec![TextureInfo {
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
        }]);
        program.use_program();
        program.set_uniform_i1("texture1", 0);
        let rendering_mesh = quad.to_rendering_mesh()?;
        program.use_program();
        rendering_mesh.attach_to_program(&program);
        Ok(GameEngine {
            program,
            rendering_mesh,
        })
    }
}

impl Engine for GameEngine {
    fn setup(&self, _world: &mut World) -> Result<(), MageError> {
        set_clear_color(Vector4::new(0.3, 0.3, 0.5, 1.0));
        Ok(())
    }

    fn render(&self, _world: &mut World, _delta_time: f32) -> Result<(), MageError> {
        clear(&vec![DrawingBuffer::Color]);
        self.program.use_program();
        self.rendering_mesh.draw();
        Ok(())
    }
}

pub fn main() {
    env_logger::init();
    let mut game = GameBuilder::new("Opengl abstractions", 800, 600)
        .unwrap()
        .build(GameEngine::new().unwrap());
    game.play(vec![]).unwrap();
}
