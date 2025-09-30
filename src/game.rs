use crate::{assets, display};

pub enum GameState {
    Start,
    Running,
    End,
}

pub struct Game {
    state: GameState,
    score: u32,
}

impl Game {
    pub fn init() -> Self {
        Game {
            state: GameState::Start,
            score: 0,
        }
    }

    pub fn update(&self) {
        match self.state {
            GameState::Start => {
                // Show. game start screen
                Game::draw_start_screen();
            }
            GameState::Running => {}
            GameState::End => {}
        }
    }

    pub fn draw_start_screen() {
        display::draw_image(&assets::GAME_NAME_IMAGE_DATA)
    }
}
