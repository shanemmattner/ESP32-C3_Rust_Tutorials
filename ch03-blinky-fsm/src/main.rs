#![allow(unused_imports)]
#![allow(dead_code)]
#[allow(unused)]
use esp_idf_hal::{
    gpio::{AnyOutputPin, Output, OutputPin, PinDriver},
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
    // Create and initialize the finitie state machine. Pass in the LED gpio pin
    let blinky_fsm = led_fsm::Blinky { led }.state_machine().init();
    // Create thread in which the fsm will run
    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(blinky_fsm))
        .unwrap();
}

// Thread logic that will trigger TimerElapsed events in the blinky FSM which will
// blinky the LED on/off
fn blinky_fsm_thread(mut fsm: InitializedStatemachine<led_fsm::Blinky>) {
    loop {
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&led_fsm::Event::TimerElapsed);
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&led_fsm::Event::TimerElapsed);
    }
}
