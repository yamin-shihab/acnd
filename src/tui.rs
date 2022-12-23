use crate::game::{GameState, InGameState};
use crate::nerds::{Nerd, Nerds, CURRENT_NERD_COLOR, NERDS, WAITING_NERD_COLOR};
use console_engine::{Color, ConsoleEngine, KeyCode};
use euclid::{Point2D, UnknownUnit};

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
const INTRO_TEXTS: [&str; 2] = ["Let AC be Academically Challenged in:", "AC NERD DUELS"];
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
const QUIT_TEXT: &str = "Use the arrow keys to select something, and 'q' to quit at any time";
const SELECT_TEXTS: [&str; 2] = ["Nerd 1: ", "Nerd 2: "];
const SELECT_COLOR: Color = Color::Magenta;
const START_TEXT: &str = "Press the enter/return key to start the game or skip the intro";

// Stuff used for displaying stuff related to the game
const MAX_ACTION_MESSAGES: usize = 5;
const HORIZONTAL_DIVIDER: &str = "-";
const ACTION_LIST_WIDTH: u32 = 30;
const VERTICAL_DIVIDER: &str = "|\n";

// Error message
const ENGINE_FAIL_ERR_MSG: &str = "Console Engine failed to start";

// Represents a point on the screen
type Point = Point2D<u32, UnknownUnit>;

// Manages the terminal, and whats displayed and inputted
pub struct Tui {
    pub engine: ConsoleEngine,
    width: u32,
    height: u32,
    current_nerd_selection: usize,
    nerd_selects: [usize; 2],
    action_messages: Vec<String>,
    current_action_selection: usize,
    inputted_number: String,
}

impl Tui {
    // Creates a new TUI
    pub fn new() -> Self {
        let engine = ConsoleEngine::init_fill_require(MIN_WIDTH, MIN_HEIGHT, FPS)
            .expect(ENGINE_FAIL_ERR_MSG);
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
            inputted_number: String::new(),
        }
    }

    // Updates the TUI
    pub fn update(
        &mut self,
        game_state: GameState,
        nerds: &Option<Nerds>,
        current_nerd: usize,
        equation: &str,
    ) {
        self.draw_and_input(game_state, nerds, current_nerd, equation);
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

    // Returns the chosen nerds if the game has started
    pub fn nerds_chosen(&self) -> Option<Nerds> {
        if self.engine.is_key_pressed(START_KEY) {
            return Some([*NERDS[self.nerd_selects[0]], *NERDS[self.nerd_selects[1]]]);
        }
        None
    }

    // Adds a new message to be displayed; cuts off messages that aren't shown
    pub fn add_action_message(&mut self, text: &str) {
        self.action_messages.push(text.to_string());
        if self.action_messages.len() > MAX_ACTION_MESSAGES {
            self.action_messages.remove(0);
        }
    }

    // Returns the chosen action (if one is chosen)
    pub fn action_chosen(&self) -> Option<usize> {
        if self.engine.is_key_pressed(START_KEY) {
            return Some(self.current_action_selection);
        }
        None
    }

    // Returns the inputted math number if it was entered
    pub fn number_chosen(&mut self) -> Option<f64> {
        if self.engine.is_key_pressed(START_KEY) {
            let num = self.inputted_number.parse();
            if let Ok(num) = num {
                self.inputted_number = String::new();
                return Some(num);
            }
        }
        None
    }

    // Draws everything related to the current game state
    fn draw_and_input(
        &mut self,
        game_state: GameState,
        nerds: &Option<Nerds>,
        current_nerd: usize,
        equation: &str,
    ) {
        match game_state {
            GameState::Intro => self.draw_intro(),
            GameState::MainMenu => {
                self.draw_menu(current_nerd);
                self.input_menu();
            }
            GameState::InGame(state) => {
                self.draw_game(state, nerds, current_nerd, equation);
                self.input_game(state);
            }
            GameState::GameEnd => {
                self.draw_game(InGameState::Choosing, nerds, current_nerd, equation)
            }
        }
    }

    // Draws the intro
    fn draw_intro(&mut self) {
        self.draw_centered_message(INTRO_TEXTS[0], -1, Color::Reset);
        if self.engine.frame_count as u32 / FPS >= INTRO_TIME {
            self.draw_centered_message(INTRO_TEXTS[1], 0, INTRO_COLOR);
        }
    }

    // Draws a horizontally centered message
    fn draw_centered_message(&mut self, text: &str, pos: i32, color: Color) {
        let pos = Point::new(
            self.width / 2 - text.len() as u32 / 2,
            ((self.height / 2) as i32 + pos) as u32,
        );
        self.engine
            .print_fbg(pos.x as i32, pos.y as i32, text, color, Color::Reset);
    }

    // Draws the main menu
    fn draw_menu(&mut self, current_nerd: usize) {
        self.draw_logo();

        self.draw_centered_message(QUIT_TEXT, -4, Color::Reset);
        self.draw_centered_message(START_TEXT, 1, Color::Reset);

        let first_text = SELECT_TEXTS[0].to_string() + NERDS[self.nerd_selects[0]].name;
        self.draw_centered_message(
            &first_text,
            -2,
            Self::selection_color(self.current_nerd_selection == 0),
        );
        let second_text = SELECT_TEXTS[1].to_string() + NERDS[self.nerd_selects[1]].name;
        self.draw_centered_message(
            &second_text,
            -1,
            Self::selection_color(self.current_nerd_selection == 1),
        );

        self.draw_nerd(NERDS[self.nerd_selects[0]], -20, current_nerd == 0);
        self.draw_nerd(NERDS[self.nerd_selects[1]], 20, current_nerd == 1);
    }

    // Draws the logo in the main menu
    fn draw_logo(&mut self) {
        let len = LOGO_TEXT.lines().next().unwrap_or(LOGO_TEXT).len();
        let pos = Point::new(self.width / 2 - len as u32 / 2, self.height / 2 - 12);
        self.engine.print_fbg(
            pos.x as i32,
            pos.y as i32,
            LOGO_TEXT,
            LOGO_COLOR,
            Color::Reset,
        );
    }

    // Returns the suitable color for whether a selection is selected
    fn selection_color(selected: bool) -> Color {
        if selected {
            SELECT_COLOR
        } else {
            Color::Reset
        }
    }

    // Draws a nerd at position with suitable color
    fn draw_nerd(&mut self, nerd: &Nerd, pos: i32, current_nerd: bool) {
        let mut lines = nerd.sprite.lines();
        let len = lines.next().unwrap_or(nerd.sprite).len();
        self.engine.print_fbg(
            (self.width / 2 - len as u32 / 2) as i32 + pos,
            self.height as i32 - MAX_ACTION_MESSAGES as i32 - lines.count() as i32 - 4,
            nerd.sprite,
            Self::nerd_color(current_nerd),
            Color::Reset,
        );
    }

    // Manages input in the main menu
    fn input_menu(&mut self) {
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
    fn draw_game(
        &mut self,
        in_game_state: InGameState,
        nerds: &Option<Nerds>,
        current_nerd: usize,
        equation: &str,
    ) {
        if let Some(nerds) = &nerds {
            match in_game_state {
                InGameState::Choosing => {
                    self.draw_action_messages();
                    self.draw_stats(nerds, current_nerd);
                    self.draw_nerds(nerds, current_nerd);
                    self.draw_action_list(nerds, current_nerd);
                }
                InGameState::Mathing => {
                    self.draw_action_messages();
                    self.draw_stats(nerds, current_nerd);
                    self.draw_nerds(nerds, current_nerd);
                    self.draw_number(equation);
                }
            }
        }
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
                1,
                (self.height - MAX_ACTION_MESSAGES as u32 + i as u32) as i32,
                message,
            );
        }
    }

    // Prints the stats of the nerds (health, multiplier)
    fn draw_stats(&mut self, nerds: &Nerds, current_nerd: usize) {
        self.engine.print_fbg(
            0,
            self.height as i32 - MAX_ACTION_MESSAGES as i32 - 2,
            &self.stats_string(&nerds[0]),
            Self::nerd_color(0 == current_nerd),
            Color::Reset,
        );
        let stats = self.stats_string(&nerds[1]);
        self.engine.print_fbg(
            self.width as i32 - stats.len() as i32,
            self.height as i32 - MAX_ACTION_MESSAGES as i32 - 2,
            &stats,
            Self::nerd_color(1 == current_nerd),
            Color::Reset,
        );
        self.engine.print(
            0,
            self.height as i32 - MAX_ACTION_MESSAGES as i32 - 3,
            &HORIZONTAL_DIVIDER.repeat(self.width as usize),
        );
    }

    // Returns the string used for printing the nerd's stats
    fn stats_string(&self, nerd: &Nerd) -> String {
        format!(
            " {}: Health = {}, Multiplier = {} ",
            nerd.name, nerd.health, nerd.multiplier,
        )
    }

    // Returns the appropriate color of the nerd (if they are the current nerds)
    fn nerd_color(current_nerd: bool) -> Color {
        if current_nerd {
            CURRENT_NERD_COLOR
        } else {
            WAITING_NERD_COLOR
        }
    }

    // Draws the nerds of the game with suitable colors
    fn draw_nerds(&mut self, nerds: &Nerds, current_nerd: usize) {
        self.draw_nerd(&nerds[0], -50, current_nerd == 0);
        self.draw_nerd(&nerds[1], 10, current_nerd == 1);
    }

    // Draws the list of actions that the current nerd can use
    fn draw_action_list(&mut self, nerds: &Nerds, current_nerd: usize) {
        self.engine.print(
            (self.width - ACTION_LIST_WIDTH) as i32 - 2,
            0,
            &VERTICAL_DIVIDER.repeat(self.height as usize - MAX_ACTION_MESSAGES - 3),
        );

        for (i, action) in nerds[current_nerd].actions.iter().enumerate() {
            self.draw_action(i, &action.name());
        }
    }

    // Draws an action in the action list
    fn draw_action(&mut self, pos: usize, name: &str) {
        self.engine.print_fbg(
            (self.width - ACTION_LIST_WIDTH) as i32,
            pos as i32 + self.height as i32 / 2 - 4,
            name,
            Self::selection_color(self.current_action_selection == pos),
            Color::Reset,
        );
    }

    // Draws the number input bar
    fn draw_number(&mut self, equation: &str) {
        self.engine
            .print(0, 0, &format!("{}: {}", equation, self.inputted_number));
        self.engine
            .print(0, 1, &HORIZONTAL_DIVIDER.repeat(self.width as usize));
    }

    // Processes input for the game
    fn input_game(&mut self, state: InGameState) {
        match state {
            InGameState::Choosing => self.action_list_input(),
            InGameState::Mathing => self.math_input(),
        }
    }

    // Process input for switching the current action
    fn action_list_input(&mut self) {
        if self.engine.is_key_pressed(UP_KEY) {
            Self::change_selected(&mut self.current_action_selection, 3, -1);
        } else if self.engine.is_key_pressed(DOWN_KEY) {
            Self::change_selected(&mut self.current_action_selection, 3, 1);
        }
    }

    // Processes input for solving math equations
    fn math_input(&mut self) {
        for num in '0'..='9' {
            if self.engine.is_key_pressed(KeyCode::Char(num)) {
                self.inputted_number.push(num);
            }
        }
        if self.engine.is_key_pressed(KeyCode::Char('.')) {
            self.inputted_number.push('.')
        }
        if self.engine.is_key_pressed(KeyCode::Backspace) {
            self.inputted_number.pop();
        }
    }
}
