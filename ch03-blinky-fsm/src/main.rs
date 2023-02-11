#![allow(unused_imports)]
#![allow(dead_code)]
#[allow(unused)]

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::{gpio, prelude::*};
use statig::{prelude::*, InitializedStatemachine};
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;

mod fsm;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led = peripherals.pins.gpio8.into_output().unwrap();
    let state_machine = fsm::Blinky::default().state_machine().init();

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(state_machine))
        .unwrap();
}

fn blinky_fsm_thread(mut fsm: InitializedStatemachine<fsm::Blinky>) {
    loop {
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&fsm::Event::TimerElapsed);
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&fsm::Event::TimerElapsed);
    }
}
