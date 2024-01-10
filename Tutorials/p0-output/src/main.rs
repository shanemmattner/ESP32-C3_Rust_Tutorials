use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Starting 0-output\nThis application is a basic blinky program that turns an LED on and off every 1 second.\n");

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();
    // Initialize Pin 8 as an output to drive the LED
    let mut led_pin = PinDriver::output(peripherals.pins.gpio8).unwrap();

    // Loop forever blinking the LED on/off every 500ms
    loop {
        // Inverse logic to turn LED on
        led_pin.set_low().unwrap();
        println!("LED ON");
        FreeRtos::delay_ms(1000);

        led_pin.set_high().unwrap();
        println!("LED OFF");
        FreeRtos::delay_ms(1000);
    }
}
