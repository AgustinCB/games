use hecs::World;

pub trait System {
    fn name(&self) -> &str;
    fn start(&self, world: &mut World) -> Result<(), String>;
    fn early_update(&self, world: &mut World, _delta_time: u64) -> Result<(), String>;
    fn update(&self, world: &mut World, _delta_time: u64) -> Result<(), String>;
    fn late_update(&self, world: &mut World, _delta_time: u64) -> Result<(), String>;
}
