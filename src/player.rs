use crate::assets;
use crate::color;
use crate::config::*;
use crate::display;

pub struct Player {
    x: Coord,
    y: Coord,
    w: u32,
    h: u32,
}

impl Player {
    pub fn init() -> Self {
        Self {
            x: INIT_PLAYER_POS_X,
            y: INIT_PLAYER_POS_Y,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        }
    }

    pub fn move_player(&mut self) {
        let old_y = self.y;
        self.y += GRAVITY;
        self.draw();
        self.clear(old_y);
    }

    fn draw(&self) {
        display::draw_image(self.x, self.w, self.y, self.h, &assets::BIRD_IMG_DATA);
    }

    fn clear(&self, old_y: Coord) {
        let change_of_y = self.y - old_y;
        let clear_y = if change_of_y.is_negative() {
            self.y + PLAYER_HEIGHT as Coord
        } else {
            old_y
        };
        display::draw_rectangle(self.x, self.w, self.y, self.h, color::BACKGROUND);
    }

    pub fn get_xy(&self) -> (Coord, Coord) {
        (self.x, self.y)
    }
}
