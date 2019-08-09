use amethyst::{
    assets::{AssetPrefab, PrefabData, ProgressCounter},
    core::Named,
    derive::PrefabData,
    ecs::prelude::*,
    ecs::world::LazyUpdate,
    gltf::{GltfSceneAsset, GltfSceneFormat},
    Error,
};

use crate::components::*;
use serde::{Deserialize, Serialize};

// This is the main prefab data for characters.
#[derive(Default, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CharacterPrefabData {
    pub name: Option<Named>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
    thruster: Option<Thruster>,
    settings: Option<CharacterData>,
}

pub enum Character {
    Quartz,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Default, Deserialize, Serialize, PrefabData)]
#[serde(default, deny_unknown_fields)]
#[prefab(Component)]
pub struct CharacterData {
    /// The speed at which projectiles from a basic attack move
    pub basic_attack_speed: f32,
    /// The range at which the projectile dies
    pub basic_attack_range: f32,
    /// Whether or not the character wants to attack
    pub attack: bool,
    /// The timer for determining basic attack recoil time
    pub basic_attack_timer: timer::Timer,
}

/// A CharacterData component with some fields omitted for easier construction in a prefab
struct CharacterDataPrefab {
    basic_attack_speed: f32,
    basic_attack_range: f32,
    /// The amount of milliseconds before the character can fire again
    basic_attack_recoil: u64,
}

impl From<CharacterDataPrefab> for CharacterData {
    fn from(c: CharacterDataPrefab) -> CharacterData {
        CharacterData {
            basic_attack_speed: c.basic_attack_speed,
            basic_attack_range: c.basic_attack_range,
            attack: false,
            basic_attack_timer: timer::Timer::new(0, c.basic_attack_recoil),
        }
    }
}


impl Component for CharacterData {
    type Storage = DenseVecStorage<Self>;
}

