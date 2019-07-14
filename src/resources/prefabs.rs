use std::collections::HashMap;
use std::fs::read_dir;

use crate::components::characters::CharacterPrefabData;
use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::World,
    utils::application_root_dir,
};


#[derive(Default)]
pub struct CharacterPrefabs {
    prefabs: HashMap<String, Handle<Prefab<CharacterPrefabData>>>,
}

impl CharacterPrefabs {
    pub fn insert(
        &mut self,
        character_name: String,
        prefab_handle: Handle<Prefab<CharacterPrefabData>>,
    ) {
        self.prefabs.insert(character_name, prefab_handle);
    }

    pub fn get_prefab(&self, name: &str) -> Option<&Handle<Prefab<CharacterPrefabData>>> {
        self.prefabs.get(name)
    }

    pub fn get_prefabs(&self) -> &HashMap<String, Handle<Prefab<CharacterPrefabData>>> {
        &self.prefabs
    }

    pub fn set_prefabs(&mut self, prefabs: HashMap<String, Handle<Prefab<CharacterPrefabData>>>) {
        self.prefabs = prefabs;
    }
}

fn make_name(subdirectory: &str, entry: &std::fs::DirEntry) -> String {
    let path_buffer = entry.path();
    let filename = path_buffer.file_name().unwrap();
    format!("{}{}", subdirectory, filename.to_str().unwrap())
}

pub fn initialize_prefabs(world: &mut World) -> ProgressCounter {
    let mut progress_counter = ProgressCounter::new();
    // load character prefabs
    {
        let prefab_iter = {
            let prefab_dir_path = application_root_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
                + "/resources/prefabs/characters";
            let prefab_iter = read_dir(prefab_dir_path).unwrap();
            prefab_iter.map(|prefab_dir_entry| {
                world.exec(|loader: PrefabLoader<'_, CharacterPrefabData>| {
                    loader.load(
                        make_name("prefabs/characters/", &prefab_dir_entry.unwrap()),
                        RonFormat,
                        &mut progress_counter,
                    )
                })
            })
        };

        let mut character_prefabs = CharacterPrefabs::default();
        for (count, prefab) in prefab_iter.enumerate() {
            character_prefabs.insert("temp_prefab_".to_string() + &count.to_string(), prefab);
        }
        world.add_resource(character_prefabs);
    }

    progress_counter
}

// Once the prefabs are loaded, this function is called to update the ekeys in the CharacterPrefabs struct.
// We use the Named component of the entity to determine which key to use.
pub fn update_prefabs(world: &mut World) {
    let updated_prefabs = {
        let character_prefabs = world.read_resource::<CharacterPrefabs>();
        let prefabs = character_prefabs.get_prefabs();
        let mut prefab_resource =
            world.write_resource::<AssetStorage<Prefab<CharacterPrefabData>>>();
        let mut new_prefabs = HashMap::new();
        for (_key, handle) in prefabs.iter() {
            if let Some(prefab) = prefab_resource.get_mut(handle) {
                if let Some(prefab_data) = prefab.entity(0) {
                    let name = prefab_data
                        .data()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap()
                        .name
                        .to_string();
                    new_prefabs.insert(name, handle.clone());
                }
            }
        }
        new_prefabs
    };
    world
        .write_resource::<CharacterPrefabs>()
        .set_prefabs(updated_prefabs);
}