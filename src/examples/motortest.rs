use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
// use lib::bcm;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_Motor: u8 = 23;


fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut apin = Gpio::new()?.get(GPIO_Motor)?.into_output();

    
    println!("running motors!!");
    apin.toggle();
    thread::sleep(Duration::from_millis(1000));
    apin.set_low();
    Ok(();
    
}