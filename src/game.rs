use crate::nerds::{self, Nerd};
use crate::tui::Tui;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
	Intro,
	MainMenu,
	InGame,
	GameEnd,
}

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
			game_state: GameState::Intro,
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

	fn update(&mut self) {
		if self.game_state == GameState::Intro && self.tui.intro_done() {
			self.game_state = GameState::MainMenu;
		} else if self.game_state == GameState::MainMenu && self.tui.game_start() {
			self.game_state = GameState::InGame;
			self.nerds = Some(self.tui.selected_nerds());
		}
	}
}
