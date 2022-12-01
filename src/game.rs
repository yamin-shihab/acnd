use crate::nerds::{self, Nerd};
use crate::tui::Tui;

pub struct Game {
	tui: Tui,
	game_state: GameState,
	nerds: Option<[Nerd; 2]>,
	current_player: usize,
}

impl Game {
	pub fn new() -> Self {
		Self {
			tui: Tui::new(),
			game_state: GameState::MainMenu,
			nerds: None,
			current_player: 0,
		}
	}

	pub fn main_loop(&mut self) {
		loop {
			self.tui.update(self.game_state, &self.nerds);
			if self.tui.quit() {
				break;
			}
			self.update();
		}
	}

	fn update(&mut self) {}
}

#[derive(Copy, Clone)]
pub enum GameState {
	Intro,
	MainMenu,
	InGame,
	GameEnd,
}
