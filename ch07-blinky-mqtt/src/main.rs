extern crate crossbeam;
extern crate crossbeam_channel;

use crossbeam_channel::bounded;
use esp_idf_hal::{
    adc::{self, AdcDriver},
    gpio::{InputPin, PinDriver, *},
    ledc::{config::TimerConfig, *},
    prelude::*,
};
use esp_idf_svc::eventloop::*;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use serde::Serialize;
use statig::prelude::*;
use std::sync::Arc;

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
    let (tx1, rx1) = bounded(1); // make channel to pass data

    // Config GPIO for input and output that will be passed to FSM
    let led = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let btn = PinDriver::input(peripherals.pins.gpio6.downgrade_input()).unwrap();
    // TODO: how to make the `btn` pin pull-up or pull-down.
    let led_fsm = led_fsm::Blinky { led }.state_machine().init();

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
    // ADC config to change LED pwm
    let a1_ch4 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio4).unwrap();
    let adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    // Start WIFI and get MQTT client
    let sysloop = EspSystemEventLoop::take().unwrap();
    let _wifi = cloud::wifi(peripherals.modem, sysloop).unwrap();
    let mut _client = cloud::get_client(MQTT_URL).unwrap();

    // Initialize threads
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
        .spawn(move || tasks::adc_thread(adc1, a1_ch4, max_duty, channel0))
        .unwrap();
}
