#![no_std]
#![no_main]
#![allow(dead_code)]

mod assets;
mod color;
mod config;
mod display;
mod game;
mod mpu6050;
mod obstacle;
mod player;

use game::Game;
use panic_halt as _;
use stm32f3xx_hal::i2c::I2c as hal_i2c;

use crate::config::MPU6050_DEV_ADDR;

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

    let sensor_input = mpu6050::SensorInput::new(MPU6050_DEV_ADDR, i2c_interface);

    let mut game = Game::init();

    Game::draw_start_screen();

    loop {
        game.update();
    }
}
