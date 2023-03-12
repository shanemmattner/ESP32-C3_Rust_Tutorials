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

    print_type_of(&a1_ch0);
    print_type_of(&a1_ch2);
    print_type_of(&a1_ch3);
    print_type_of(&a1_ch4);
    // print_type_of(&peripherals.pins.gpio0);

    let mut test = adc_driver::adc_stream {
        adc :  adc1,
        a1_ch0 :  a1_ch0,
        a1_ch2 :  a1_ch2,
        a1_ch3 :  a1_ch3,
        a1_ch4 :  a1_ch4,
    };

    
    loop {
        FreeRtos::delay_ms(1000);
        match test.adc.read(&mut test.a1_ch0)
        {
            Ok(x) => {
                println!("a1_ch0: {x}\n");
            }
    
            Err(e) => println!("err reading a1_ch0: {e}\n"),
        }
        match test.adc.read(&mut test.a1_ch2)
        {
            Ok(x) => {
                println!("a1_ch2: {x}\n");
            }
    
            Err(e) => println!("err reading a1_ch2: {e}\n"),
        }
        match test.adc.read(&mut test.a1_ch3)
        {
            Ok(x) => {
                println!("a1_ch3: {x}\n");
            }
    
            Err(e) => println!("err reading a1_ch3: {e}\n"),
        }
        match test.adc.read(&mut test.a1_ch4)
        {
            Ok(x) => {
                println!("a1_ch4: {x}\n");
            }
    
            Err(e) => println!("err reading a1_ch4: {e}\n"),
        }
    }
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}