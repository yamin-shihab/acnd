use crate::nerds::{Nerd, Nerds};
use crate::tui::Tui;

// String used at beginning of game to introduce players
const GAME_START_MESSAGE: &str = "Two nerds bump into each other. nerd0 and nerd1 glare at each other. The fight chant is heard. The AC Nerd Duels have begun.";
const GAME_END_MESSAGE: &str = "As the dust settles, nerd0 looks down at the unconcious nerd1 before being escorted to the principal's office.";

// Represents the possible states the game can be in
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
	Intro,
	MainMenu,
	InGame,
	GameEnd,
}

// Contains game information
pub struct Game {
	tui: Tui,
	game_state: GameState,
	nerds: Nerds,
	current_player: usize,
}

impl Game {
	// Creates new instance of the game
	pub fn new() -> Self {
		Self {
			tui: Tui::new(),
			game_state: GameState::Intro,
			nerds: None,
			current_player: 0,
		}
	}

	// Runs every frame
	pub fn main_loop(&mut self) {
		loop {
			self.tui.update(self.game_state, &self.nerds);
			if self.tui.should_quit() {
				break;
			}
			self.update();
		}
	}

	// Updates the game
	fn update(&mut self) {
		match self.game_state {
			GameState::Intro => {
				if self.tui.intro_done() {
					self.game_state = GameState::MainMenu
				}
			}
			GameState::MainMenu => {
				if self.tui.game_started() {
					self.start_game()
				}
			}
			GameState::InGame => {
				if let Some(nerd) = self.game_ended() {
					self.end_game(nerd)
				}
			}
			GameState::GameEnd => (),
		}
	}

	// Initializes the start of the game
	fn start_game(&mut self) {
		self.game_state = GameState::InGame;
		self.nerds = self.tui.selected_nerds();
		let nerds = self.nerds.unwrap();
		self.tui.add_action_message(
			&GAME_START_MESSAGE
				.replace("nerd0", nerds[0].name)
				.replace("nerd1", nerds[1].name),
		);
	}

	// Returns the winning nerd, otherwise none
	fn game_ended(&self) -> Option<&'static str> {
		let nerds = self.nerds.unwrap();
		for nerd in nerds {
			if nerd.health < 1 {
				return Some(nerd.name);
			}
		}
		None
	}

	// Ends the game
	fn end_game(&mut self, nerd: &str) {
		self.game_state = GameState::GameEnd;
		let nerds = self.nerds.unwrap();
		let loser = if nerds[0].name == nerd {
			&nerds[1].name
		} else {
			&nerds[0].name
		};
		self.tui.add_action_message(
			&GAME_END_MESSAGE
				.replace("nerd0", nerd)
				.replace("nerd1", loser),
		);
	}
}
