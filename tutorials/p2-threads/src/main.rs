use crossbeam_channel::bounded;
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{AnyIOPin, AnyOutputPin, IOPin, Input, Output, OutputPin, PinDriver, Pull},
    peripherals::Peripherals,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;

static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Starting 2-threads\nThis application separates the input button logic and blinky logic into their own threaads.");

    let peripherals = Peripherals::take().unwrap();
    let led_pin = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let mut btn_pin = PinDriver::input(peripherals.pins.gpio6.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    let (tx, rx) = bounded(1);

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_thread(led_pin, rx))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || button_thread(btn_pin, tx))
        .unwrap();
}

// Thread function that will blink the LED on/off every 500ms
fn blinky_thread(
    mut led_pin: PinDriver<AnyOutputPin, Output>,
    rx: crossbeam_channel::Receiver<bool>,
) {
    let mut blinky_status = false;
    loop {
        match rx.try_recv() {
            Ok(x) => blinky_status = x,
            Err(_) => {}
        }
        if blinky_status {
            led_pin.set_low().unwrap();
            println!("LED ON");
            FreeRtos::delay_ms(1000);

            led_pin.set_high().unwrap();
            println!("LED OFF");
        }
        FreeRtos::delay_ms(1000);
    }
}

fn button_thread(btn_pin: PinDriver<AnyIOPin, Input>, tx: crossbeam_channel::Sender<bool>) {
    let mut btn_status = false;

    loop {
        if btn_pin.is_high() {
            if !btn_status {
                btn_status = true;
                println!("BUTTON ON");
                tx.send(btn_status).unwrap();
            }
        } else {
            if btn_status {
                btn_status = false;
                println!("BUTTON OFF");
                tx.send(btn_status).unwrap();
            }
        }
        FreeRtos::delay_ms(100);
    }
}
