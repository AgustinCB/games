use std::collections::HashMap;

use hecs::World;
use nalgebra::{Rotation, Vector3};
use russimp::texture::TextureType;

use mage::core::game::Game;
use mage::core::system::System;
use mage::gameplay::camera::{FixedCamera, FixedCameraBuilder};
use mage::rendering::engine::{Engine, SimpleEngine};
use mage::rendering::model::cube::cube;
use mage::rendering::model::mesh::{TextureInfo, TextureSource};
use mage::rendering::opengl::texture::{TextureParameter, TextureParameterValue};
use mage::rendering::TransformBuilder;
use mage::MageError;

struct GameSystem {
    engine: SimpleEngine<FixedCamera>,
}

impl GameSystem {
    fn new() -> Result<GameSystem, MageError> {
        let camera = FixedCameraBuilder::new(800, 600, Vector3::new(0f32, 0f32, 3f32)).build();
        Ok(GameSystem {
            engine: SimpleEngine::new(camera, Vector3::new(0.3, 0.3, 0.5))?,
        })
    }
}

impl System for GameSystem {
    fn name(&self) -> &str {
        "Game System"
    }

    fn start(&self, world: &mut World) -> Result<(), MageError> {
        self.engine.setup(world)?;
        Ok(())
    }

    fn early_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }

    fn update(&self, world: &mut World, delta_time: u64) -> Result<(), MageError> {
        self.engine.render(world, delta_time)?;
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}

pub fn main() {
    env_logger::init();
    let mut game = Game::new("Fixed camera", 800, 600).unwrap();
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
    let transform = TransformBuilder::new()
        .with_rotation(Rotation::from_axis_angle(
            &Vector3::x_axis(),
            -55f32.to_radians(),
        ))
        .build();
    game.spawn((cube, transform));
    game.play(vec![Box::new(GameSystem::new().unwrap())])
        .unwrap();
}
