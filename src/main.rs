use amethyst::assets::PrefabLoaderSystem;
use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy,
    core::transform::TransformBundle,
    gltf::GltfSceneLoaderSystem,
    input::InputBundle,
    prelude::*,
    renderer::{types::DefaultBackend, visibility::VisibilitySortingSystem, RenderingSystem},
    utils::application_root_dir,
    window::WindowBundle,
};

mod components;
mod rendering;
mod resources;
mod states;
mod systems;
mod util;

use components as c;
use rendering::RenderGraph;
use util::GameBindings;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources = application_root_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/resources";
    let display_config_path = resources.clone() + "/display_config.ron";
    let key_bindings_path = resources.clone() + "/input.ron";

    // The global game data. Here we register all systems and bundles that will run for every game state. The game states
    // will define additional dispatchers for state specific systems. Note that the dispatchers will run in sequence,
    // so this setup sacrifices performance for modularity (for now).
    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(
            InputBundle::<GameBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(TransformBundle::new())?
        .with(
            PrefabLoaderSystem::<c::CharacterPrefabData>::default(),
            "character_prefab_loader",
            &[],
        )
        .with(
            VisibilitySortingSystem::new(),
            "visibility_system",
            &["transform_system"],
        )
        .with(
            GltfSceneLoaderSystem::default(),
            "gltf_loader",
            &["character_prefab_loader"], // This is important so that entity instantiation is performed in a single frame.
        )
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ));

    // Set up the core application.
    let mut game: Application<GameData> =
        CoreApplication::build(resources, crate::states::LoadingState::default())?
            .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
            .build(game_data)?;
    game.run();
    Ok(())
}
