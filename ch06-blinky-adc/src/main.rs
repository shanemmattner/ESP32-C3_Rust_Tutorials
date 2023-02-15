#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
extern crate crossbeam;
extern crate crossbeam_channel;

use crossbeam_channel::bounded;
use embedded_hal_0_2::PwmPin;
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::{
    adc::{self, config::Config, AdcDriver, Atten11dB, *},
    gpio::{
        self, AnyInputPin, AnyOutputPin, Input, InputMode, InputPin, Output, OutputPin, PinDriver,
        *,
    },
    ledc::*,
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use statig::{prelude::*, InitializedStatemachine};
use std::{
    sync::{mpsc::*, Arc},
    thread,
    time::Duration,
};

static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;
static ADC_STACK_SIZE: usize = 5000;

static ADC_MAX_COUNTS: u32 = 2850;

mod led_fsm;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // Config GPIO for input and output
    let led = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let btn = PinDriver::input(peripherals.pins.gpio6.downgrade_input()).unwrap();

    // LED controller config
    let config = TimerConfig::new().frequency(25.kHz().into());
    let timer = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &config).unwrap());
    let channel0 = LedcDriver::new(
        peripherals.ledc.channel0,
        timer.clone(),
        peripherals.pins.gpio9,
    )
    .unwrap();

    let max_duty = channel0.get_max_duty();

    let led_fsm = led_fsm::Blinky { led }.state_machine().init();

    let (tx, rx) = bounded(1);

    // print_type_of(&tx);

    let a1_ch4 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio4).unwrap();

    let adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_fsm_thread(led_fsm, rx))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || button_thread(btn, tx))
        .unwrap();

    let _adc_thread = std::thread::Builder::new()
        .stack_size(ADC_STACK_SIZE)
        .spawn(move || adc_thread(adc1, a1_ch4, max_duty, channel0))
        .unwrap();
}

fn adc_thread<T: esp_idf_hal::gpio::ADCPin>(
    mut adc: AdcDriver<adc::ADC1>,
    mut pin: adc::AdcChannelDriver<T, adc::Atten11dB<adc::ADC1>>,
    max_duty: u32,
    mut channel: LedcDriver<'_>,
) where
    Atten11dB<ADC1>: Attenuation<<T as ADCPin>::Adc>,
{
    loop {
        // Read ADC and and set the LED PWM to the percentage of full scale
        match adc.read(&mut pin) {
            Ok(x) => {
                let pwm = (x as u32 * max_duty) / ADC_MAX_COUNTS;
                match channel.set_duty(pwm) {
                    Ok(x) => (),
                    Err(e) => println!("err setting duty of led: {e}\n"),
                }
            }
            Err(e) => println!("err reading ADC: {e}\n"),
        }

        thread::sleep(Duration::from_millis(100));
    }
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

fn button_thread(btn: PinDriver<'_, AnyInputPin, Input>, tx: crossbeam_channel::Sender<bool>) {
    let mut btn_state = true;
    loop {
        if btn.is_high() {
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
