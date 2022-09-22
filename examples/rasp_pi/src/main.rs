mod rasp_pi_driver;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use std::thread;
use std::time::Duration;
use onewire_rs::HardwareDriver;

// Gpio uses BCM pin numbering. BCM GPIO 24 is tied to physical pin 18.
const GPIO_LED: u8 = 24;


fn main() {
    let mut driver = crate::rasp_pi_driver::Driver::new(GPIO_LED).unwrap();
    let res = driver.reset();
    println!("res {:?}", res);
}
