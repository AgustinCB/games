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
use mage::rendering::model::cube::rectangle;
use mage::rendering::Transform;

mod game_logic;
pub(crate) mod level;

const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 800.0;
const PLAYER_HEIGHT: f32 = 20.0;
const PLAYER_WIDTH: f32 = 100.0;

#[derive(Clone)]
pub(crate) struct GameTextures {
    pub(crate) background: TextureInfo,
    pub(crate) ball: TextureInfo,
    pub(crate) block_solid: TextureInfo,
    pub(crate) blue_block: TextureInfo,
    pub(crate) green_block: TextureInfo,
    pub(crate) orange_block: TextureInfo,
    pub(crate) paddle: TextureInfo,
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
                source: TextureSource::File(format!("{}/resources/textures/background.jpg", env!("CARGO_MANIFEST_DIR"))),
                texture_type: TextureType::Diffuse,
            },
            ball: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::File(format!("{}/resources/textures/awesomeface.png", env!("CARGO_MANIFEST_DIR"))),
                texture_type: TextureType::Diffuse,
            },
            block_solid: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!("{}/resources/textures/block_solid.png", env!("CARGO_MANIFEST_DIR")),
                    Vector4::new(204, 204, 178, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            blue_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!("{}/resources/textures/block.png", env!("CARGO_MANIFEST_DIR")),
                    Vector4::new(51, 153, 255, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            green_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!("{}/resources/textures/block.png", env!("CARGO_MANIFEST_DIR")),
                    Vector4::new(0, 178, 0, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            orange_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!("{}/resources/textures/block.png", env!("CARGO_MANIFEST_DIR")),
                    Vector4::new(255, 127, 0, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            paddle: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::File(
                    format!("{}/resources/textures/paddle.png", env!("CARGO_MANIFEST_DIR")),
                ),
                texture_type: TextureType::Diffuse,
            },
            white_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!("{}/resources/textures/block.png", env!("CARGO_MANIFEST_DIR")),
                    Vector4::new(255, 255, 255, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            yellow_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!("{}/resources/textures/block.png", env!("CARGO_MANIFEST_DIR")),
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
        Fixed2dCameraBuilder::new(Point2::new(0.0, 0.0), Point2::new(WIDTH, HEIGHT)).build();
    let mut game = GameBuilder::new("Breakout", WIDTH as _, HEIGHT as _).unwrap().build(
        SimpleEngine::new(camera, Vector3::new(0.0, 0.0, 0.0), texture_loader.clone()).unwrap(),
    );
    let mut transform = Transform::identity();
    transform.position = Vector3::new(WIDTH / 2.0, HEIGHT / 2.0, 0.0);
    game.spawn((
        rectangle(WIDTH / 2.0, HEIGHT / 2.0, vec![textures.background.clone()]),
        transform,
    ));
    let mut player_transform = Transform::identity();
    player_transform.position = Vector3::new(WIDTH / 2.0, PLAYER_HEIGHT / 2.0, 1.0);
    game.spawn((
        rectangle(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0, vec![textures.paddle.clone()]),
        player_transform,
    ));
    game.play(vec![Box::new(
        game_logic::GameLogic::new(texture_loader, textures, WIDTH as _, HEIGHT as _).unwrap(),
    )])
    .unwrap();
}
