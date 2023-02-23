use anyhow::{bail, Context};
use crossbeam_channel::bounded;
use crossbeam_utils::atomic::AtomicCell;
use embedded_svc::wifi::{
    AccessPointConfiguration, AuthMethod, ClientConfiguration, Configuration,
};
use esp_idf_hal::{
    adc::{self, *},
    delay::FreeRtos,
    gpio::{ADCPin, AnyIOPin, IOPin, Input, PinDriver, Pull},
    ledc::{config::TimerConfig, *},
    modem::{Modem, WifiModem},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_svc::{
    eventloop::{EspEventLoop, EspSystemEventLoop, System},
    mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration},
    netif::{EspNetif, EspNetifWait},
    nvs::{EspDefaultNvsPartition, EspNvsPartition, NvsDefault},
    timer::EspTaskTimerService,
    wifi::{EspWifi, WifiWait},
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use serde::Serialize;
use std::{env, net::Ipv4Addr, sync::atomic::*, sync::Arc, thread, time::Duration};

static ADC_MAX_COUNTS: u32 = 2850;

pub fn adc_thread<T: ADCPin>(
    adc_mutex: Arc<AtomicCell<u16>>,
    mut adc: AdcDriver<adc::ADC1>,
    mut pin: adc::AdcChannelDriver<T, adc::Atten11dB<adc::ADC1>>,
) where
    Atten11dB<ADC1>: Attenuation<<T as ADCPin>::Adc>,
{
    loop {
        // Read ADC and and set the LED PWM to the percentage of full scale
        match adc.read(&mut pin) {
            Ok(x) => {
                adc_mutex.store(x);
            }

            Err(e) => println!("err reading ADC: {e}\n"),
        }

        thread::sleep(Duration::from_millis(100));
    }
}

// Thread function that will blink the LED on/off every 500ms
pub fn blinky_thread(
    rx: crossbeam_channel::Receiver<bool>,
    adc_mutex: Arc<AtomicCell<u16>>,
    mut channel: LedcDriver<'_>,
) {
    let mut blinky_status = false;
    let max_duty = channel.get_max_duty();
    loop {
        // Watch for button press messages
        match rx.try_recv() {
            Ok(x) => {
                blinky_status = x;
            }
            Err(_) => {}
        }

        // blinky if the button was pressed
        if blinky_status {
            match channel.set_duty(0) {
                Ok(_x) => (),
                Err(e) => println!("err setting duty of led: {e}\n"),
            }
            println!("LED ON");
            thread::sleep(Duration::from_millis(1000));

            match channel.set_duty(max_duty) {
                Ok(_x) => (),
                Err(e) => println!("err setting duty of led: {e}\n"),
            }
            println!("LED OFF");
            thread::sleep(Duration::from_millis(1000));
        } else {
            let duty = adc_mutex.load() as u32;
            let pwm = (duty as u32 * max_duty) / ADC_MAX_COUNTS;
            match channel.set_duty(pwm) {
                Ok(_x) => (),
                Err(e) => println!("err setting duty of led: {e}\n"),
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

pub fn button_thread(btn_pin: PinDriver<AnyIOPin, Input>, tx: crossbeam_channel::Sender<bool>) {
    let mut btn_status = false;

    loop {
        if btn_pin.is_high() {
            if !btn_status {
                btn_status = true;
                println!("BUTTON ON");
                tx.send(btn_status).unwrap();
            }
        } else {
            if btn_status {
                btn_status = false;
                println!("BUTTON OFF");
                tx.send(btn_status).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
