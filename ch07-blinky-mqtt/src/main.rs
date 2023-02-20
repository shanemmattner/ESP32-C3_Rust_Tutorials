#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
extern crate crossbeam;
extern crate crossbeam_channel;

use crossbeam_channel::bounded;
use embedded_hal_0_2::PwmPin;
use esp_idf_hal::{
    adc::{self, config::Config, AdcDriver, Atten11dB, *},
    gpio::{
        self, AnyInputPin, AnyOutputPin, Input, InputMode, InputPin, Output, OutputPin, PinDriver,
        *,
    },
    ledc::{config::TimerConfig, *},
    peripheral,
    prelude::*,
};
use esp_idf_svc::eventloop::*;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use serde::Serialize;
use statig::{prelude::*, InitializedStatemachine};
use std::{
    env,
    sync::{mpsc::*, Arc},
    thread,
    time::Duration,
};

mod cloud;
mod led_fsm;
mod tasks;

#[derive(Serialize, Debug)]
struct MqttData {
    distance: u16,
    temperature: f32,
    tds: f32,
}

const MQTT_URL: &str = env!("MQTT_URL");

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // Config GPIO for input and output
    let led = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let btn = PinDriver::input(peripherals.pins.gpio6.downgrade_input()).unwrap();
    // TODO: how to make the `btn` pin pull-up or pull-down.
    // LED controller config
    let config = TimerConfig::new().frequency(25.kHz().into());
    let timer = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &config).unwrap());
    let channel0 = LedcDriver::new(
        peripherals.ledc.channel0,
        timer.clone(),
        peripherals.pins.gpio7,
    )
    .unwrap();

    let max_duty = channel0.get_max_duty();

    let led_fsm = led_fsm::Blinky { led }.state_machine().init();

    let (tx1, rx1) = bounded(1);
    let (tx2, rx2) = (tx1.clone(), rx1.clone());

    let a1_ch4 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio4).unwrap();

    let adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    // start wifi
    let sysloop = EspSystemEventLoop::take().unwrap();
    let _wifi = cloud::wifi(peripherals.modem, sysloop).unwrap();
    let mut client = cloud::get_client(MQTT_URL).unwrap();

    // initialize the tasks
    let _blinky_thread = std::thread::Builder::new()
        .stack_size(tasks::BLINKY_STACK_SIZE)
        .spawn(move || tasks::blinky_fsm_thread(led_fsm, rx1))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(tasks::BUTTON_STACK_SIZE)
        .spawn(move || tasks::button_thread(btn, tx1))
        .unwrap();

    let _adc_thread = std::thread::Builder::new()
        .stack_size(tasks::ADC_STACK_SIZE)
        .spawn(move || tasks::adc_thread(adc1, a1_ch4, max_duty, channel0, tx2))
        .unwrap();
}