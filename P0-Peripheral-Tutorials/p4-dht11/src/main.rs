use dht11::Dht11;
use esp_idf_hal::{
    delay::{Ets, FreeRtos},
    gpio::*,
    prelude::Peripherals,
};

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let dht11_pin = PinDriver::input_output_od(peripherals.pins.gpio5.downgrade()).unwrap();

    let mut dht11 = Dht11::new(dht11_pin);

    loop {
        let mut dht11_delay = Ets;
        match dht11.perform_measurement(&mut dht11_delay) {
            Ok(measurement) => println!("{:?}", measurement),
            Err(e) => println!("{:?}", e),
        }
        FreeRtos::delay_ms(1000);
    }
}

