use amethyst::{
    assets::{PrefabData, ProgressCounter, AssetPrefab},
    core::Named,
    derive::PrefabData,
    ecs::Entity,
    Error,
    gltf::{GltfSceneAsset, GltfSceneFormat},
};

use serde::{Deserialize, Serialize};

// This is the main prefab data for characters.
#[derive(Default, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CharacterPrefabData {
    pub name: Option<Named>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
}