use anyhow::Result;
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
    let dht11_pin = PinDriver::output(peripherals.pins.gpio5.downgrade_output()).unwrap();

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(5000)
        .spawn(move || dht11_thread_function(dht11_pin))
        .unwrap();

    loop {
        //event!(Level::DEBUG, "starting mesurements");

        //  let measurement = dht11.measure()?;
        //let measurement = 0;
        //event!(Level::INFO, %measurement);
        FreeRtos::delay_ms(1000);
    }
}

// Thread function that will blink the LED on/off every 500ms
fn dht11_thread_function(mut led_pin: PinDriver<AnyOutputPin, Output>) {
    led_pin.set_low().unwrap();

    loop {
        // attempt to read the dht11
        //
        // 1. MCU sets pin low for >18ms
        //led_pin.set_low().unwrap();
        println!("LED ON");
        FreeRtos::delay_ms(1000);

        //led_pin.set_high().unwrap();
        println!("LED OFF");
        FreeRtos::delay_ms(1000);
    }
}

