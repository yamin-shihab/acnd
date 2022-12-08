mod game;
mod nerds;
mod tui;

use crate::game::Game;

// Entry point of program
fn main() {
	let mut game = Game::new();
	game.main_loop();
}
