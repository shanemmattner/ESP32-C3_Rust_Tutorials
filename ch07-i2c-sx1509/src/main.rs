use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use sx1509;

const SSD1306_ADDRESS: u8 = 0x3e;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio1;

    println!("Starting I2C SSD1306 test");

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let mut expander = sx1509::Sx1509::new(&mut i2c, sx1509::DEFAULT_ADDRESS);
    expander.borrow(&mut i2c).software_reset().unwrap();
    expander.borrow(&mut i2c).set_bank_a_direction(0).unwrap();
    expander.borrow(&mut i2c).set_bank_a_pullup(0xFF).unwrap();

    expander
        .borrow(&mut i2c)
        .set_bank_b_direction(0xFF)
        .unwrap();
    i2c.write(
        SSD1306_ADDRESS,
        &[sx1509::Register::RegPullDownB as u8],
        0xFF,
    )
    .unwrap();

    loop {
        let pins = expander.borrow(&mut i2c).get_bank_a_data().unwrap();
        println!("bank a: {pins}");

        expander.borrow(&mut i2c).set_bank_b_data(pins).unwrap();

        FreeRtos::delay_ms(100);
    }
}