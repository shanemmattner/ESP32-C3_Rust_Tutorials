use esp_idf_hal::{
    gpio::{AnyOutputPin, Output, OutputPin, PinDriver},
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();
    // Initialize Pin 8 as an output to drive the LED
    let led_pin = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();

    // Create thread to blink the LED and pass it the initialized GPIO
    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_thread(led_pin))
        .unwrap();
}

// Thread function that will blink the LED on/off every 500ms
fn blinky_thread(mut led_pin: PinDriver<AnyOutputPin, Output>) {
    loop {
        led_pin.set_low().unwrap();
        println!("LED ON");
        thread::sleep(Duration::from_millis(1000));

        led_pin.set_high().unwrap();
        println!("LED OFF");
        thread::sleep(Duration::from_millis(1000));
    }
}
