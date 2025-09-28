use macroquad::prelude::*;
use pong::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(BLACK);

        game.handle_input();
        game.update(get_frame_time());
        game.draw();

        // Handle quit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
