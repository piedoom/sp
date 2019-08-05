use crate::components::*;
use amethyst::core::Time;
use amethyst::core::{math::Vector3, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use nalgebra_glm::clamp_vec;

use specs_physics::{
    colliders::Shape,
    nalgebra::*,
    nphysics::{
        algebra::Velocity3,
        math::{Force, ForceType},
        object::BodyStatus,
    },
    parameters::Gravity,
    PhysicsBody, PhysicsBodyBuilder, PhysicsColliderBuilder,
};

const ANGULAR_DAMPING: f32 = 20.00f32;
const LINEAR_DAMPING: f32 = 1f32;

#[derive(Default, Debug)]
pub struct ThrusterSystem;

impl<'a> System<'a> for ThrusterSystem {
    type SystemData = (
        WriteStorage<'a, Thruster>,
        WriteStorage<'a, PhysicsBody<f32>>,
        ReadStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut thrusters, mut bodies, transforms, time): Self::SystemData) {
        for (body, thruster, transform) in (&mut bodies, &mut thrusters, &transforms).join() {
            // get linear acceleration by multiplying its control by speed factor
            let linear_acceleration =
                transform.rotation() * Vector3::z().scale(thruster.thrust_force);
            // get angular acceleration
            let angular_acceleration = Vector3::y().scale(thruster.rotational_force);
            // apply forces
            body.apply_external_force(&Force::new(
                linear_acceleration.scale(thruster.thrust_control),
                angular_acceleration.scale(thruster.rotation_control),
            ));
            // calculate damping
            let mut angular_damping = body.velocity.angular.scale(-1f32).scale(ANGULAR_DAMPING);
            let mut linear_damping = body.velocity.linear.scale(-1f32).scale(LINEAR_DAMPING);
            // only apply forces if controls are not 0 (or on)
            if thruster.thrust_control != 0f32 {
                linear_damping = Vector3::zeros()
            }
            if thruster.rotation_control != 0f32 {
                angular_damping = Vector3::zeros()
            }
            body.apply_external_force(&Force::new(linear_damping, angular_damping));

            // Limit the force if a max speed is set
            if thruster.max_speed.is_some() {
                if body.velocity.linear.magnitude() > thruster.max_speed.unwrap() {
                    body.velocity.linear = body
                        .velocity
                        .linear
                        .scale(thruster.max_speed.unwrap() / body.velocity.linear.magnitude());
                }
            }
            // Limit the force if a max speed is set
            if thruster.max_angular_speed.is_some() {
                if body.velocity.angular.magnitude() > thruster.max_angular_speed.unwrap() {
                    body.velocity.angular = body.velocity.angular.scale(
                        thruster.max_angular_speed.unwrap() / body.velocity.angular.magnitude(),
                    );
                }
            }
        }
    }
}
