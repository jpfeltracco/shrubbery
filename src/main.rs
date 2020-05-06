use raylib::prelude::*;

const PLAYER_HEIGHT: i32 = 40;
const PLAYER_WIDTH: i32 = 10;
const PLAYER_HEAD_RADIUS: i32 = 10;
const PLAYER_HEAD_FUDGE: i32 = 5;

fn main() {
    use raylib::consts::KeyboardKey::*;

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let mut x: i32 = 50;
    let mut y: i32 = 50;

    while !rl.window_should_close() {
        if rl.is_key_down(KEY_A) {
            x -= 1;
        } else if rl.is_key_down(KEY_D) {
            x += 1;
        }
        if rl.is_key_down(KEY_W) {
            y -= 1;
        } else if rl.is_key_down(KEY_S) {
            y += 1;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_rectangle(
            x - PLAYER_WIDTH / 2,
            y - PLAYER_HEIGHT / 2,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            Color::VIOLET,
        );
        d.draw_circle(
            x,
            y - PLAYER_HEIGHT / 2 - PLAYER_HEAD_RADIUS + PLAYER_HEAD_FUDGE,
            PLAYER_HEAD_RADIUS as f32,
            Color::LIME,
        );
    }
}
