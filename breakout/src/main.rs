#![feature(bool_to_option)]
extern crate core;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8};

use nalgebra::{Point2, Vector2, Vector3, Vector4};

use mage::core::game::{Game, GameBuilder};
use mage::gameplay::camera::{Fixed2dCamera, Fixed2dCameraBuilder};
use mage::gameplay::input::{Input, InputType};
use mage::MageError;
use mage::physics::{
    ActiveCollisionTypes, ActiveEvents, ColliderBuilder, Collisions, RigidBodyBuilder, Triggers,
    Velocity,
};
use mage::rendering::engine::SimpleEngine;
use mage::rendering::model::cube::rectangle;
use mage::rendering::model::mesh::{TextureInfo, TextureSource, TextureType};
use mage::rendering::opengl::texture::{TextureParameter, TextureParameterValue};
use mage::rendering::TransformBuilder;
use mage::resources::texture::TextureLoader;

use crate::bouncing_controls::{BouncingControlsSystem, BouncingProperties, BouncingStatus};
use crate::game_logic::{GameState, StartingProperties};
use crate::level::LevelElement;
use crate::player_controls::PlayerVelocity;

mod bouncing_controls;
mod game_logic;
pub(crate) mod level;
mod player_controls;

const BALL_RADIUS: f32 = 25.0;
const HEIGHT: f32 = 600.0;
const INITIAL_BALL_VELOCITY_X: f32 = 100.0 / 25.0;
const INITIAL_BALL_VELOCITY_Y: f32 = 350.0 / 25.0;
const INITIAL_PLAYER_VELOCITY: u32 = 12500;
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
                source: TextureSource::File(format!(
                    "{}/resources/textures/background.jpg",
                    env!("CARGO_MANIFEST_DIR")
                )),
                texture_type: TextureType::Diffuse,
            },
            ball: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::File(format!(
                    "{}/resources/textures/awesomeface.png",
                    env!("CARGO_MANIFEST_DIR")
                )),
                texture_type: TextureType::Diffuse,
            },
            block_solid: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!(
                        "{}/resources/textures/block_solid.png",
                        env!("CARGO_MANIFEST_DIR")
                    ),
                    Vector4::new(204, 204, 178, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            blue_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!(
                        "{}/resources/textures/block.png",
                        env!("CARGO_MANIFEST_DIR")
                    ),
                    Vector4::new(51, 153, 255, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            green_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!(
                        "{}/resources/textures/block.png",
                        env!("CARGO_MANIFEST_DIR")
                    ),
                    Vector4::new(0, 178, 0, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            orange_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!(
                        "{}/resources/textures/block.png",
                        env!("CARGO_MANIFEST_DIR")
                    ),
                    Vector4::new(255, 127, 0, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            paddle: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::File(format!(
                    "{}/resources/textures/paddle.png",
                    env!("CARGO_MANIFEST_DIR")
                )),
                texture_type: TextureType::Diffuse,
            },
            white_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!(
                        "{}/resources/textures/block.png",
                        env!("CARGO_MANIFEST_DIR")
                    ),
                    Vector4::new(255, 255, 255, 255),
                ),
                texture_type: TextureType::Diffuse,
            },
            yellow_block: TextureInfo {
                id: 0,
                parameters: parameters.clone(),
                source: TextureSource::ColoredFile(
                    format!(
                        "{}/resources/textures/block.png",
                        env!("CARGO_MANIFEST_DIR")
                    ),
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
    let mut game = GameBuilder::new("Breakout", WIDTH as _, HEIGHT as _)
        .unwrap()
        .with_frame_rate(1000 / 120)
        .build(
            SimpleEngine::new(camera, Vector3::new(0.0, 0.0, 0.0), texture_loader.clone()).unwrap(),
        );
    add_map(&textures, &mut game);
    add_player(&textures, &mut game);
    add_ball(&textures, &mut game);
    let status = Arc::new(AtomicU8::new(GameState::Active as _));
    let unstick = Arc::new(AtomicBool::default());
    game.play(vec![
        Box::new(
            game_logic::GameLogic::new(
                texture_loader,
                textures,
                WIDTH as _,
                HEIGHT as _,
                status.clone(),
            )
                .unwrap(),
        ),
        Box::new(player_controls::PlayerControlsSystem {
            unstick: unstick.clone(),
            against_left_wall: RefCell::new(false),
            against_right_wall: RefCell::new(false),
            hx: PLAYER_WIDTH / 2.0,
            width: WIDTH,
        }),
        Box::new(BouncingControlsSystem { unstick, game_state: status }),
    ])
        .unwrap();
}

fn add_map(textures: &GameTextures, game: &mut Game<SimpleEngine<Fixed2dCamera>>) {
    let background_position = Vector3::new(WIDTH / 2.0, HEIGHT / 2.0, 0.0);
    let transform = TransformBuilder::new()
        .with_position(background_position)
        .build();
    game.spawn((
        rectangle(WIDTH / 2.0, HEIGHT / 2.0, vec![textures.background.clone()]),
        transform,
    ));
    add_frontier(
        game,
        Vector3::new(WIDTH / 2.0, -0.5, 0.0),
        WIDTH / 2.0,
        0.5,
        true,
        LevelElement::BottomWall,
    );
    add_frontier(
        game,
        Vector3::new(WIDTH / 2.0, HEIGHT + 0.5, 0.0),
        WIDTH / 2.0,
        0.5,
        false,
        LevelElement::TopWall,
    );
    add_frontier(
        game,
        Vector3::new(-0.5, HEIGHT / 2.0, 0.0),
        0.5,
        HEIGHT / 2.0,
        false,
        LevelElement::LeftWall,
    );
    add_frontier(
        game,
        Vector3::new(WIDTH + 0.5, HEIGHT / 2.0, 0.0),
        0.5,
        HEIGHT / 2.0,
        false,
        LevelElement::RightWall,
    );
}

fn add_frontier(
    game: &mut Game<SimpleEngine<Fixed2dCamera>>,
    position: Vector3<f32>,
    hx: f32,
    hy: f32,
    is_sensor: bool,
    element: LevelElement,
) {
    let transform = TransformBuilder::new().with_position(position).build();
    let frontier = game.spawn((transform, ));
    let collider = ColliderBuilder::cuboid(hx, hy, 1.0)
        .user_data(element as _)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .translation(position)
        .sensor(is_sensor)
        .build();
    game.add_collider(frontier, collider);
}

fn add_ball(textures: &GameTextures, game: &mut Game<SimpleEngine<Fixed2dCamera>>) {
    let position = Vector3::new(
        WIDTH / 2.0,
        PLAYER_HEIGHT + BALL_RADIUS,
        0.3,
    );
    let transform = TransformBuilder::new().build();
    let handle = game.spawn((
        rectangle(BALL_RADIUS, BALL_RADIUS, vec![textures.ball.clone()]),
        transform,
        Collisions(vec![]),
        Triggers(vec![]),
        Velocity(Vector3::zeros()),
        StartingProperties {
            position,
            velocity: Vector3::new(INITIAL_BALL_VELOCITY_X, INITIAL_BALL_VELOCITY_Y, 0.0),
        },
        BouncingProperties {
            max_distance: PLAYER_WIDTH / 2.0,
            initial_velocity: Vector2::new(INITIAL_BALL_VELOCITY_X, INITIAL_BALL_VELOCITY_Y),
            current_velocity: Vector2::new(INITIAL_BALL_VELOCITY_X, INITIAL_BALL_VELOCITY_Y),
            status: BouncingStatus::Stuck,
        },
    ));
    let rigidbody = RigidBodyBuilder::kinematic_velocity_based()
        .translation(position)
        .build();
    let collider = ColliderBuilder::ball(BALL_RADIUS)
        .user_data(LevelElement::Ball as _)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .active_collision_types(
            ActiveCollisionTypes::KINEMATIC_KINEMATIC | ActiveCollisionTypes::KINEMATIC_STATIC,
        )
        .build();
    game.add_collider_and_rigidbody(handle, collider, rigidbody);
}

fn add_player(textures: &GameTextures, game: &mut Game<SimpleEngine<Fixed2dCamera>>) {
    let player_position = Vector3::new(WIDTH / 2.0, PLAYER_HEIGHT / 2.0, 0.3);
    let player_transform = TransformBuilder::new().build();
    let player_handle = game.spawn((
        rectangle(
            PLAYER_WIDTH / 2.0,
            PLAYER_HEIGHT / 2.0,
            vec![textures.paddle.clone()],
        ),
        player_transform,
        Input {
            input_types: vec![InputType::Keyboard],
            events: vec![],
        },
        Collisions(vec![]),
        PlayerVelocity(INITIAL_PLAYER_VELOCITY as f32),
        StartingProperties {
            position: player_position,
            velocity: Vector3::zeros(),
        },
        Velocity(Vector3::zeros()),
    ));
    let rigidbody = RigidBodyBuilder::kinematic_velocity_based()
        .translation(player_position)
        .gravity_scale(1.0)
        .build();
    let collider = ColliderBuilder::cuboid(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0, 0.1)
        .user_data(LevelElement::Player as u128)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .active_collision_types(
            ActiveCollisionTypes::KINEMATIC_KINEMATIC | ActiveCollisionTypes::KINEMATIC_STATIC,
        )
        .build();
    game.add_collider_and_rigidbody(player_handle, collider, rigidbody);
}
