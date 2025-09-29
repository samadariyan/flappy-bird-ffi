#[repr(C)]
pub struct DisplayDriver {
    __private: [u8; 0],
}

pub fn register_driver(driver: &DisplayDriver) {}

pub fn init() {}
