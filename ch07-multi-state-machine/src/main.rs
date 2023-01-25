#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::{gpio, prelude::*};
use statig::{prelude::*, InitializedStatemachine};
use std::sync::mpsc;
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;

mod button_fsm;
mod led_fsm;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led = peripherals.pins.gpio8.into_output().unwrap();
    let btn = peripherals.pins.gpio6.into_input().unwrap();

    let (tx_button, rx_blinky) = mpsc::channel::<String>();

    let led_fsm = led_fsm::Blinky { led }.state_machine().init();

    let button_fsm = button_fsm::Button { btn }.state_machine().init();

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(led_fsm))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || button_fsm_thread(button_fsm))
        .unwrap();
}

fn blinky_fsm_thread(mut fsm: InitializedStatemachine<led_fsm::Blinky>) {
    loop {
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&led_fsm::Event::TimerElapsed);
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&led_fsm::Event::TimerElapsed);
    }
}

fn button_fsm_thread(mut fsm: InitializedStatemachine<button_fsm::Button>) {
    loop {
        thread::sleep(Duration::from_millis(10));
        fsm.handle(&button_fsm::Event::Tick);
    }
}
