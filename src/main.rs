//! Pong Tutorial

mod pong;
mod systems;
mod audio;

use crate::pong::Pong;

use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    audio::AudioBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(AudioBundle::default())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with(systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"]);
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;

    game.run();

    Ok(())
}
