mod game;
mod nerds;
mod tui;

use crate::game::Game;

// First entry point of the game
fn main() {
    let mut game = Game::new();
    game.main_loop();
}
