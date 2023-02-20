use esp_idf_hal::{
    gpio::{AnyInputPin, Input, InputPin, OutputPin, PinDriver},
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use statig::{prelude::*, InitializedStatemachine};
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;

mod led_fsm;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();
    // Initialize an output pin to drive the LED
    let led = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    // Initialize an input pin for the button
    let btn = PinDriver::input(peripherals.pins.gpio6.downgrade_input()).unwrap();
    // Create and initialize the finitie state machine. Pass in the LED gpio pin
    let led_fsm = led_fsm::Blinky { led }.state_machine().init();

    // Create thread in which the fsm will run and the button status will be monitored
    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(led_fsm, btn))
        .unwrap();
}

fn blinky_fsm_thread(
    mut fsm: InitializedStatemachine<led_fsm::Blinky>,
    btn: PinDriver<AnyInputPin, Input>,
) {
    let mut btn_state = true;
    let mut led_count = 0;
    loop {
        led_count += 1;
        if led_count > 10 {
            led_count = 0;
            fsm.handle(&led_fsm::Event::TimerElapsed);
        }

        if btn.is_high() {
            if !btn_state {
                btn_state = true;
                fsm.handle(&led_fsm::Event::ButtonPressed);
            }
        } else {
            btn_state = false;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
