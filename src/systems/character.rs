use crate::{components::*, resources::*};
use amethyst::core::{ecs::prelude::*, Time};
use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

#[derive(Default, Debug)]
pub struct CharacterSystem;

impl<'a> System<'a> for CharacterSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Character>,
        WriteStorage<'a, CharacterState>,
        Read<'a, Time>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, PrimitiveResource>,
        ReadExpect<'a, MaterialResource>,
    );

    fn run(
        &mut self,
        (entities, mut characters, mut states, time, lazy, primitives, materials): Self::SystemData,
    ) {
        // Loop through all players and assign direction
        for (character, state) in (&mut characters, &mut states).join() {
            match character {
                // Quartz Character logic
                Character::Quartz => {
                    if state.attack {
                        println!("firing");
                        lazy.create_entity(&entities)
                            .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic).build())
                            .with(
                                PhysicsColliderBuilder::<f32>::from(Shape::<f32>::Ball {
                                    radius: 1f32,
                                })
                                .build(),
                            )
                            .with(primitives.sphere())
                            // .with(materials.clone())
                            .build();
                    }
                }
            }
        }
    }
}
