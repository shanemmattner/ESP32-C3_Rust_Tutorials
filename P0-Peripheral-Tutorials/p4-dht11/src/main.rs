use dht11::Dht11;
use esp_idf_hal::{
    delay::{Ets, FreeRtos},
    gpio::*,
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let dht11_pin = PinDriver::output_od(peripherals.pins.gpio5).unwrap();

    let mut dht11 = Dht11::new(dht11_pin);

    loop {
        println!("starting measurements");
        let mut delay = Ets;
        match dht11.perform_measurement(&mut delay) {
            Ok(measurement) => println!("measurement: {measurement}"),
            Err(e) => println!("err: {e}"),
        }

        FreeRtos::delay_ms(1000);
    }
}
