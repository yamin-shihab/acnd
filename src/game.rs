mod nerds;

use console_engine::ConsoleEngine;
use nerds::Nerd;
use std::process;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;
const FPS: u32 = 60;

enum GameState {
    Intro,
    MainMenu,
    InGame,
    GameEnd,
}

pub struct Game {
    engine: ConsoleEngine,
    game_state: GameState,
    players: Option<[Nerd; 2]>,
    current_player: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            engine: ConsoleEngine::init_fill_require(WIDTH, HEIGHT, FPS).unwrap_or_else(|err| {
                eprintln!("Error starting ConsoleEngine: {}", err);
                process::exit(1);
            }),
            game_state: GameState::MainMenu,
            players: None,
            current_player: 0,
        }
    }
}
