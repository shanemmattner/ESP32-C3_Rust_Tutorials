use esp_idf_hal::{gpio::PinDriver, prelude::*};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
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
        // Inverse logic to turn LED on
        led.set_low().unwrap();
        println!("LED ON");
        thread::sleep(Duration::from_millis(1000));

        led.set_high().unwrap();
        println!("LED OFF");
        thread::sleep(Duration::from_millis(1000));
    }
}
