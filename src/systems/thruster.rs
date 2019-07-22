use crate::components::*;
use amethyst::core::Time;
use amethyst::core::{math::Vector3, Transform};
use amethyst::ecs::{Join, Read, System, WriteStorage};

#[derive(Default, Debug)]
pub struct ThrusterSystem;

impl<'a> System<'a> for ThrusterSystem {
    type SystemData = (
        WriteStorage<'a, Body>,
        WriteStorage<'a, Thruster>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut bodies, mut thrusters, mut transforms, time): Self::SystemData) {
        for (body, thruster, transform) in (&mut bodies, &mut thrusters, &mut transforms).join() {
            // body.
            // rotate based on unit points
            // transform.append_rotation_y_axis(
            //     // This will orient the rotation direction correctly
            //     thruster.rotation_control *
            //     // Multiply by our angular acceleration, which is just a multiplier.
            //     thruster.rotational_force *
            //     // Finally, multiply everything by our delta to keep consistent across framerates
            //     time.delta_seconds(),
            // );

            // If we have no input, we're not changing anything
            if thruster.thrust_control != 0. {
                // Calculate impulse
                let magnitude_scalar = Vector3::z().scale(
                    thruster.thrust_force
                    * thruster.thrust_control,
                );
                let force = transform.rotation() * magnitude_scalar;

                // Limit the force if a max speed is set
                if thruster.max_velocity.is_some() {
                    let magnitude = body.velocity().magnitude();
                    if magnitude > thruster.max_velocity.unwrap() {
                        body.set_velocity(body.velocity() / magnitude / thruster.max_velocity.unwrap());
                    }
                }

                // Apply the thrust force to our physics body
                body.apply_force(force);                
            }
        }
    }
}

