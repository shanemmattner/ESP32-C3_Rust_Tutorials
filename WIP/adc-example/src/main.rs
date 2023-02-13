use embedded_hal::adc::OneShot;
use esp_idf_hal::{
    adc::{self, Atten11dB, PoweredAdc},
    gpio,
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::{thread, time::Duration};
static BLINKY_STACK_SIZE: usize = 5000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let config = adc::config::Config::new().calibration(true);

    // let mut adc1_ch0 = peripherals.pins.gpio0.into_analog_atten_11db().unwrap();
    // let mut adc1_ch1 = peripherals.pins.gpio1.into_analog_atten_11db().unwrap();
    // let mut adc1_ch2 = peripherals.pins.gpio2.into_analog_atten_11db().unwrap();
    // let mut adc1_ch3 = peripherals.pins.gpio3.into_analog_atten_11db().unwrap();
    // let mut adc1_ch4 = peripherals.pins.gpio4.into_analog_atten_11db().unwrap();
    let mut adc2_ch0 = peripherals.pins.gpio5.into_analog_atten_11db().unwrap();

    // let mut adc1 = PoweredAdc::new(peripherals.adc1, config).unwrap();
    let mut adc2 = PoweredAdc::new(peripherals.adc2, config).unwrap();

    let _adc_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || loop {
            // let mut adc_reading = adc1.read(&mut adc1_ch0).unwrap();
            // println!("ADC1_0: {}", adc_reading);
            // adc_reading = adc1.read(&mut adc1_ch1).unwrap();
            // println!("ADC1_1: {}", adc_reading);
            // adc_reading = adc1.read(&mut adc1_ch2).unwrap();
            // println!("ADC1_2: {}", adc_reading);
            // adc_reading = adc1.read(&mut adc1_ch3).unwrap();
            // println!("ADC1_3: {}", adc_reading);
            // adc_reading = adc1.read(&mut adc1_ch4).unwrap();
            // println!("ADC1_4: {}", adc_reading);
            let mut adc_reading = adc2.read(&mut adc2_ch0).unwrap();
            println!("ADC2_0: {}", adc_reading);

            thread::sleep(Duration::from_millis(100));
        })
        .unwrap();
}
