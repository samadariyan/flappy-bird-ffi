#![no_std]
#![no_main]
#![allow(dead_code)]

mod display;
mod game;

use display::DisplayDriver;
use game::Game;
use panic_halt as _;

extern "C" {
    fn c_main();
    static gc9a01a_driver: DisplayDriver;
}

#[no_mangle]
extern "C" fn main() -> ! {
    unsafe {
        c_main();
        display::register_driver(&gc9a01a_driver);
    }

    let game = Game::init();

    loop {
        game.update()
    }
}
