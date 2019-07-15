use crate::components::Thruster;
use amethyst::core::Time;
use amethyst::core::{math::Vector3, Transform};
use amethyst::ecs::{Join, Read, System, WriteStorage};

#[derive(Default, Debug)]
pub struct ThrusterSystem;

impl<'a> System<'a> for ThrusterSystem {
    type SystemData = (
        WriteStorage<'a, Thruster>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut thrusters, mut transforms, time): Self::SystemData) {
        for (thruster, transform) in (&mut thrusters, &mut transforms).join() {
            // rotate based on unit points
            transform.append_rotation_z_axis(
                // This will orient the rotation direction correctly
                thruster.rotation_control *
                // Multiply by our turn speed, which is just a multiplier.
                thruster.turn_speed *
                // Finally, multiply everything by our delta to keep consistent across framerates
                time.delta_seconds(),
            );

            // If our input is 0, we're not changing our velocity.
            if thruster.thrust_control != 0. {
                // Calculate impulse
                let added_magnitude = Vector3::y().scale(
                    thruster.traction
                        * time.delta_seconds()
                        * thruster.thrust_control,
                );
                let added_vector = transform.rotation() * added_magnitude;

                // Change our velocity vector
                thruster.velocity += added_vector;

                // add an initial velocity if applicable
                if thruster.start_speed > 0.0 {
                    thruster.velocity += transform.rotation() * Vector3::y().scale(thruster.start_speed);
                    // reset initial velocity so we don't apply across more than one frame
                    thruster.start_speed = 0.0;
                }

                // Limit velocity
                let magnitude = thruster.velocity.magnitude();
                if magnitude > thruster.max_speed {
                    thruster.velocity /= magnitude / thruster.max_speed;
                } else if magnitude < thruster.min_speed {
                    thruster.velocity = transform.rotation() * Vector3::y().scale(thruster.min_speed);
                }
            }

            // Apply existing velocity and rotational velocity.
            let movement = thruster.velocity.scale(time.delta_seconds());

            // Finally, actually transform
            transform.prepend_translation(movement);
        }
    }
}

