use anyhow::{bail, Result};
use crossbeam_channel::bounded;
use crossbeam_utils::atomic::AtomicCell;
use esp_idf_hal::{
    adc::{self, *},
    delay::FreeRtos,
    gpio::{ADCPin, AnyIOPin, IOPin, Input, PinDriver, Pull},
    ledc::{config::TimerConfig, *},
    peripherals::Peripherals,
    prelude::*,
    rmt::{config::TransmitConfig, *},
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

    let led = peripherals.pins.gpio5;
    let channel = peripherals.rmt.channel0;
    let config = TransmitConfig::new().clock_divider(1);
    let mut tx = TxRmtDriver::new(channel, led, &config).unwrap();

    // 3 seconds white at 10% brightness
    neopixel(
        RGB {
            r: 25,
            g: 25,
            b: 25,
        },
        &mut tx,
    )
    .unwrap();
    FreeRtos::delay_ms(3000);

    // rainbow loop at 20% brightness
    let mut i: u32 = 0;
    loop {
        let rgb = hsv2rgb(i, 100, 20).unwrap();
        neopixel(rgb, &mut tx).unwrap();
        if i == 360 {
            i = 0;
        }
        i += 1;
        FreeRtos::delay_ms(10);
    }
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

struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}

fn neopixel(rgb: RGB, tx: &mut TxRmtDriver) -> Result<()> {
    // e.g. rgb: (1,2,4)
    // G        R        B
    // 7      0 7      0 7      0
    // 00000010 00000001 00000100
    let color: u32 = ((rgb.g as u32) << 16) | ((rgb.r as u32) << 8) | rgb.b as u32;
    let ticks_hz = tx.counter_clock().unwrap();
    let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(350)).unwrap();
    let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(800)).unwrap();
    let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(700)).unwrap();
    let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(600)).unwrap();
    let mut signal = FixedLengthSignal::<24>::new();
    for i in (0..24).rev() {
        let p = 2_u32.pow(i);
        let bit = p & color != 0;
        let (high_pulse, low_pulse) = if bit { (t1h, t1l) } else { (t0h, t0l) };
        signal
            .set(23 - i as usize, &(high_pulse, low_pulse))
            .unwrap();
    }
    tx.start_blocking(&signal).unwrap();

    Ok(())
}

/// Converts hue, saturation, value to RGB
fn hsv2rgb(h: u32, s: u32, v: u32) -> Result<RGB> {
    if h > 360 || s > 100 || v > 100 {
        bail!("The given HSV values are not in valid range");
    }
    let s = s as f64 / 100.0;
    let v = v as f64 / 100.0;
    let c = s * v;
    let x = c * (1.0 - (((h as f64 / 60.0) % 2.0) - 1.0).abs());
    let m = v - c;
    let (r, g, b);
    if h < 60 {
        r = c;
        g = x;
        b = 0.0;
    } else if h >= 60 && h < 120 {
        r = x;
        g = c;
        b = 0.0;
    } else if h >= 120 && h < 180 {
        r = 0.0;
        g = c;
        b = x;
    } else if h >= 180 && h < 240 {
        r = 0.0;
        g = x;
        b = c;
    } else if h >= 240 && h < 300 {
        r = x;
        g = 0.0;
        b = c;
    } else {
        r = c;
        g = 0.0;
        b = x;
    }

    Ok(RGB {
        r: ((r + m) * 255.0) as u8,
        g: ((g + m) * 255.0) as u8,
        b: ((b + m) * 255.0) as u8,
    })
}
