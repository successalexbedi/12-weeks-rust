#[derive(Debug, Clone)]
pub enum GameState {
    Menu,
    Playing { score: i32, level: i32 },
    Paused { saved_score: i32, saved_level: i32 },
    GameOver { final_score: i32 },
}

impl GameState {
    pub fn new() -> Self {
        Self::Menu
    }

    pub fn description(&self) -> String {
        match self {
            Self::Menu => format!("In main menu"),
            Self::Playing { score, level } => {
                format!("Playing - Level {level}, Score {score}")
            }
            Self::Paused { saved_score, saved_level } => {
                format!("Paused - Level {saved_level}, Score {saved_score}")
            }
            Self::GameOver { final_score } => {
                format!("Game Over - Final Score: {final_score}")
            }
        }
    }

    pub fn get_score(&self) -> Option<i32> {
        match self {
            Self::Playing { score, .. } => Some(*score),
            Self::Paused { saved_score, .. } => Some(*saved_score),
            Self::GameOver { final_score } => Some(*final_score),
            Self::Menu => None,
        }
    }

    pub fn pause(self) -> Self {
        match self {
            Self::Playing { score, level } => Self::Paused {
                saved_score: score,
                saved_level: level,
            },
            _ => self,
        }
    }

    pub fn resume(self) -> Self {
        match self {
            Self::Paused { saved_score, saved_level } => Self::Playing {
                score: saved_score,
                level: saved_level,
            },
            _ => self,
        }
    }
}

fn main() {
    let mut state = GameState::new();
    println!("{}", state.description());

    state = GameState::Playing { score: 0, level: 1 };
    println!("{}", state.description());

    match state {
        GameState::Playing { .. } => println!("Is playing: true"),
        _ => println!("Is playing: false"),
    }

    if let Some(s) = state.get_score() {
        println!("Score: {}", s);
    }

    state = GameState::Playing { score: 150, level: 3 };
    
    state = state.pause();
    println!("{}", state.description());

    state = state.resume();
    println!("{}", state.description());

    state = GameState::GameOver { final_score: 200 };
    println!("{}", state.description());

    if let Some(final_s) = state.get_score() {
        println!("Final score: {}", final_s);
    }
}
