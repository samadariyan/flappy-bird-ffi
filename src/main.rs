#![no_std]
#![no_main]
#![allow(dead_code)]

mod assets;
mod color;
mod config;
mod display;
mod game;

use game::Game;
use panic_halt as _;

// use config::*;

extern "C" {
    fn c_main();
    static gc9a01a_driver: display::DisplayDriver;
}

#[no_mangle]
extern "C" fn main() -> ! {
    unsafe {
        c_main();
        display::register_driver(&gc9a01a_driver);
    }

    display::init();

    let game = Game::init();
    // let mut game = game_init(sensor_input).expect("Game init failed");
    loop {
        game.update();
    }
}
