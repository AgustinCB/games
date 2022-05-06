use crate::MageError;
use hecs::World;

pub trait System {
    fn name(&self) -> &str;
    fn start(&self, world: &mut World) -> Result<(), MageError>;
    fn early_update(&self, world: &mut World, _delta_time: u64) -> Result<(), MageError>;
    fn update(&self, world: &mut World, _delta_time: u64) -> Result<(), MageError>;
    fn late_update(&self, world: &mut World, _delta_time: u64) -> Result<(), MageError>;
}
