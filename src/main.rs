#![no_std]
#![no_main]
#![allow(dead_code)]

use panic_halt as _;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    loop {}
}
