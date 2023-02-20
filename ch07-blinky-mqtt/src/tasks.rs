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

use crate::led_fsm;

pub static ADC_MAX_COUNTS: u32 = 2850;
pub static BLINKY_STACK_SIZE: usize = 2000;
pub static BUTTON_STACK_SIZE: usize = 2000;
pub static ADC_STACK_SIZE: usize = 5000;

pub fn button_thread(btn: PinDriver<'_, AnyInputPin, Input>, tx: crossbeam_channel::Sender<bool>) {
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

pub fn blinky_fsm_thread(
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

pub fn adc_thread<T: esp_idf_hal::gpio::ADCPin>(
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
