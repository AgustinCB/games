use std::collections::HashMap;

use nalgebra::Vector3;
use russimp::texture::TextureType;

use mage::core::game::GameBuilder;
use mage::gameplay::camera::FixedCameraBuilder;
use mage::rendering::engine::SimpleEngine;
use mage::rendering::model::cube::cuboid;
use mage::rendering::model::mesh::{TextureInfo, TextureSource};
use mage::rendering::model::sphere::sphere;
use mage::rendering::opengl::texture::{TextureParameter, TextureParameterValue};
use mage::rendering::TransformBuilder;
use rapier3d::dynamics::RigidBodyBuilder;
use rapier3d::geometry::ColliderBuilder;

pub fn main() {
    env_logger::init();
    let camera = FixedCameraBuilder::new(800, 600, Vector3::new(0f32, 0f32, 10f32)).build();
    let mut game = GameBuilder::new("Basic Physics", 800, 600)
        .unwrap()
        .build(SimpleEngine::new(camera, Vector3::new(0.3, 0.3, 0.5)).unwrap());
    let cube = cuboid(
        100.0,
        0.1,
        1.0,
        vec![TextureInfo {
            id: 0,
            texture_type: TextureType::Diffuse,
            source: TextureSource::Color(Vector3::new(255, 0, 0)),
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
        }],
    );
    let sphere = sphere(
        0.5,
        vec![TextureInfo {
            id: 0,
            texture_type: TextureType::Diffuse,
            source: TextureSource::Color(Vector3::new(0, 255, 0)),
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
        }],
    );
    let sphere_position = Vector3::new(0.0, 4.5, 0.0);
    let floor_position = Vector3::new(0.0, -4f32, 0.0);
    let sphere_transform = TransformBuilder::new()
        .with_position(sphere_position)
        .build();
    let cube_collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0)
        .translation(floor_position)
        .build();
    let sphere_collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let sphere_rigidbody = RigidBodyBuilder::dynamic()
        .translation(sphere_position)
        .build();
    let transform = TransformBuilder::new()
        .with_position(floor_position)
        .build();

    let cube_entity = game.spawn((cube, transform));
    game.add_collider(cube_entity, cube_collider);

    let sphere_entity = game.spawn((sphere, sphere_transform));
    game.add_collider_and_rigidbody(sphere_entity, sphere_collider, sphere_rigidbody);

    game.play(vec![]).unwrap();
}
