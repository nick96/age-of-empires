mod pong;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");

    let display_config_path = config_dir.join("display.ron");
    let binding_config_path = config_dir.join("bindings.ron");

    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)
                .with_clear([0.34, 0.36, 0.52, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());
    let transform_bundle = TransformBundle::new();
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[]);

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}
