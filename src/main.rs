#![allow(dead_code)]

use macroquad::prelude::*;

const MAP: usize = 20;
const T_SIZE: (f32, f32) = (32., 16.); // tile size

enum AppState {
    Menu,
    Playing,
    GameOver,
}

struct Game {}

impl Game {
    fn new() -> Self {
        Game {}
    }

    fn update(&mut self, _dt: f32) -> bool {
        // fake game logic
        if is_key_pressed(KeyCode::Space) {
            return true;
        }
        false
    }

    fn draw(&self) {
        // draw the game
        draw_text("Game running...", 20., 40., 30., BLACK);
        draw_text("Press Space to die...", 20., 80., 30., DARKGRAY);
    }
}

#[macroquad::main("Crablo")]
async fn main() {}
