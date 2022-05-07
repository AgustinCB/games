use std::collections::HashMap;

use nalgebra::{Rotation, Vector3};
use russimp::texture::TextureType;

use mage::core::game::GameBuilder;
use mage::gameplay::camera::FixedCameraBuilder;
use mage::rendering::engine::SimpleEngine;
use mage::rendering::model::cube::cube;
use mage::rendering::model::mesh::{TextureInfo, TextureSource};
use mage::rendering::opengl::texture::{TextureParameter, TextureParameterValue};
use mage::rendering::TransformBuilder;

pub fn main() {
    env_logger::init();
    let camera = FixedCameraBuilder::new(800, 600, Vector3::new(0f32, 0f32, 3f32)).build();
    let mut game = GameBuilder::new("Fixed camera", 800, 600)
        .unwrap()
        .build(SimpleEngine::new(camera, Vector3::new(0.3, 0.3, 0.5)).unwrap());
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
    game.play(vec![]).unwrap();
}
