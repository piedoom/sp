use crate::{
    resources::prefabs::{initialize_prefabs, update_prefabs},
    states::MainGameState,
};

use amethyst::{assets::ProgressCounter, prelude::*};

pub struct LoadingState {
    prefab_loading_progress: Option<ProgressCounter>,
}

impl Default for LoadingState {
    fn default() -> Self {
        LoadingState {
            prefab_loading_progress: None,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.prefab_loading_progress = Some(initialize_prefabs(&mut data.world));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        if let Some(ref counter) = self.prefab_loading_progress.as_ref() {
            if counter.is_complete() {
                self.prefab_loading_progress = None;
                update_prefabs(&mut data.world);
                return Trans::Switch(Box::new(MainGameState::new(data.world)));
            }
        }
        Trans::None
    }
}