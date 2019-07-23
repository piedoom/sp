use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::{
            Vector3,
            Vector2,
            UnitQuaternion,
            Quaternion,
        },
        Time,
    },
    ecs::prelude::*,
};

#[derive(Debug)]
pub struct Body {
    velocity: Vector3<f32>,
    acceleration: Vector3<f32>,
    angular_velocity: f32,
    angular_acceleration: f32,
    max_angular_velocity: Option<f32>,
    max_velocity: Option<f32>,
}

impl Body {
    pub fn velocity(&self) -> Vector3<f32> {
        self.velocity
    }
    pub fn angular_velocity(&self) -> f32 {
        self.angular_velocity
    }
    pub fn acceleration(&self) -> Vector3<f32> {
        self.acceleration
    }
    pub fn angular_acceleration(&self) -> f32 {
        self.angular_acceleration
    }
    pub fn set_velocity(&mut self, velocity: Vector3<f32>) -> &Self {
        self.velocity = velocity;
        self
    }
    pub fn set_max_angular_velocity(&mut self, max_angular_velocity: f32) -> &Self {
        self.max_angular_velocity = Some(max_angular_velocity);
        self
    }
    /// This is useful for setting a hard max to prevent anything crazy. However, if you need
    /// to limit speed for gameplay reasons, it is best to do so in the consumer controller component & system
    pub fn set_max_velocity(&mut self, max_velocity: f32) -> &Self {
        self.max_velocity = Some(max_velocity);
        self
    }
    /// Extract coordinates and rotation from an associated transform to create the component
    pub fn new(transform: &Transform) -> Self {
        Self::default()
    }
    pub fn apply_force(&mut self, force: Vector3<f32>) -> &Self {
        self.acceleration += force;
        self
    }
    pub fn apply_torque(&mut self, force: f32) -> &Self {
        self.angular_acceleration += force;
        self
    }
    /// Update position and rotation according to deltatime. This is called in our `PhysicsSystem`
    pub fn update(&mut self, delta_seconds: f32) -> &Self {
        // Calculate our positional and rotational velocity based on the time step
        self.velocity = self.velocity + (self.acceleration * delta_seconds);
        self.angular_velocity = self.angular_velocity + (self.angular_acceleration * delta_seconds);
        self
    }
}

impl Default for Body {
    fn default() -> Self {
        Self {
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
            angular_velocity: 0f32,
            angular_acceleration: 0f32,
            max_angular_velocity: None,
            max_velocity: None,
        }
    }
}

impl Component for Body {
    type Storage = DenseVecStorage<Body>;
}
