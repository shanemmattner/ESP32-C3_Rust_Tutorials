use embedded_sdmmc::*;
use esp_idf_hal::{
    adc::{self, *},
    delay::FreeRtos,
    gpio::*,
    peripherals::Peripherals,
    prelude::*,
    spi::config::Duplex,
    spi::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
use std::time::*;

const FILE_TO_CREATE: &'static str = "logs.txt";

pub struct SdMmcClock;

impl TimeSource for SdMmcClock {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Starting 6-spi\nThis application writes ADC readings to a micro-SD card\n");

    let peripherals = Peripherals::take().unwrap();

    // ADC init
    let mut adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();
    let mut a1_ch0 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();

    // SPI sd card init
    let driver = SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio8,       // SCK
        peripherals.pins.gpio7,       // MOSI
        Some(peripherals.pins.gpio9), // MISO
        Dma::Disabled,
    )
    .unwrap();

    let mut spi_config = SpiConfig::new();
    spi_config.duplex = Duplex::Full;
    let _ = spi_config.baudrate(24.MHz().into());
    let spi = SpiDeviceDriver::new(driver, Option::<Gpio6>::None, &spi_config).unwrap();
    let sdmmc_cs = PinDriver::output(peripherals.pins.gpio6).unwrap();
    let mut sdmmc_spi = embedded_sdmmc::SdMmcSpi::new(spi, sdmmc_cs);

    // keep track of the time for logging
    let start_time = Instant::now();

    loop {
        match adc1.read(&mut a1_ch0) {
            Ok(adc_reading) => {
                match sdmmc_spi.acquire() {
                    Ok(block) => {
                        let mut controller = embedded_sdmmc::Controller::new(block, SdMmcClock);

                        let mut volume =
                            controller.get_volume(embedded_sdmmc::VolumeIdx(0)).unwrap();

                        let root_dir = controller.open_root_dir(&volume).unwrap();

                        let mut f = controller
                            .open_file_in_dir(
                                &mut volume,
                                &root_dir,
                                FILE_TO_CREATE,
                                Mode::ReadWriteCreateOrAppend,
                            )
                            .unwrap();

                        f.seek_from_end(0).unwrap();
                        // Create log string
                        let mut log_string: String = start_time.elapsed().as_millis().to_string();
                        log_string.push_str(&",A1_CH0,".to_string());
                        log_string.push_str(&(adc_reading as u32).to_string());
                        log_string.push('\n');
                        // Write the string to the SD card
                        let _bytes_written = controller
                            .write(&mut volume, &mut f, &log_string.as_bytes()[..])
                            .unwrap();
                        println!("String written: {log_string}");

                        controller.close_file(&volume, f).unwrap();
                    }
                    Err(e) => println!("Error acquire SPI bus {:?}", e),
                };
            }
            Err(e) => println!("err reading A1_CH0: {e}\n"),
        }

        FreeRtos::delay_ms(500);
    }
}
