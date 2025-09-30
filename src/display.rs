extern "C" {
    fn display_register_driver(driver: *const DisplayDriver);
    fn display_init();
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
