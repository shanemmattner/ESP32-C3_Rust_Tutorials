use anyhow::Result;
use dht11_esp32::DHT11;
use esp_idf_hal::{
    delay::{Ets, FreeRtos},
    gpio::*,
    prelude::Peripherals,
};
use tracing::{event, Level};

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    tracing_subscriber::fmt::init();

    let peripherals = Peripherals::take().unwrap();

    //let dht11_pin = PinDriver::input_output(peripherals.pins.gpio5.downgrade()).unwrap();
    //let dht11_pin = PinDriver::input_output_od(peripherals.pins.gpio5.downgrade()).unwrap();

    let mut dht11 = DHT11::new(peripherals.pins.gpio5)?;

    //let _dht11_thread = std::thread::Builder::new()
    //    .stack_size(5000)
    //    .spawn(move || dht11_thread_function(dht11_pin))
    //    .unwrap();

    loop {
        event!(Level::DEBUG, "starting mesurements");

        let measurement = dht11.measure()?;
        event!(Level::INFO, %measurement);
        FreeRtos::delay_ms(5000);
    }
}

// Thread function that will blink the LED on/off every 500ms
fn dht11_thread_function(mut pin: PinDriver<AnyIOPin, InputOutput>) {
    // Set pin high and wait 1 second before trying to read the DHT11
    pin.set_high().unwrap();
    FreeRtos::delay_ms(1000);

    loop {
        println!("Trying to read DHT11\n");
        // attempt to read the dht11
        //
        // 1. MCU sets pin low for >18ms
        pin.set_low().unwrap();
        FreeRtos::delay_ms(20);

        // 2. MCU pull voltage up
        pin.set_high().unwrap();
        Ets::delay_us(40);

        //3. DHT sends a low voltage response for 80us

        //4. DHT sets data bus from low to high and keeps it there for 80uSec in preparation to
        //   send data

        //5. DHT pulls up voltage and keeps it there for 80uSec in preparation for data transmit

        //6. Read the bits:
        //
        //6a. 50uSec low voltage
        //6b. 0 or 1 bit determined by length of pin being high
        //    26-28us = 0
        //    70us    = 1
        //
        //
        FreeRtos::delay_ms(5000);
    }
}

