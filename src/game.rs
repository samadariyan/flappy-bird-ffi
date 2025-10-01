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
        display::draw_image(40, 160, 40, 80, &assets::GAME_NAME_IMAGE_DATA);
    }
}

pub fn set_background() {
    // 1. set the background color
    display::set_background_color(color::BACKGROUND);

    // 2. print the scoreboard area
    print_score_card_background();
    // 3. print the plant
    display::draw_image(0, 60, 210, 30, &assets::PLANT_IMAGE_DATA);
    display::draw_image(60, 60, 210, 30, &assets::PLANT_IMAGE_DATA);
    display::draw_image(120, 60, 210, 30, &assets::PLANT_IMAGE_DATA);
    display::draw_image(180, 60, 210, 30, &assets::PLANT_IMAGE_DATA);

    // display::set_background_color(bg_color: color::BACKGROUND) ;
}
