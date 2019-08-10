use crate::{components::*, resources::*};
use amethyst::core::{ecs::prelude::*, math::Vector3, Time, Transform};
use specs_physics::{
    colliders::Shape,
    nphysics::{algebra::Velocity3, object::BodyStatus},
    PhysicsBodyBuilder, PhysicsColliderBuilder,
};

#[derive(Default, Debug)]
pub struct CharacterSystem;

impl<'a> System<'a> for CharacterSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Character>,
        WriteStorage<'a, CharacterData>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, Time>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, PrimitiveResource>,
        ReadExpect<'a, MaterialResource>,
    );

    fn run(
        &mut self,
        (entities, mut characters, mut datas, transforms, time, lazy, primitives, materials): Self::SystemData,
    ) {
        // Loop through all players and assign direction
        for (character, data, transform) in (&mut characters, &mut datas, &transforms).join() {
            let mut ptransform = transform.clone();
            ptransform.set_scale(Vector3::new(1f32, 1f32, 1f32).scale(0.1));
            // get the direction of the player
            let direction = transform.rotation() * Vector3::z();
            match character {
                // Quartz Character logic
                Character::Quartz => {
                    if data.attack {
                        match data
                            .basic_attack_timer
                            .check_and_reset(&time.absolute_time())
                        {
                            timer::TimerStatus::Complete(_) => {
                                lazy.create_entity(&entities)
                                    .with(
                                        PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                                            .velocity(Velocity3::new(
                                                direction.scale(data.basic_attack_speed),
                                                Vector3::zeros(),
                                            ))
                                            .build(),
                                    )
                                    .with(primitives.sphere())
                                    .with(lifetime::DistanceLimit::new(
                                        transform.translation().clone(),
                                        data.basic_attack_range,
                                    ))
                                    .with(ptransform)
                                    .with(materials.diffuse_white.clone().unwrap())
                                    .build();
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}
