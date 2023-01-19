use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::bail;
use anyhow::Result;
use embedded_hal::digital::v2::OutputPin;
use embedded_svc::wifi::*;
use esp_idf_hal::{gpio::*, prelude::*};
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::wifi::*;
use statig::prelude::*;
use std::sync::Arc;
use std::{thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 5000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // let mut led = PinDriver::output(peripherals.pins.gpio8).unwrap();
    // let mut state_machine = Blinky { led }.state_machine().init();

    // let _blinky_thread = std::thread::Builder::new()
    //     .stack_size(BLINKY_STACK_SIZE)
    //     .spawn(move || loop {
    //         thread::sleep(Duration::from_millis(500));
    //         state_machine.handle(&Event::TimerElapsed);
    //         thread::sleep(Duration::from_millis(500));
    //         state_machine.handle(&Event::TimerElapsed);
    //     })
    //     .unwrap();
}
