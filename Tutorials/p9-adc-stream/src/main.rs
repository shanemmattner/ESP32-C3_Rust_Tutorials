mod cli;
mod adc_driver;


use esp_idf_hal::{delay::FreeRtos, adc, gpio::{self,AnyIOPin, AnyOutputPin, IOPin, Input, Output, OutputPin, PinDriver, Pull}, peripherals::Peripherals, prelude::*, uart};
use esp_idf_sys as _;

static CLI_STACK_SIZE: usize = 5000;
static ADC_STACK_SIZE: usize = 5000;

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


    // Create ADC channel driver
    let a1_ch0 =
        adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();
    let adc1 = adc::AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();

    // print_type_of(&peripherals.pins.gpio0);

    let mut test = adc_driver::adc_stream_t {
        adc :  adc1,
        pin :  a1_ch0
    };
    match test.adc.read(&mut test.pin)
    {
        Ok(x) => {
            println!("reading: {x}\n");
        }

        Err(e) => println!("err reading ADC: {e}\n"),
        }
    // adc_driver::adc_init(adc1,a1_ch0);
    // let mut a1_ch0 =
    //     adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio0).unwrap();
    // let mut a1_ch2 =
    // adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio2).unwrap();
    // let mut a1_ch3 =
    // adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio3).unwrap();
    // let mut a1_ch4 =
    // adc::AdcChannelDriver::<_, adc::Atten11dB<adc::ADC1>>::new(peripherals.pins.gpio4).unwrap();

    
    loop {
        FreeRtos::delay_ms(100);
    }
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}