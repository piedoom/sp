use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

/// Thrusters are basic physics controllers that simulate a constant force on a body in any direction
#[derive(Clone, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[prefab(Component)]
pub struct Thruster {
    /// A anormalized value that controls the rotation
    pub rotation_control: f32,
    /// A normalized value that controls the acceleration
    pub thrust_control: f32,
    /// Scalar value that modifies over what time to apply an angular force
    pub rotational_force: f32,
    /// Scalar value that modifies over what time to apply a force
    pub thrust_force: f32,
    /// Limits the maximum magnitude of this thruster
    pub max_velocity: Option<f32>,
}

impl Default for Thruster {
    fn default() -> Self {
        Self {
            rotation_control: 0f32,
            thrust_control: 0f32,
            rotational_force: 0f32,
            thrust_force: 1.0f32,
            max_velocity: None,
        }
    }
}

impl Component for Thruster {
    type Storage = DenseVecStorage<Self>;
}

