//here will keep code which reads the sensor data and enables us to control the birds's movement
//1. read the raw accelerometer data
//2. calculate the roll angle
//3. map the roll angle to screen position( px 0 -- px 239); angle ---> pixel position on y axis

use embedded_hal::i2c::I2c;
use embedded_hal_mpu6050_driver::mpu6050::Mpu6050;

pub struct SensorInput<T: I2c> {
    pub mpu6050: Mpu6050<T>,
}

impl<T: I2c> SensorInput<T> {
    pub fn new(device_addr: u8, i2c_interface: T) -> Self {
        Self {
            mpu6050: Mpu6050::new(i2c_interface, device_addr),
        }
    }
}
