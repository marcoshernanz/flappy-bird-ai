mod bird;
mod collisions;
mod config;
mod game;
mod neural_network;
mod obstacle;

use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("AI Flappy Bird"),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    show_mouse(false);

    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        next_frame().await
    }
}
