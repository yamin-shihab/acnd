mod game;
mod nerds;
mod tui;

use crate::game::Game;

fn main() {
	let mut game = Game::new();
	game.main_loop();
}
