use crate::game::GameState;
use crate::nerds::{self, Nerd, Nerds};
use console_engine::{Color, ConsoleEngine, KeyCode};
use euclid::{Point2D, UnknownUnit};
use std::process;

// Represents a point on the screen
type Point = Point2D<u32, UnknownUnit>;

// Console engine initialization
const MIN_WIDTH: u32 = 80;
const MIN_HEIGHT: u32 = 24;
const FPS: u32 = 60;

// Controls
const QUIT_KEY: KeyCode = KeyCode::Char('q');
const START_KEY: KeyCode = KeyCode::Enter;
const UP_KEY: KeyCode = KeyCode::Up;
const DOWN_KEY: KeyCode = KeyCode::Down;
const LEFT_KEY: KeyCode = KeyCode::Left;
const RIGHT_KEY: KeyCode = KeyCode::Right;

// Stuff displayed on the intro
const INTRO_TEXT: [&str; 2] = ["Let AC be Academic Challenge in:", "AC NERD DUELS"];
const INTRO_COLOR: Color = Color::Red;
const INTRO_TIME: u32 = 2;

// Stuff shown in the main menu and nerds
const LOGO_TEXT: &str = "  ___  _____  _   _______
 / _ \\/  __ \\| \\ | |  _  \\
/ /_\\ \\ /  \\/|  \\| | | | |
|  _  | |    | . ` | | | |
| | | | \\__/\\| |\\  | |/ /
\\_| |_/\\____/\\_| \\_/___/";
const LOGO_COLOR: Color = Color::Blue;
const QUIT_TEXT: &str = "Press 'q' to quit at any time";
const NERDS: [&Nerd; 4] = [&nerds::JOE, &nerds::ISAAC, &nerds::WILLIAM, &nerds::SUZIE];
const SELECT_TEXT: [&str; 2] = ["Nerd 1: ", "Nerd 2: "];
const SELECT_COLOR: Color = Color::Magenta;
const START_TEXT: &str = "Press the enter/return key to start the game or skip the intro";

// Manages the terminal, and whats displayed and inputted
pub struct Tui {
	engine: ConsoleEngine,
	width: u32,
	height: u32,
	current_selection: usize,
	selects: [usize; 2],
}

impl Tui {
	// Creates a new TUI
	pub fn new() -> Self {
		let engine =
			ConsoleEngine::init_fill_require(MIN_WIDTH, MIN_HEIGHT, FPS).unwrap_or_else(|err| {
				eprintln!("Error starting ConsoleEngine: {}", err);
				process::exit(1);
			});
		let width = engine.get_width();
		let height = engine.get_height();
		Self {
			engine,
			width,
			height,
			current_selection: 0,
			selects: [0, 0],
		}
	}

	// Updates the TUI
	pub fn update(&mut self, game_state: GameState, nerds: &Nerds) {
		self.draw(game_state, nerds);
		self.engine.draw();
		self.engine.clear_screen();
		self.engine.wait_frame();
	}

	// Returns whether the player wants to quit
	pub fn quit(&self) -> bool {
		self.engine.is_key_pressed(QUIT_KEY)
	}

	// Returns whether the intro is done
	pub fn intro_done(&self) -> bool {
		self.engine.frame_count as u32 / FPS >= INTRO_TIME * 2
			|| self.engine.is_key_pressed(START_KEY)
	}

	// Returns whether the game has started
	pub fn game_start(&self) -> bool {
		self.engine.is_key_pressed(START_KEY)
	}

	// Returns the nerds selected in the main menu
	pub fn selected_nerds(&self) -> Nerds {
		Some([*NERDS[self.selects[0]], *NERDS[self.selects[1]]])
	}

	// Draws everything related to the current game state
	fn draw(&mut self, game_state: GameState, nerds: &Nerds) {
		match game_state {
			GameState::Intro => self.draw_intro(),
			GameState::MainMenu => self.draw_menu(),
			GameState::InGame => self.draw_game(nerds),
			GameState::GameEnd => self.draw_end(),
		}
	}

	// Draws the intro
	fn draw_intro(&mut self) {
		let pos = |text: &str, pos| {
			Point::new(
				self.width / 2 - text.len() as u32 / 2,
				((self.height / 2) as i32 + pos) as u32,
			)
		};
		let intro_pos = [pos(INTRO_TEXT[0], -1), pos(INTRO_TEXT[1], 0)];
		self.engine
			.print(intro_pos[0].x as i32, intro_pos[0].y as i32, INTRO_TEXT[0]);
		if self.engine.frame_count as u32 / FPS >= INTRO_TIME {
			self.engine.print_fbg(
				intro_pos[1].x as i32,
				intro_pos[1].y as i32,
				INTRO_TEXT[1],
				INTRO_COLOR,
				Color::Reset,
			);
		}
	}

	// Draws the main menu
	fn draw_menu(&mut self) {
		self.draw_logo();
		[(QUIT_TEXT, -2), (START_TEXT, 3)].map(|message| self.draw_message(message.0, message.1));
		[0, 1].map(|selection| {
			self.draw_select(
				&[SELECT_TEXT[selection], NERDS[self.selects[selection]].name].concat(),
				selection as i32,
				selection == self.current_selection,
			)
		});
		self.menu_input();
	}

	// Draws the logo in the main menu
	fn draw_logo(&mut self) {
		let logo_pos = Point::new(
			self.width / 2 - LOGO_TEXT.lines().next().unwrap().len() as u32 / 2,
			self.height / 2 - 10,
		);
		self.engine.print_fbg(
			logo_pos.x as i32,
			logo_pos.y as i32,
			LOGO_TEXT,
			LOGO_COLOR,
			Color::Reset,
		);
	}

	// Draws either the quit or start message
	fn draw_message(&mut self, text: &str, pos: i32) {
		let pos = Point::new(
			self.width / 2 - text.len() as u32 / 2,
			((self.height / 2) as i32 + pos) as u32,
		);
		self.engine.print(pos.x as i32, pos.y as i32, text);
	}

	// Draws a selectable option in the main menu
	fn draw_select(&mut self, select_text: &str, pos: i32, selected: bool) {
		let select_pos = Point::new(
			self.width / 2 - select_text.len() as u32 / 2,
			((self.height / 2) as i32 + pos) as u32,
		);
		self.engine.print_fbg(
			select_pos.x as i32,
			select_pos.y as i32,
			select_text,
			if selected { SELECT_COLOR } else { Color::Reset },
			Color::Reset,
		);
	}

	// Manages input in the main menu
	fn menu_input(&mut self) {
		if self.engine.is_key_pressed(UP_KEY) {
			Self::change_selected(&mut self.current_selection, 1, 1);
		} else if self.engine.is_key_pressed(DOWN_KEY) {
			Self::change_selected(&mut self.current_selection, 1, -1);
		} else if self.engine.is_key_pressed(LEFT_KEY) {
			Self::change_selected(&mut self.selects[self.current_selection], 3, -1);
		} else if self.engine.is_key_pressed(RIGHT_KEY) {
			Self::change_selected(&mut self.selects[self.current_selection], 3, 1);
		}
	}

	// Changes the value of the currently selected main menu option
	fn change_selected(value: &mut usize, max: usize, pos: i32) {
		if *value == 0 && pos == -1 {
			*value = max;
		} else if *value == max && pos == 1 {
			*value = 0;
		} else {
			let n = *value as i32 + pos;
			*value = n as usize;
		}
	}

	// Draws the game
	fn draw_game(&mut self, nerds: &Nerds) {
		todo!();
	}

	// Draws the end screen
	fn draw_end(&mut self) {
		todo!();
	}
}
