use crate::MageError;
use hecs::World;
use include_dir::{include_dir, Dir};

mod simple;

pub(crate) const SHADER_LIBRARY: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/shaders");

pub trait Engine {
    fn setup(&self, world: &mut World) -> Result<(), MageError>;

    fn render(&self, world: &mut World, delta_time: f32) -> Result<(), MageError>;
}

pub use simple::SimpleEngine;
