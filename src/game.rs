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

    pub fn update(&mut self) {
        // todo()
    }
}
