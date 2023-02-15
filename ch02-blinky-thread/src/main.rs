use esp_idf_hal::{
    gpio::{AnyOutputPin, Output, OutputPin, PinDriver},
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    // print_type_of(&led);

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_thread(led))
        .unwrap();
}

fn blinky_thread(mut led: PinDriver<AnyOutputPin, Output>) {
    loop {
        thread::sleep(Duration::from_millis(500));
        println!("LED ON");
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(500));
        println!("LED OFF");
        led.set_low().unwrap();
    }
}
