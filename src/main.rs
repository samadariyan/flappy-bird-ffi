#![no_std]
#![no_main]
#![allow(dead_code)]

mod game;

use game::Game;
use panic_halt as _;

extern "C" {
    fn c_main();
}

#[no_mangle]
extern "C" fn main() -> ! {
    unsafe {
        c_main();
    }

    let game = Game::init();

    loop {
        // game.update()
    }
}
