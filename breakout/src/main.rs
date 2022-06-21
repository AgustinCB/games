use mage::core::game::GameBuilder;
use mage::gameplay::camera::Fixed2dCameraBuilder;
use mage::rendering::engine::SimpleEngine;
use mage::rendering::model::mesh::{TextureInfo, TextureSource, TextureType};
use mage::rendering::opengl::texture::{TextureParameter, TextureParameterValue};
use mage::resources::texture::TextureLoader;
use mage::MageError;
use nalgebra::{Point2, Vector3, Vector4};
use std::collections::HashMap;
use std::sync::Arc;

mod game_logic;
pub(crate) mod level;

#[derive(Clone)]
pub(crate) struct GameTextures {
    pub(crate) background: TextureInfo,
    pub(crate) ball: TextureInfo,
    pub(crate) block_solid: TextureInfo,
    pub(crate) blue_block: TextureInfo,
    pub(crate) green_block: TextureInfo,
    pub(crate) orange_block: TextureInfo,
    pub(crate) white_block: TextureInfo,
    pub(crate) yellow_block: TextureInfo,
}

impl GameTextures {
    fn new() -> Result<GameTextures, MageError> {
        let mut parameters = HashMap::new();
        parameters.insert(
            TextureParameter::TextureWrapS,
            TextureParameterValue::Repeat,
        );
        parameters.insert(
            TextureParameter::TextureWrapT,
            TextureParameterValue::Repeat,
        );
        parameters.insert(
            TextureParameter::TextureMinFilter,
            TextureParameterValue::Linear,
        );
        parameters.insert(
            TextureParameter::TextureMagFilter,
            TextureParameterValue::Linear,
        );
        Ok(GameTextures {
            background: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::File("resources/textures/background.jpg".to_owned()),
                texture_type: TextureType::Diffuse,
            },
            ball: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::File("resources/textures/awesomeface.png".to_owned()),
                texture_type: TextureType::Diffuse,
            },
            block_solid: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    "resources/textures/block_solid.png".to_owned(),
                    Vector4::new(204, 204, 178, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            blue_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    "resources/textures/block.png".to_owned(),
                    Vector4::new(51, 153, 255, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            green_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    "resources/textures/block.png".to_owned(),
                    Vector4::new(0, 178, 0, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            orange_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    "resources/textures/block.png".to_owned(),
                    Vector4::new(255, 127, 0, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            white_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    "resources/textures/block.png".to_owned(),
                    Vector4::new(255, 255, 255, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            yellow_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    "resources/textures/block.png".to_owned(),
                    Vector4::new(204, 204, 102, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
        })
    }
}

fn main() {
    env_logger::init();
    let textures = GameTextures::new().unwrap();
    let texture_loader = Arc::new(TextureLoader::new());
    let camera =
        Fixed2dCameraBuilder::new(Point2::new(0.0, 0.0), Point2::new(800.0, 600.0)).build();
    let mut game = GameBuilder::new("Breakout", 800, 600).unwrap().build(
        SimpleEngine::new(camera, Vector3::new(0.0, 0.0, 0.0), texture_loader.clone()).unwrap(),
    );
    game.play(vec![Box::new(
        game_logic::GameLogic::new(texture_loader, textures, 800, 600).unwrap(),
    )])
    .unwrap();
}
