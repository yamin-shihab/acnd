use crate::game::GameState;
use crate::nerds::Nerd;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use std::process;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;
const FPS: u32 = 60;

pub struct Tui {
	engine: ConsoleEngine,
}

impl Tui {
	pub fn new() -> Self {
		Self {
			engine: ConsoleEngine::init_fill_require(WIDTH, HEIGHT, FPS).unwrap_or_else(|err| {
				eprintln!("Error starting ConsoleEngine: {}", err);
				process::exit(1);
			}),
		}
	}

	pub fn update(&mut self, game_state: GameState, nerds: &Option<[Nerd; 2]>) {
		self.draw(game_state, nerds);
		self.engine.draw();
		self.engine.clear_screen();
		self.engine.wait_frame();
	}

	pub fn quit(&self) -> bool {
		self.engine.is_key_pressed(KeyCode::Char('q'))
	}

	fn draw(&mut self, game_state: GameState, nerds: &Option<[Nerd; 2]>) {}
}
