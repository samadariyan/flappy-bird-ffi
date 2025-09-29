#![no_std]
#![no_main]
#![allow(dead_code)]

use panic_halt as _;

unsafe extern "C" {
    fn c_main();
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    unsafe {
        c_main();
    }
    loop {}
}
