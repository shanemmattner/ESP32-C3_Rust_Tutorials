use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{IOPin, PinDriver, Pull},
    peripherals::Peripherals,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Starting 1-output\nThis application adds a button to control an LED blinking on and off every 1 second.\n");

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();
    // Initialize Pin 8 as an output to drive the LED
    let mut led_pin = PinDriver::output(peripherals.pins.gpio8).unwrap();
    // Initialize Pin 6 as an input to read the button status
    let mut btn_pin = PinDriver::input(peripherals.pins.gpio6.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    loop {
        if btn_pin.is_high() {
            led_pin.set_low().unwrap();
            println!("LED ON");
            FreeRtos::delay_ms(1000);

            led_pin.set_high().unwrap();
            println!("LED OFF");
        } else {
            led_pin.set_high().unwrap();
        }
        FreeRtos::delay_ms(1000);
    }
}
