use esp_idf_hal::{
    delay::{FreeRtos, NON_BLOCK},
    gpio,
    peripherals::Peripherals,
    prelude::*,
    uart::*,
};
use esp_idf_sys as _;

fn main() -> anyhow::Result<()> {
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

    let mut uart_buf: Vec<u8> = Vec::new();

    loop {
        let mut buf: [u8; 10] = [0; 10];
        match uart.read(&mut buf, NON_BLOCK) {
            Ok(x) => {
                if x > 0 {
                    uart_buf.push(buf[0]);
                    if uart_buf[uart_buf.len() - 1] == 13 {
                        match uart.write(&uart_buf) {
                            Ok(_) => println!("{:?} written", buf),
                            Err(_) => {}
                        }
                        uart_buf.clear();
                    }
                }
            }
            Err(_) => {}
        }
        FreeRtos::delay_ms(100);
    }
}
