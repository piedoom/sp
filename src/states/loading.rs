use crate::{
    resources::{prefabs::update_prefabs, GameResource, ResourceBundle},
    states::MainGameState,
};

use amethyst::{assets::ProgressCounter, prelude::*};

pub struct LoadingState {
    progress_counter: ProgressCounter,
}

impl Default for LoadingState {
    fn default() -> Self {
        LoadingState {
            progress_counter: ProgressCounter::new(),
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        ResourceBundle::initialize(data.world, &mut self.progress_counter);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        if self.progress_counter.is_complete() {
            self.progress_counter = ProgressCounter::new();
            update_prefabs(&mut data.world);
            return Trans::Switch(Box::new(MainGameState::new(data.world)));
        }
        Trans::None
    }
}
