use amethyst::{
    core::{transform::TransformBundle, Time},
    derive::SystemDesc,
    ecs::{Read, ReadExpect, System, SystemData, Write},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        debug_drawing::DebugLines,
        palette::Srgba,
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    window::ScreenDimensions,
};

mod input;
mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with(ExampleLinesSystem::new(), "example_lines_system", &[])
        .with(
            input::CameraMovementSystem::new(),
            "camera_movement_system",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?;

    let mut game = Application::new(assets_dir, state::MyState, game_data)?;
    game.run();

    Ok(())
}

#[derive(SystemDesc)]
struct ExampleLinesSystem;

impl ExampleLinesSystem {
    pub fn new() -> Self {
        Self
    }
}

impl<'s> System<'s> for ExampleLinesSystem {
    type SystemData = (
        ReadExpect<'s, ScreenDimensions>,
        Write<'s, DebugLines>,
        Read<'s, Time>,
    );

    fn run(&mut self, (screen_dimensions, mut debug_lines_resource, time): Self::SystemData) {
        let t = (time.absolute_time_seconds() as f32).cos() / 2.0 + 0.5;

        let screen_w = screen_dimensions.width();
        let screen_h = screen_dimensions.height();
        let y = t * screen_h;

        debug_lines_resource.draw_line(
            [0.0, y, 1.0].into(),
            [screen_w, y + 2.0, 1.0].into(),
            Srgba::new(0.3, 0.3, 1.0, 1.0),
        );
    }
}
