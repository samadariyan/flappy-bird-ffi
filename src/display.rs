use crate::config::*;
use core::convert::TryInto;

extern "C" {
    fn display_register_driver(driver: *const DisplayDriver);
    fn display_init();
    fn display_draw_image(x: u16, w: u16, y: u16, h: u16, img_data: *const u16);
    fn display_fill_screen(color: u16);

}

#[repr(C)]
pub struct DisplayDriver {
    __private: [u8; 0],
}

pub fn register_driver(driver: &DisplayDriver) {
    unsafe {
        display_register_driver(driver as *const DisplayDriver);
    }
}

pub fn init() {
    unsafe {
        display_init();
    }
}

pub fn draw_image(x: Coord, w: u32, y: Coord, h: u32, image_data: &[u16]) {
    let x: u16 = x.try_into().expect("x co-ordinate is out of range");
    let y: u16 = y.try_into().expect("y co-ordinate is out of range");
    let w: u16 = w.try_into().expect("with out of range");
    let h: u16 = h.try_into().expect("height out of range");
    unsafe {
        display_draw_image(x, w, y, h, image_data.as_ptr());
    }
}

pub fn set_background_color(bg_color: u16) {
    unsafe {
        display_fill_screen(bg_color);
    }
}
