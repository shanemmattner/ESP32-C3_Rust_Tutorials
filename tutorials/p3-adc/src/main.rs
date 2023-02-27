use esp_idf_hal::{
    adc::{self, *},
    delay::FreeRtos,
    peripherals::Peripherals,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!(
        "Starting 3-adc\nThis application reads 4 ADC pins and prints the values every 1 second.\n"
    );

    let peripherals = Peripherals::take().unwrap();

    // ADC init
    let mut adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();
    let mut a1_ch0 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();
    let mut a1_ch2 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio2).unwrap();
    let mut a1_ch3 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio3).unwrap();
    let mut a1_ch4 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio4).unwrap();

    loop {
        match adc1.read(&mut a1_ch0) {
            Ok(x) => println!("A1_CH0: {x}\n"),
            Err(e) => println!("err reading A1_CH0: {e}\n"),
        }
        match adc1.read(&mut a1_ch2) {
            Ok(x) => println!("A1_CH2: {x}\n"),
            Err(e) => println!("err reading A1_CH2: {e}\n"),
        }
        match adc1.read(&mut a1_ch3) {
            Ok(x) => println!("A1_CH3: {x}\n"),
            Err(e) => println!("err reading A1_CH3: {e}\n"),
        }
        match adc1.read(&mut a1_ch4) {
            Ok(x) => println!("A1_CH4: {x}\n"),
            Err(e) => println!("err reading A1_CH4: {e}\n"),
        }
        println!("\n");
        FreeRtos::delay_ms(1000);
    }
}
