use crate::components::*;
use amethyst::{
    core::{
        Time,
        transform::Transform,
    },
    ecs::{Join, Read, System, WriteStorage},
};

#[derive(Default, Debug)]
pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Body>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut bodies, mut transforms, time): Self::SystemData) {
        // Apply forces
        for (body, transform) in (&mut bodies, &mut transforms).join() {
            body.update(time.delta_seconds());
            // TODO: Limit speeds
            transform.append_translation(body.velocity());
            transform.append_rotation_y_axis(body.angular_velocity());
        }
    }
}
