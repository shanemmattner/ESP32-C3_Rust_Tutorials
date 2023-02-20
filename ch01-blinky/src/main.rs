use esp_idf_hal::{gpio::PinDriver, prelude::*};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::{thread, time::Duration};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();
    // Initialize Pin 8 as an output to drive the LED
    let mut led = PinDriver::output(peripherals.pins.gpio8).unwrap();

    // Loop forever blinking the LED on/off every 500ms
    loop {
        thread::sleep(Duration::from_millis(500));
        println!("LED ON");
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(500));
        println!("LED OFF");
        led.set_low().unwrap();
    }
}
