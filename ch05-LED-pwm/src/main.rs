use crossbeam_channel::bounded;
use esp_idf_hal::{
    gpio::{AnyIOPin, AnyOutputPin, IOPin, Input, Output, OutputPin, PinDriver, Pull},
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();
    // Initialize Pin 8 as an output to drive the LED
    let led_pin = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    // Initialize Pin 6 as an input to read the button status
    let mut btn_pin = PinDriver::input(peripherals.pins.gpio6.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    let (tx, rx) = bounded(1);

    // Create thread to blink the LED and pass it the initialized GPIO
    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_thread(led_pin, rx))
        .unwrap();

    // Create thread to blink the LED and pass it the initialized GPIO
    let _button_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || button_thread(btn_pin, tx))
        .unwrap();
}

// Thread function that will blink the LED on/off every 500ms
fn blinky_thread(
    mut led_pin: PinDriver<AnyOutputPin, Output>,
    rx: crossbeam_channel::Receiver<bool>,
) {
    let mut blinky_status = false;
    loop {
        match rx.try_recv() {
            Ok(x) => blinky_status = x,
            Err(_) => {}
        }
        if blinky_status {
            led_pin.set_low().unwrap();
            println!("LED ON");
            thread::sleep(Duration::from_millis(1000));

            led_pin.set_high().unwrap();
            println!("LED OFF");
            thread::sleep(Duration::from_millis(1000));
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn button_thread(btn_pin: PinDriver<AnyIOPin, Input>, tx: crossbeam_channel::Sender<bool>) {
    let mut btn_status = false;

    loop {
        if btn_pin.is_high() {
            if !btn_status {
                btn_status = true;
                println!("BUTTON ON");
                tx.send(btn_status).unwrap();
            }
        } else {
            if btn_status {
                btn_status = false;
                println!("BUTTON OFF");
                tx.send(btn_status).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
