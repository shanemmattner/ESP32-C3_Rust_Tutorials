extern crate crossbeam;
extern crate crossbeam_channel;

use crossbeam_channel::bounded;
use esp_idf_hal::{
    gpio::{AnyInputPin, Input, InputPin, OutputPin, PinDriver},
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use statig::{prelude::*, InitializedStatemachine};
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;

mod led_fsm;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led_pin = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let btn_pin = PinDriver::input(peripherals.pins.gpio6.downgrade_input()).unwrap();

    let led_fsm = led_fsm::Blinky { led_pin }.state_machine().init();

    let (tx, rx) = bounded(1);
    print_type_of(&tx);

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(led_fsm, rx))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || button_thread(btn_pin, tx))
        .unwrap();
}

fn blinky_fsm_thread(
    mut fsm: InitializedStatemachine<led_fsm::Blinky>,
    rx: crossbeam_channel::Receiver<bool>,
) {
    loop {
        fsm.handle(&led_fsm::Event::TimerElapsed);
        match rx.try_recv() {
            Ok(_) => fsm.handle(&led_fsm::Event::ButtonPressed),
            Err(_) => {}
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

fn button_thread(btn_pin: PinDriver<'_, AnyInputPin, Input>, tx: crossbeam_channel::Sender<bool>) {
    let mut btn_state = true;
    loop {
        if btn_pin.is_high() {
            if !btn_state {
                btn_state = true;
                tx.send(btn_state).unwrap();
            }
        } else {
            btn_state = false;
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
