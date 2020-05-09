use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

/// This system is responsible for moving all the paddles according to the user
/// provided input.
#[derive(SystemDesc)]
pub struct CameraMovementSystem;

impl CameraMovementSystem {
    pub fn new() -> Self {
        Self
    }
}

const CAMERA_SPEED: f32 = 1000.0;

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut transforms, input, time): Self::SystemData) {
        // let t = (time.absolute_time_seconds() as f32).cos() / 2.0 + 0.5;
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            let camera_x = input.axis_value("camera_x").unwrap();
            let camera_y = input.axis_value("camera_y").unwrap();

            let dt = time.delta_seconds();

            transform.prepend_translation_x(camera_x * CAMERA_SPEED * dt);
            transform.prepend_translation_y(camera_y * CAMERA_SPEED * dt);
        }
    }
}
