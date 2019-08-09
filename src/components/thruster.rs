use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};

/// Thrusters are basic physics controllers that simulate a constant force on a body in any direction
#[derive(Clone, Deserialize, Serialize, PrefabData)]
#[serde(default, deny_unknown_fields)]
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
    pub max_speed: Option<f32>,
    pub max_angular_speed: Option<f32>,
}

/// A Thruster component with some fields omitted for easier construction in a prefab
#[derive(Clone, Copy, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThrusterPrefab {
    torque: f32,
    thrust: f32,
    speed: Option<f32>,
    angular_speed: Option<f32>,
}

impl From<ThrusterPrefab> for Thruster {
    fn from(p: ThrusterPrefab) -> Thruster {
        Thruster {
            rotational_force: p.torque,
            thrust_force: p.thrust,
            max_speed: p.speed,
            max_angular_speed: p.angular_speed,
            ..Default::default()
        }
    }
}

impl<'a> PrefabData<'a> for ThrusterPrefab {
    type SystemData = WriteStorage<'a, Thruster>;
    type Result = ();
    fn add_to_entity(
        &self,
        entity: Entity,
        thrusters: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<(), Error> {
        let thruster = Into::<Thruster>::into(*self);
        thrusters.insert(entity, thruster).map(|_| ())?;
        Ok(())
    }
}

impl Default for Thruster {
    fn default() -> Self {
        Self {
            rotation_control: 0f32,
            thrust_control: 0f32,
            rotational_force: 200.0f32,
            thrust_force: 400.0f32,
            max_speed: Some(10f32),
            max_angular_speed: Some(5f32),
        }
    }
}

impl Component for Thruster {
    type Storage = DenseVecStorage<Self>;
}
