use raylib::prelude::*;

mod thing;
use thing::Thing;

const PLAYER_HEIGHT: f32 = 40.0;
const PLAYER_WIDTH: f32 = 10.0;
const PLAYER_HEAD_RADIUS: f32 = 10.0;
const PLAYER_HEAD_FUDGE: f32 = 20.0;
const PLAYER_SPEED: f32 = 150.0;

fn main() {
    use raylib::consts::KeyboardKey::*;

    let (mut rl, thread) = raylib::init().size(640, 480).title("Shrubbery").build();

    let x: f32 = 50.0;
    let y: f32 = 50.0;
    let player = Rectangle::new(x, y, PLAYER_WIDTH, PLAYER_HEIGHT);
    let mut player_head_center = Vector2::new(
        x,
        y - PLAYER_HEIGHT / 2.0 - PLAYER_HEAD_RADIUS + PLAYER_HEAD_FUDGE,
    );

    let mut delta_time;

    let mut player_thing = Thing {
        position: Vector2::new(x, y),
        velocity: Vector2::new(0.0, 0.0),
        bounding_box: player,
        color: Color::VIOLET,
    };

    let mut camera = Camera2D {
        offset: vec2(320.0, 240.0),
        target: vec2(0., 0.),
        rotation: 0.0,
        zoom: 1.0,
    };

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        delta_time = rl.get_frame_time();

        if rl.is_key_down(KEY_A) {
            player_thing.position.x -= PLAYER_SPEED * delta_time;
        } else if rl.is_key_down(KEY_D) {
            player_thing.position.x += PLAYER_SPEED * delta_time;
        }
        if rl.is_key_down(KEY_W) {
            player_thing.position.y -= PLAYER_SPEED * delta_time;
        } else if rl.is_key_down(KEY_S) {
            player_thing.position.y += PLAYER_SPEED * delta_time;
        }

        camera.target.x += 1.0;
        camera.target.y += 1.0;

        let mut d = rl.begin_drawing(&thread);

        let mut draw = d.begin_mode_2D(camera);

        draw.clear_background(Color::WHITE);
        draw.draw_circle(
            player_thing.position.x as i32,
            player_thing.position.y as i32,
            10.0,
            Color::VIOLET,
        );
    }
}
