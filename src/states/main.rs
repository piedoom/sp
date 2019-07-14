use amethyst;

use amethyst::{
    core::{transform::Transform},
    ecs::*,
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
    },
    window::ScreenDimensions,
};

pub struct MainGameState {
    dispatcher: Dispatcher<'static, 'static>,
    camera: Option<Entity>,
}

impl MainGameState {
    pub fn new(_world: &mut World) -> Self {
        MainGameState {
            dispatcher: DispatcherBuilder::new().build(),
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

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, 12.0);
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
                .build(),
        );
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world.res);
        data.data.update(&data.world);
        Trans::None
    }
}