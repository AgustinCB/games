use hecs::World;
use include_dir::{Dir, include_dir};

pub use simple::SimpleEngine;

use crate::MageError;

mod simple;

pub(crate) const SHADER_LIBRARY: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/shaders");

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderingParameters {
    pub blending_enabled: bool,
    pub particles_enabled: bool,
}

pub trait Engine {
    fn setup(
        &mut self,
        world: &mut World,
        rendering_parameters: RenderingParameters,
    ) -> Result<(), MageError>;

    fn render(&self, world: &mut World, delta_time: f32) -> Result<(), MageError>;
}
