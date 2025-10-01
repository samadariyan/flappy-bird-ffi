use crate::assets;
use crate::color;
use crate::display;

extern "C" {
    fn HAL_GetTick() -> u32;
}

pub enum GameState {
    Start,
    Running,
    End,
}

pub struct Game {
    state: GameState,
    score: u32,
    countdown_start_time: u32,
}

impl Game {
    pub fn init() -> Self {
        Game {
            state: GameState::Start,
            score: 0,
            countdown_start_time: 0,
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Start => {
                // Show. game start screen
                if self.run_countdown() {
                    self.state = GameState::Running
                }
            }
            GameState::Running => {}
            GameState::End => {}
        }
    }

    pub fn draw_start_screen() {
        Game::set_background();
        display::draw_image(40, 160, 40, 80, &assets::GAME_NAME_IMG_DATA);
        let text = c"Game Starts In";
        display::write_string(0, 120, text, color::RED, color::BACKGROUND);
    }

    pub fn set_background() {
        // 1. set the background color
        display::set_background_color(color::BACKGROUND);

        // 2. print the scoreboard area
        print_score_card_background();
        // 3. print the plant
        display::draw_image(0, 60, 210, 30, &assets::PLANT_IMG_DATA);
        display::draw_image(60, 60, 210, 30, &assets::PLANT_IMG_DATA);
        display::draw_image(120, 60, 210, 30, &assets::PLANT_IMG_DATA);
        display::draw_image(180, 60, 210, 30, &assets::PLANT_IMG_DATA);

        // display::set_background_color(bg_color: color::BACKGROUND) ;
    }

    //returns 'true' if countdown is over , otherwise 'false'
    fn run_countdown(&mut self) -> bool {
        if self.countdown_start_time == 0 {
            self.countdown_start_time = unsafe { HAL_GetTick() };
        }

        let elapsed = unsafe { HAL_GetTick() } - self.countdown_start_time;
        let number = if elapsed < 1000 {
            c"3"
        } else if elapsed < 2000 {
            c"2"
        } else if elapsed < 3000 {
            c"1"
        } else {
            self.countdown_start_time = 0;
            return true;
        };
        display::write_string(112, 156, number, color::BLACK, color::BACKGROUND);

        false
    }
}

fn print_score_card_background() {
    display::draw_rectangle(0, 240, 0, 28, color::WHITE);
    display::draw_rectangle(0, 240, 28, 2, color::BLACK);
}
