mod cli;
mod adc_driver;


use esp_idf_hal::{
    delay::FreeRtos, 
    adc, 
    gpio,
    peripherals::Peripherals, 
    prelude::*, 
    uart};
use esp_idf_sys as _;

static CLI_STACK_SIZE: usize = 5000;
static ADC_STACK_SIZE: usize = 5000;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    initialization(peripherals);

    loop {

        FreeRtos::delay_ms(1000);
 
    }
}

fn initialization(peripherals : Peripherals){
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

    // Create ADC channel driver
    let a1_ch0 =
    adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();
    let a1_ch2 =
    adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio2).unwrap();
    let a1_ch3 =
    adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio3).unwrap();
    let a1_ch4 =
    adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio4).unwrap();
   
    let adc1 = adc::AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    let adc_streamer = adc_driver::AdcStream {
        adc :  adc1,
        a1_ch0 :  a1_ch0,
        a1_ch2 :  a1_ch2,
        a1_ch3 :  a1_ch3,
        a1_ch4 :  a1_ch4,
    };

    let _adc_thread = std::thread::Builder::new()
        .stack_size(ADC_STACK_SIZE)
        .spawn(move || adc_driver::adc_thread(adc_streamer))
        .unwrap();
    
}
