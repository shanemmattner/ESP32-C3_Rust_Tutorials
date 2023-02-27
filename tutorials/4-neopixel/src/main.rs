mod neopixel;

use esp_idf_hal::{
    adc::{self, *},
    delay::FreeRtos,
    peripherals::Peripherals,
    rmt::{config::TransmitConfig, *},
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;

const ADC_MAX: u16 = 2800;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Starting 4-neopixel\n");

    let peripherals = Peripherals::take().unwrap();

    // ADC init
    let mut adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();
    let mut a1_ch0 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();

    let led = peripherals.pins.gpio5;
    let channel = peripherals.rmt.channel0;
    let config = TransmitConfig::new().clock_divider(1);
    let mut tx = TxRmtDriver::new(channel, led, &config).unwrap();

    neopixel::neopixel(
        neopixel::RGB {
            r: 25,
            g: 25,
            b: 25,
        },
        &mut tx,
    )
    .unwrap();

    loop {
        match adc1.read(&mut a1_ch0) {
            Ok(x) => {
                println!("A1_CH0: {x}\n");
                let mut prcnt: f32 = x as f32 / ADC_MAX as f32;
                // make sure we don't go over 100%
                prcnt = if prcnt > 1.0 { 1.0 } else { prcnt };
                let rgb = neopixel::hsv2rgb((360.0 as f32 * prcnt) as u32, 100, 20).unwrap();

                neopixel::neopixel(rgb, &mut tx).unwrap();
            }
            Err(e) => println!("err reading A1_CH0: {e}\n"),
        }

        println!("\n");
        FreeRtos::delay_ms(100);
    }
}
