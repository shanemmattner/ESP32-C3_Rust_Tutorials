#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
use esp_idf_hal::{
    gpio::{
        AnyInputPin, AnyOutputPin, Input, InputMode, InputPin, InterruptType, Output, OutputPin,
        PinDriver, Pull,
    },
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use statig::{prelude::*, InitializedStatemachine};
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;

static mut COUNTER: u32 = 0;

mod led_fsm;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let btn = PinDriver::input(peripherals.pins.gpio7.downgrade_input()).unwrap();

    let mut led2 = PinDriver::output(peripherals.pins.gpio6.downgrade_output()).unwrap();
    let mut btn2 = PinDriver::input(peripherals.pins.gpio0).unwrap();
    btn2.set_pull(Pull::Up).unwrap();
    btn2.set_interrupt_type(InterruptType::AnyEdge).unwrap();

    unsafe {
        btn2.subscribe(|| btn_int()).unwrap();
    }

    let led_fsm = led_fsm::Blinky { led }.state_machine().init();

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(led_fsm, btn))
        .unwrap();

    loop {
        unsafe {
            println!("counter {COUNTER}\n");
        }
        thread::sleep(Duration::from_millis(1000));
    }
}

fn btn_int() {
    //log::error!("int");
    unsafe {
        COUNTER += 1;
    }
    //    println!("int\n");
}

fn blinky_fsm_thread<'a>(
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
