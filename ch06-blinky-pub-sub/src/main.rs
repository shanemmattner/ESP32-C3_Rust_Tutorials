use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use critical_section::Mutex;
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::{gpio, interrupt, prelude::*};
use nuts::*;
use statig::{prelude::*, InitializedStatemachine};
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;

mod fsm;

struct Activity;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led = peripherals.pins.gpio8.into_output().unwrap();
    let state_machine = fsm::Blinky { led }.state_machine().init();

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(state_machine))
        .unwrap();

    let activity = nuts::new_activity(Activity);
    activity.subscribe(|_activity, n: &usize| println!("Subscriber received {}", n));
    nuts::publish(17usize);
    // "Subscriber received 17" is printed
    nuts::publish(289usize);
    // "Subscriber received 289" is printed
}

fn blinky_fsm_thread(mut fsm: InitializedStatemachine<fsm::Blinky>) {
    loop {
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&fsm::Event::TimerElapsed);
        thread::sleep(Duration::from_millis(500));
        fsm.handle(&fsm::Event::TimerElapsed);
    }
}
