use raylib::prelude::*;

const PLAYER_HEIGHT: f32 = 40.0;
const PLAYER_WIDTH: f32 = 10.0;
const PLAYER_HEAD_RADIUS: f32 = 10.0;
const PLAYER_HEAD_FUDGE: f32 = 20.0;
const PLAYER_SPEED: f32 = 150.0;

fn main() {
    use raylib::consts::KeyboardKey::*;

    let (mut rl, thread) = raylib::init().size(640, 480).title("Shrubbery").build();

    let mut x: f32 = 50.0;
    let mut y: f32 = 50.0;
    let mut player = Rectangle::new(x, y, PLAYER_WIDTH, PLAYER_HEIGHT);
    let mut player_head_center = Vector2::new(x, y - PLAYER_HEIGHT / 2.0 - PLAYER_HEAD_RADIUS + PLAYER_HEAD_FUDGE);

    let mut delta_time = rl.get_frame_time();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        delta_time = rl.get_frame_time();

        if rl.is_key_down(KEY_A) {
            player.x -= PLAYER_SPEED * delta_time;
        } else if rl.is_key_down(KEY_D) {
            player.x += PLAYER_SPEED * delta_time;
        }
        if rl.is_key_down(KEY_W) {
            player.y -= PLAYER_SPEED * delta_time;
        } else if rl.is_key_down(KEY_S) {
            player.y += PLAYER_SPEED * delta_time;
        }

        player_head_center.x = player.x;
        player_head_center.y = player.y - PLAYER_HEIGHT / 2.0 - PLAYER_HEAD_RADIUS + PLAYER_HEAD_FUDGE;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_rectangle_rec(&player, Color::VIOLET);
        d.draw_circle_v(
            player_head_center,
            PLAYER_HEAD_RADIUS as f32,
            Color::LIME,
        );
    }
}
