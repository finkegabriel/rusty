use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_Motor: u8 = 23;
const GPIO_MotorTwo: u8 =24; //???

fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut apin = Gpio::new()?.get(GPIO_Motor)?.into_output();
    let mut bpin = Gpio::new()?.get(GPIO_MotorTwo)?.into_output();
    
    print!("running motors!!");

    loop{
        apin.toggle();
        thread::sleep(Duration::from_millis(500));
        bpin.toggle();
        thread::sleep(Duration::from_millis(500));
    }
}