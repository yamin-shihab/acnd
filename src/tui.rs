use crate::game::GameState;
use crate::nerds::{Nerds, NERDS, NERD_COLOR};
use console_engine::{Color, ConsoleEngine, KeyCode};
use euclid::{Point2D, UnknownUnit};
use std::process;

// Represents a point on the screen
type Point = Point2D<u32, UnknownUnit>;

// Console engine initialization
const MIN_WIDTH: u32 = 128;
const MIN_HEIGHT: u32 = 32;
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

// Stuff shown in the main menu
const LOGO_TEXT: &str = "  ___  _____  _   _______
 / _ \\/  __ \\| \\ | |  _  \\
/ /_\\ \\ /  \\/|  \\| | | | |
|  _  | |    | . ` | | | |
| | | | \\__/\\| |\\  | |/ /
\\_| |_/\\____/\\_| \\_/___/";
const LOGO_COLOR: Color = Color::Blue;
const QUIT_TEXT: &str = "Press 'q' to quit at any time";
const SELECT_TEXT: [&str; 2] = ["Nerd 1: ", "Nerd 2: "];
const SELECT_COLOR: Color = Color::Magenta;
const START_TEXT: &str = "Press the enter/return key to start the game or skip the intro";

// Stuff used for displaying stuff related to the game
const MAX_ACTION_MESSAGES: usize = 5;
const HORIZONTAL_DIVIDER: &str = "-";

// Manages the terminal, and whats displayed and inputted
pub struct Tui {
    engine: ConsoleEngine,
    width: u32,
    height: u32,
    current_nerd_selection: usize,
    nerd_selects: [usize; 2],
    action_messages: Vec<String>,
    current_action_selection: usize,
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
            current_nerd_selection: 0,
            nerd_selects: [0, 0],
            action_messages: Vec::new(),
            current_action_selection: 0,
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
    pub fn should_quit(&self) -> bool {
        self.engine.is_key_pressed(QUIT_KEY)
    }

    // Returns whether the intro is done
    pub fn intro_done(&self) -> bool {
        self.engine.frame_count as u32 / FPS >= INTRO_TIME * 2
            || self.engine.is_key_pressed(START_KEY)
    }

    // Returns whether the game has started
    pub fn game_started(&self) -> bool {
        self.engine.is_key_pressed(START_KEY)
    }

    // Returns the nerds selected in the main menu
    pub fn selected_nerds(&self) -> Nerds {
        Some([*NERDS[self.nerd_selects[0]], *NERDS[self.nerd_selects[1]]])
    }

    // Adds a new message to be displayed; cuts off messages that aren't shown
    pub fn add_action_message(&mut self, text: &str) {
        self.action_messages.push(String::from(text));
        if self.action_messages.len() > MAX_ACTION_MESSAGES {
            self.action_messages.remove(0);
        }
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
        self.draw_message(INTRO_TEXT[0], -1, Color::Reset);
        if self.engine.frame_count as u32 / FPS >= INTRO_TIME {
            self.draw_message(INTRO_TEXT[1], 0, INTRO_COLOR);
        }
    }

    // Draws the main menu
    fn draw_menu(&mut self) {
        self.draw_logo();
        self.draw_message(QUIT_TEXT, -2, Color::Reset);
        self.draw_message(START_TEXT, 3, Color::Reset);

        let first_text = [SELECT_TEXT[0], NERDS[self.nerd_selects[0]].name].concat();
        self.draw_select(&first_text, 0, self.current_nerd_selection == 0);
        let second_text = [SELECT_TEXT[1], NERDS[self.nerd_selects[1]].name].concat();
        self.draw_select(&second_text, 1, self.current_nerd_selection == 1);

        self.draw_menu_nerd();
        self.menu_input();
    }

    // Draws the logo in the main menu
    fn draw_logo(&mut self) {
        let logo_pos = Point::new(
            self.width / 2 - LOGO_TEXT.lines().next().unwrap().len() as u32 / 2,
            self.height / 2 - 12,
        );
        self.engine.print_fbg(
            logo_pos.x as i32,
            logo_pos.y as i32,
            LOGO_TEXT,
            LOGO_COLOR,
            Color::Reset,
        );
    }

    // Draws a horizontally centered message
    fn draw_message(&mut self, text: &str, pos: i32, color: Color) {
        let pos = Point::new(
            self.width / 2 - text.len() as u32 / 2,
            ((self.height / 2) as i32 + pos) as u32,
        );
        self.engine
            .print_fbg(pos.x as i32, pos.y as i32, text, color, Color::Reset);
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

    // Draws the current player selected at the main menu
    fn draw_menu_nerd(&mut self) {
        let nerd = NERDS[self.nerd_selects[self.current_nerd_selection]].sprite;
        self.engine.print_fbg(
            (self.width / 2 - nerd.lines().next().unwrap().len() as u32 / 2) as i32,
            (self.height - 12) as i32,
            nerd,
            NERD_COLOR,
            Color::Reset,
        );
    }

    // Manages input in the main menu
    fn menu_input(&mut self) {
        let select = &mut self.nerd_selects[self.current_nerd_selection];
        let selection = &mut self.current_nerd_selection;
        let len = NERDS.len() - 1;

        if self.engine.is_key_pressed(UP_KEY) {
            Self::change_selected(selection, 1, 1);
        } else if self.engine.is_key_pressed(DOWN_KEY) {
            Self::change_selected(selection, 1, -1);
        } else if self.engine.is_key_pressed(LEFT_KEY) {
            Self::change_selected(select, len, -1);
        } else if self.engine.is_key_pressed(RIGHT_KEY) {
            Self::change_selected(select, len, 1);
        }
    }

    // Changes the value of a main menu option or which is selected
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
        self.draw_action_messages();
    }

    // Draws a list of messages stating the actions that have been done
    fn draw_action_messages(&mut self) {
        self.engine.print(
            0,
            (self.height - MAX_ACTION_MESSAGES as u32 - 1) as i32,
            &HORIZONTAL_DIVIDER.repeat(self.width as usize),
        );

        for (i, message) in self.action_messages.iter().enumerate() {
            self.engine.print(
                0,
                (self.height - MAX_ACTION_MESSAGES as u32 + i as u32) as i32,
                message,
            );
        }
    }

    // Draws the end screen
    fn draw_end(&mut self) {
        todo!();
    }
}
