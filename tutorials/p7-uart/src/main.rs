use esp_idf_hal::{delay::FreeRtos, gpio, peripherals::Peripherals, prelude::*, uart::*};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::fmt::Write;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio21;
    let rx = peripherals.pins.gpio20;

    println!("Starting UART loopback test");
    let config = config::Config::new().baudrate(Hertz(115_200));
    let mut uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )
    .unwrap();

    loop {
        for i in 0..10 {
            write!(uart, "{i}").unwrap();
        }
        write!(uart, "\n").unwrap();

        // let mut buf: [u8; 4] = [0; 4];
        // let mut x = uart.read(&mut buf, 1000).unwrap();
        // while x > 0 {
        //     uart.write(&[buf[x - 1]]).unwrap();
        //     x -= 1;
        // }
        FreeRtos::delay_ms(1000);
    }
}
