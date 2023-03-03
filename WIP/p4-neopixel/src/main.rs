use anyhow::{bail, Result};
use esp_idf_hal::{
    adc::{self, *},
    delay::FreeRtos,
    peripherals::Peripherals,
    rmt::{config::TransmitConfig, *},
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use std::time::Duration;

const ADC_MAX: u16 = 2800;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Starting 4-neopixel\nThis application reads an ADC pin and changes the color of a neopixel.\n");

    let peripherals = Peripherals::take().unwrap();

    // ADC init
    let mut adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();
    let mut a1_ch0 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();

    // Neopixel init
    let led = peripherals.pins.gpio5;
    let channel = peripherals.rmt.channel0;
    let config = TransmitConfig::new().clock_divider(1);
    let mut tx = TxRmtDriver::new(channel, led, &config).unwrap();

    neopixel(
        RGB {
            r: 25,
            g: 25,
            b: 25,
        },
        &mut tx,
    )
    .unwrap();

    loop {
        match adc1.read(&mut a1_ch0) {
            Ok(x) => {
                println!("A1_CH0: {x}\n");
                let mut prcnt: f32 = x as f32 / ADC_MAX as f32;
                // make sure we don't go over 100%
                prcnt = if prcnt > 1.0 { 1.0 } else { prcnt };
                let rgb = hsv2rgb((360.0 as f32 * prcnt) as u32, 100, 20).unwrap();

                neopixel(rgb, &mut tx).unwrap();
            }
            Err(e) => println!("err reading A1_CH0: {e}\n"),
        }

        println!("\n");
        FreeRtos::delay_ms(100);
    }
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}

pub fn neopixel(rgb: RGB, tx: &mut TxRmtDriver) -> Result<()> {
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
pub fn hsv2rgb(h: u32, s: u32, v: u32) -> Result<RGB> {
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
