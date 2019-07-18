use amethyst;

use amethyst::{
    assets::{Handle, Prefab},
    core::{
        transform::Transform,
    },
    ecs::*,
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
    },
    window::ScreenDimensions,
};

use crate::resources::prefabs::CharacterPrefabs;
use crate::components::*;
use crate::systems::*;

pub struct MainGameState {
    dispatcher: Dispatcher<'static, 'static>,
    camera: Option<Entity>,
}

impl MainGameState {
    pub fn new(_world: &mut World) -> Self {
        MainGameState {
            dispatcher: DispatcherBuilder::new()
            .with(InputSystem::default(), "input_system", &[])
            .with(ThrusterSystem::default(), "thruster_system", &[])
            .with(TrackerSystem::default(), "tracker_system", &[])
            .build(),
            camera: None,
        }
    }
}

impl SimpleState for MainGameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        self.dispatcher.setup(&mut data.world.res);

        // Setup camera
        let (width, height) = {
            let dim = data.world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let character_prefab = get_character_prefab(data.world, "default");
        let character = data.world.create_entity()
            .with(character_prefab.clone())
            .with(Transform::default())
            .with(Player::default())
            .build();

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 30.0, 0.0);
        transform.set_rotation_euler(-1.5707963, 0.0, 0.0);
        self.camera = Some(
            data.world
                .create_entity()
                .named("Main camera")
                .with(Camera::from(Projection::perspective(
                    width / height,
                    std::f32::consts::FRAC_PI_2,
                    0.01f32,
                    1000.0f32,
                )))
                .with(transform)
                .with(Tracker::new(character))
                .build(),
        );
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world.res);
        data.data.update(&data.world);
        Trans::None
    }
}

pub fn get_character_prefab(world: &mut World, key: &str) -> Handle<Prefab<crate::components::characters::CharacterPrefabData>> {
    world.exec(|prefab_store: ReadExpect<CharacterPrefabs>| {
        prefab_store
            .get_prefab(key)
            .expect(&format!("Getting prefab with key {} failed.", key))
            .clone()
    })
}
