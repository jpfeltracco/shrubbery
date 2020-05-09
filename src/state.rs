use amethyst::{
    core::transform::Transform,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
    renderer::{palette::Srgba, Camera},
    window::ScreenDimensions,
};

pub struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        init_lines(world);
        init_camera(world, &dimensions);
    }
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(dimensions.width() / 2., dimensions.height() / 2., 10.0);
    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(local_transform)
        .build();
}

fn init_lines(world: &mut World) {
    // Setup debug lines as a resource
    world.insert(DebugLines::new());
    // Configure width of lines. Optional step
    world.insert(DebugLinesParams { line_width: 2.0 });

    // Setup debug lines as a component and add lines to render axis&grid
    let mut debug_lines_component = DebugLinesComponent::new();

    let (screen_w, screen_h) = {
        let screen_dimensions = world.read_resource::<ScreenDimensions>();
        (screen_dimensions.width(), screen_dimensions.height())
    };

    for y in (0..(screen_h as u16)).step_by(50).map(f32::from) {
        debug_lines_component.add_line(
            [0.0, y, 1.0].into(),
            [screen_w, (y + 2.0), 1.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 1.0),
        );
    }

    for x in (0..(screen_w as u16)).step_by(50).map(f32::from) {
        debug_lines_component.add_line(
            [x, 0.0, 1.0].into(),
            [x, screen_h, 1.0].into(),
            Srgba::new(0.3, 0.3, 0.3, 1.0),
        );
    }

    world.create_entity().with(debug_lines_component).build();
}
