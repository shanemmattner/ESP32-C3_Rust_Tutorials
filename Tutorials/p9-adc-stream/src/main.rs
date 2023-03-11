mod cli;

use esp_idf_hal::{delay::FreeRtos, adc, gpio::{self,AnyIOPin, AnyOutputPin, IOPin, Input, Output, OutputPin, PinDriver, Pull}, peripherals::Peripherals, prelude::*, uart};
use esp_idf_sys as _;

static CLI_STACK_SIZE: usize = 5000;
static ADC_STACK_SIZE: usize = 5000;
const ADC_CHANNEL_SIZE:usize = 4;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio21;
    let rx = peripherals.pins.gpio20;

    let config = uart::config::Config::new().baudrate(Hertz(115_200));
    let uart = uart::UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )
    .unwrap();

    let _cli_thread = std::thread::Builder::new()
        .stack_size(CLI_STACK_SIZE)
        .spawn(move || cli::uart_thread(uart))
        .unwrap();


    // ADC init
    let mut adc1 = adc::AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    let mut a1_ch0 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();
    let mut a1_ch2 =
    adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio2).unwrap();
    let mut a1_ch3 =
    adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio3).unwrap();
    let mut a1_ch4 =
    adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio4).unwrap();

    let adc_channels:[adc::AdcChannelDriver<_, adc::Atten11dB<adc::ADC1>>;ADC_CHANNEL_SIZE] = [a1_ch0,a1_ch2,a1_ch3,a1_ch4];
    
    loop {
        FreeRtos::delay_ms(100);
    }
}
