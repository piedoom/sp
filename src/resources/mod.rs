mod material;
pub mod prefabs;
mod primitive;

pub use material::MaterialResource;
pub use primitive::PrimitiveResource;

use amethyst::{assets::ProgressCounter, core::ecs::World};

pub trait GameResource {
    fn initialize(world: &mut World, progress_counter: &mut ProgressCounter);
}

pub struct ResourceBundle {}

impl GameResource for ResourceBundle {
    fn initialize(world: &mut World, progress_counter: &mut ProgressCounter) {
        PrimitiveResource::initialize(world, progress_counter);
        MaterialResource::initialize(world, progress_counter);
        prefabs::CharacterPrefabs::initialize(world, progress_counter);
    }
}
