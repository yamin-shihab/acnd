use crate::nerds::{Nerd, Nerds};
use crate::tui::Tui;

// String used at beginning of game to introduce players
const GAME_START_MESSAGE: &str = "Two nerds bump into each other. nerd0 and nerd1 glare at each other. The fight chant is heard. The AC Nerd Duels have begun.";
const GAME_END_MESSAGE: &str = "As the dust settles, nerd0 looks down at the unconcious nerd1 before being escorted to the principal's office.";

// Contains game information
pub struct Game {
    tui: Tui,
    game_state: GameState,
    nerds: Option<Nerds>,
    current_nerd: usize,
    action_selected: usize,
    equation: String,
    answer: i32,
    critical: bool,
}

impl Game {
    // Creates new instance of the game
    pub fn new() -> Self {
        Self {
            tui: Tui::new(),
            game_state: GameState::Intro,
            nerds: None,
            current_nerd: 0,
            action_selected: 0,
            equation: String::new(),
            answer: 0,
            critical: Nerd::critical(),
        }
    }

    // Runs every frame
    pub fn main_loop(&mut self) {
        loop {
            self.tui.update(
                self.game_state,
                &self.nerds,
                self.current_nerd,
                &self.equation,
            );
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
                if let Some(nerds) = self.tui.nerds_chosen() {
                    self.start_game(nerds);
                }
            }
            GameState::InGame(InGameState::Choosing) => self.update_choosing(),
            GameState::InGame(InGameState::Mathing) => self.update_mathing(),
            GameState::GameEnd => (),
        }
    }

    // Initializes the start of the game
    fn start_game(&mut self, nerds: Nerds) {
        self.game_state = GameState::InGame(InGameState::Choosing);
        self.tui.add_action_message(
            &GAME_START_MESSAGE
                .replace("nerd0", nerds[0].name)
                .replace("nerd1", nerds[1].name),
        );
        self.nerds = Some(nerds);
    }

    // Updates the game when choosing action
    fn update_choosing(&mut self) {
        if let Some(nerd_names) = self.game_ended() {
            self.end_game(nerd_names)
        }
        if let Some(action) = self.tui.action_chosen() {
            self.action_selected = action;
            self.game_state = GameState::InGame(InGameState::Mathing);
            let other = usize::from(self.current_nerd == 0);
            if let Some(nerds) = &self.nerds {
                (self.equation, self.answer) = nerds[self.current_nerd].equation(
                    self.action_selected,
                    &nerds[other],
                    self.critical,
                );
            }
        }
    }

    // Returns the winning nerd, otherwise none
    fn game_ended(&self) -> Option<[&'static str; 2]> {
        if let Some(nerds) = &self.nerds {
            if nerds[0].health < 1 {
                return Some([nerds[1].name, nerds[0].name]);
            } else if nerds[1].health < 1 {
                return Some([nerds[0].name, nerds[1].name]);
            }
        }
        None
    }

    // Ends the game
    fn end_game(&mut self, nerd_names: [&str; 2]) {
        self.game_state = GameState::GameEnd;
        self.tui.add_action_message(
            &GAME_END_MESSAGE
                .replace("nerd0", nerd_names[0])
                .replace("nerd1", nerd_names[1]),
        );
    }

    // Updates the game when entering math answer
    fn update_mathing(&mut self) {
        if self.tui.back() {
            self.game_state = GameState::InGame(InGameState::Choosing);
            return;
        }
        if let Some(nerds) = &mut self.nerds {
            let other = usize::from(self.current_nerd == 0);
            if let Some(num) = self.tui.math_chosen() {
                if num == self.answer {
                    let (first, second) = nerds.split_at_mut(1);
                    self.tui.add_action_message(&if self.current_nerd == 0 {
                        first[0].use_action(
                            self.action_selected,
                            self.answer,
                            self.critical,
                            &mut second[0],
                        )
                    } else {
                        second[0].use_action(
                            self.action_selected,
                            self.answer,
                            self.critical,
                            &mut first[0],
                        )
                    });
                }
                self.game_state = GameState::InGame(InGameState::Choosing);
                self.current_nerd = other;
                self.critical = Nerd::critical();
            }
        }
    }
}

// Represents the possible states the game can be in
#[derive(Copy, Clone)]
pub enum GameState {
    Intro,
    MainMenu,
    InGame(InGameState),
    GameEnd,
}

// Represents what is going on in game
#[derive(Copy, Clone)]
pub enum InGameState {
    Choosing,
    Mathing,
}
