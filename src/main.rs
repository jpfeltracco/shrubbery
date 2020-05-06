use raylib::prelude::*;

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
        d.draw_text("Hello, shrubbery!", x, y, 20, Color::BLACK);
    }
}
