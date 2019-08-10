//! This system handles entities that should be removed after certain conditions are met. This includes
//! time or distance limits.

use amethyst::{
    core::{Time, Transform},
    ecs::prelude::*,
};

use crate::components::*;

#[derive(Default, Debug)]
pub struct LifetimeSystem;

impl<'a> System<'a> for LifetimeSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, lifetime::DistanceLimit>,
        ReadStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (entities, distance_limits, transforms, time): Self::SystemData) {
        // Check all distances
        for (e, distance_limit, transform) in (&*entities, &distance_limits, &transforms).join() {
            if !distance_limit.check(&transform.translation()) {
                entities.delete(e);
            }
        }
    }
}
