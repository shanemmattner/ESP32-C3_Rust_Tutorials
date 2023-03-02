use arrayvec::ArrayVec;
use esp_idf_hal::{
    delay::{FreeRtos, NON_BLOCK},
    gpio,
    peripherals::Peripherals,
    prelude::*,
    uart::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio21;
    let rx = peripherals.pins.gpio20;

    println!("Starting UART loopback test");
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
        let mut buf: [u8; 100] = [0; 100];
        match uart.read(&mut buf, NON_BLOCK) {
            Ok(x) => {
                if x > 0 {
                    uart_buf.push(buf[0]);
                }
            }
            Err(_) => {}
        }

        if uart_buf.len() > 0 {
            if uart_buf[uart_buf.len() - 1] == 13 {
                println!("{:?}", uart_buf);
                uart_buf.clear();
                // let send_buf = uart_buf.to_owned();
                // let array: ArrayVec<_, 100> = send_buf.into_iter().collect();
                // let array: [u8; 100] = array.into_inner().unwrap();
                // uart.write(&array).unwrap();
            }
        }
        // append bytes read to uart_buf

        // look for '\n' character, if found print out the string so far, if not append current
        // byte to buffer

        FreeRtos::delay_ms(10);
    }
}
