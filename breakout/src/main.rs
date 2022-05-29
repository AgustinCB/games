use mage::core::game::GameBuilder;
use mage::gameplay::camera::Fixed2dCameraBuilder;
use mage::rendering::engine::SimpleEngine;
use nalgebra::{Point2, Vector3};

fn main() {
    env_logger::init();
    let camera =
        Fixed2dCameraBuilder::new(Point2::new(0.0, 0.0), Point2::new(800.0, 600.0)).build();
    let mut game = GameBuilder::new("Breakout", 800, 600)
        .unwrap()
        .build(SimpleEngine::new(camera, Vector3::new(0.0, 0.0, 0.0)).unwrap());
    game.play(vec![]).unwrap();
}
