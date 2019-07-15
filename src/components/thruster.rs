use amethyst::{
    assets::PrefabData,
    core::{
        math::{Unit, Vector3, UnitQuaternion, self as na, Isometry, Matrix4, Quaternion, RealField, Translation3, Rotation3},
        Transform,
    },
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
    /// Preserves current velocity
    pub velocity: Vector3<f32>,
    pub rotation_control: f32,
    pub thrust_control: f32,
    pub turn_speed: f32,
    pub traction: f32,
    pub max_speed: f32,
    pub min_speed: f32,
    pub start_speed: f32,
}

impl Default for Thruster {
    fn default() -> Self {
        Self {
            velocity: Vector3::zeros(),
            rotation_control: 0.0,
            thrust_control: 0.0,
            turn_speed: 1.0,
            traction: 0.05,
            max_speed: 10.0,
            min_speed: 0.0,
            start_speed: 0.0,
        }
    }
}

impl Component for Thruster {
    type Storage = DenseVecStorage<Self>;
}

impl Thruster {
    /// Followers
    pub fn speed_towards(&mut self, current_translation: &Vector3<f32>, target_translation: &Vector3<f32>) {
        let direction = Unit::new_normalize(target_translation - current_translation);
        self.velocity = direction.scale(self.max_speed);
    }
}
