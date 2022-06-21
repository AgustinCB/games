use std::collections::HashMap;

use hecs::World;
use include_dir::{include_dir, Dir};
use nalgebra::{Vector3, Vector4};
use rapier3d::math::Rotation;
use russimp::texture::TextureType;

use mage::core::game::GameBuilder;
use mage::gameplay::camera::{Camera, FixedCameraBuilder};
use mage::rendering::engine::Engine;
use mage::rendering::model::cube::cube;
use mage::rendering::model::mesh::{RenderingMesh, TextureInfo, TextureSource};
use mage::rendering::opengl::program::Program;
use mage::rendering::opengl::shader::ShaderType;
use mage::rendering::opengl::texture::{TextureParameter, TextureParameterValue};
use mage::rendering::opengl::{clear, enable, set_clear_color, DrawingBuffer, Feature};
use mage::rendering::TransformBuilder;
use mage::resources::shader::ShaderLoader;
use mage::resources::texture::TextureLoader;
use mage::MageError;
use std::sync::Arc;

static SHADER_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/examples/resources/shaders");

struct GameEngine {
    program: Program,
    rendering_mesh: RenderingMesh,
}

impl GameEngine {
    fn new() -> Result<GameEngine, MageError> {
        let loader = ShaderLoader::new(&SHADER_DIR)?;
        let program = Program::new(
            loader.load(ShaderType::Vertex, "basic_vertex.glsl")?,
            loader.load(ShaderType::Fragment, "basic_fragment.glsl")?,
        )?;
        let cube = cube(vec![TextureInfo {
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
        let camera = FixedCameraBuilder::new(800, 600, Vector3::new(0f32, 0f32, 3f32)).build();
        program.use_program();
        program.set_uniform_i1("texture1", 0);
        program.set_uniform_matrix4(
            "model",
            TransformBuilder::new()
                .with_rotation(Rotation::from_axis_angle(
                    &Vector3::x_axis(),
                    -55f32.to_radians(),
                ))
                .build()
                .get_model_matrix(),
        );
        program.set_uniform_matrix4("view", camera.look_at_matrix());
        program.set_uniform_matrix4("projection", camera.projection());
        let loader = Arc::new(TextureLoader::new());
        let rendering_mesh = cube.to_rendering_mesh(loader)?;
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
        enable(Feature::Depth);
        set_clear_color(Vector4::new(0.3, 0.3, 0.5, 1.0));
        Ok(())
    }

    fn render(&self, _world: &mut World, _delta_time: f32) -> Result<(), MageError> {
        clear(&vec![DrawingBuffer::Color, DrawingBuffer::Depth]);
        self.program.use_program();
        self.rendering_mesh.draw();
        Ok(())
    }
}

pub fn main() {
    env_logger::init();
    let mut game = GameBuilder::new("Fixed camera", 800, 600)
        .unwrap()
        .build(GameEngine::new().unwrap());
    game.play(vec![]).unwrap();
}
