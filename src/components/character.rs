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
}

pub enum Character {
    Quartz,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

pub struct CharacterState {
    pub attack: bool,
}

impl Component for CharacterState {
    type Storage = DenseVecStorage<Self>;
}
