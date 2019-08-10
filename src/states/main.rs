use amethyst;

use amethyst::{
    assets::{Handle, Prefab},
    core::*,
    ecs::*,
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        light::{Light, PointLight},
    },
    window::ScreenDimensions,
};

use crate::components::*;
use crate::resources::prefabs::CharacterPrefabs;
use crate::systems::*;

use specs_physics::{
    colliders::Shape,
    nalgebra::Vector3,
    nphysics::{algebra::Velocity3, object::BodyStatus},
    parameters::Gravity,
    register_physics_systems, PhysicsBody, PhysicsBodyBuilder, PhysicsColliderBuilder,
};
pub struct MainGameState {
    dispatcher: Dispatcher<'static, 'static>,
    camera: Option<Entity>,
}

impl MainGameState {
    pub fn new(_world: &mut World) -> Self {
        MainGameState {
            dispatcher: {
                let mut builder = DispatcherBuilder::new()
                    .with(InputSystem::default(), "input_system", &[])
                    .with(TrackerSystem::default(), "tracker_system", &[])
                    .with(ThrusterSystem::default(), "thruster_system", &[])
                    .with(CharacterSystem::default(), "character_system", &[])
                    .with(LifetimeSystem::default(), "lifetime_system", &[]);
                register_physics_systems::<f32, Transform>(&mut builder);
                builder.build()
            },
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

        // register components that do not have systems
        data.world.register::<Character>();

        let character = build_character(data.world, "quartz")
            .with(Player::default())
            .build();
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 15.0, 0.0);
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
                .with(transform.clone())
                //.with(Tracker::new(character))
                .build(),
        );
        // add a light
        let light: Light = PointLight {
            intensity: 6.0,
            ..PointLight::default()
        }
        .into();
        transform.set_translation_xyz(0.0, 10.0, 0.0);
        data.world
            .create_entity()
            .with(light)
            .with(transform.clone())
            .build();
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world.res);
        data.data.update(&data.world);
        Trans::None
    }
}

pub fn get_character_prefab(
    world: &mut World,
    key: &str,
) -> Handle<Prefab<crate::components::CharacterPrefab>> {
    world.exec(|prefab_store: ReadExpect<CharacterPrefabs>| {
        prefab_store
            .get_prefab(key)
            .expect(&format!("Getting prefab with key {} failed.", key))
            .clone()
    })
}

pub fn build_character<'a>(world: &'a mut World, key: &str) -> EntityBuilder<'a> {
    let character_prefab = get_character_prefab(world, key);
    world
        .create_entity()
        .with(character_prefab.clone())
        .with(Transform::default())
        .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic).build())
        .with(PhysicsColliderBuilder::<f32>::from(Shape::<f32>::Ball { radius: 1f32 }).build())
        .with(get_character_component(key))
}

pub fn get_character_component(key: &str) -> impl Component {
    match key {
        "quartz" => Character::Quartz,
        _ => unreachable!(),
    }
}
