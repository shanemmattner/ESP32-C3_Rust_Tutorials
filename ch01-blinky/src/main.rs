use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use std::{thread, time::Duration};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio8.into_output().unwrap();

    loop{
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(500));
        println!("LED ON");
        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(500));
        println!("LED OFF");
    }




}
