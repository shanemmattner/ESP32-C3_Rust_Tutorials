use esp_idf_hal::{
    delay::{FreeRtos, NON_BLOCK},
    gpio,
    peripherals::Peripherals,
    prelude::*,
    uart::*,
};
use esp_idf_sys as _;

const CR: u8 = 13;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio21;
    let rx = peripherals.pins.gpio20;

    let config = config::Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )
    .unwrap();

    let mut cli_buf: Vec<u8> = Vec::new();

    loop {
        let mut buf: [u8; 10] = [0; 10];
        match uart.read(&mut buf, NON_BLOCK) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let b = buf[0];
                    cli_buf.push(b);
                    if b == CR {
                        match uart.write(&cli_buf) {
                            Ok(_) => println!("{:?} written", cli_buf),
                            Err(_) => {}
                        }
                        cli_buf.clear();
                    }
                }
            }
            Err(_) => {}
        }
        FreeRtos::delay_ms(100);
    }
}
