use crossbeam_channel::bounded;
use crossbeam_utils::atomic::AtomicCell;
use esp_idf_hal::{
    adc::{self, *},
    gpio::{ADCPin, AnyIOPin, IOPin, Input, PinDriver, Pull},
    ledc::{config::TimerConfig, *},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use std::{sync::Arc, thread, time::Duration};

static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;
static ADC_STACK_SIZE: usize = 2000;

static ADC_MAX_COUNTS: u32 = 2850;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut btn_pin = PinDriver::input(peripherals.pins.gpio6.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    // Crossbeam channel
    let (tx, rx) = bounded(1);

    // ADC init
    // Create atomic to store adc readings
    let atomic_value: AtomicCell<u16> = AtomicCell::new(0);
    let arc_data = Arc::new(atomic_value);
    // Create ADC channel driver
    let a1_ch0 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();
    let adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    let timer_config = TimerConfig::new().frequency(25.kHz().into());
    let timer = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config).unwrap());
    let channel_0 = LedcDriver::new(
        peripherals.ledc.channel0,
        timer.clone(),
        peripherals.pins.gpio8,
    )
    .unwrap();
    let a1 = arc_data.clone();
    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_thread(rx, a1, channel_0))
        .unwrap();

    let _button_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || button_thread(btn_pin, tx))
        .unwrap();

    let a2 = arc_data.clone();
    let _adc_thread = std::thread::Builder::new()
        .stack_size(ADC_STACK_SIZE)
        .spawn(move || adc_thread(a2, adc1, a1_ch0))
        .unwrap();
}

fn adc_thread<T: ADCPin>(
    adc_mutex: Arc<AtomicCell<u16>>,
    mut adc: AdcDriver<adc::ADC1>,
    mut pin: adc::AdcChannelDriver<T, adc::Atten11dB<adc::ADC1>>,
) where
    Atten11dB<ADC1>: Attenuation<<T as ADCPin>::Adc>,
{
    loop {
        // Read ADC and and set the LED PWM to the percentage of full scale
        match adc.read(&mut pin) {
            Ok(x) => {
                adc_mutex.store(x);
            }

            Err(e) => println!("err reading ADC: {e}\n"),
        }

        thread::sleep(Duration::from_millis(100));
    }
}

// Thread function that will blink the LED on/off every 500ms
fn blinky_thread(
    rx: crossbeam_channel::Receiver<bool>,
    adc_mutex: Arc<AtomicCell<u16>>,
    mut channel: LedcDriver<'_>,
) {
    let mut blinky_status = false;
    let max_duty = channel.get_max_duty();
    loop {
        // Watch for button press messages
        match rx.try_recv() {
            Ok(x) => {
                blinky_status = x;
            }
            Err(_) => {}
        }

        // blinky if the button was pressed
        if blinky_status {
            match channel.set_duty(0) {
                Ok(_x) => (),
                Err(e) => println!("err setting duty of led: {e}\n"),
            }
            println!("LED ON");
            thread::sleep(Duration::from_millis(1000));

            match channel.set_duty(max_duty) {
                Ok(_x) => (),
                Err(e) => println!("err setting duty of led: {e}\n"),
            }
            println!("LED OFF");
            thread::sleep(Duration::from_millis(1000));
        } else {
            let duty = adc_mutex.load() as u32;
            let pwm = (duty as u32 * max_duty) / ADC_MAX_COUNTS;
            match channel.set_duty(pwm) {
                Ok(_x) => (),
                Err(e) => println!("err setting duty of led: {e}\n"),
            }
        }

        thread::sleep(Duration::from_millis(100));
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
        thread::sleep(Duration::from_millis(100));
    }
}
