use amethyst::{core::math::Vector3, ecs::prelude::*};

/// Essentially a Parent definition, except this one will mirror the Parent's translation only - no rotation.
#[derive(Clone, Default)]
pub struct Tracker {
    entity: Option<Entity>,
    // The mask can be used to restrict tracking
    // mask: (bool, bool, bool),
}

impl Component for Tracker {
    type Storage = DenseVecStorage<Self>;
}

impl Tracker {
    pub fn entity(&self) -> Entity {
        self.entity.unwrap()
    }

    pub fn new(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            // mask: (true, true, true),
        }
    }
}
