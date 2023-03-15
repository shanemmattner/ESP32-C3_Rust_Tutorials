mod cli;
mod adc_thread;

use esp_idf_hal::{
    delay::FreeRtos, 
    adc, 
    gpio,
    peripherals::Peripherals, 
    prelude::*, 
    uart};
use esp_idf_sys as _;
use crossbeam_utils::atomic::AtomicCell;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

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


    let atomic_value: AtomicCell<[u16;4]> = AtomicCell::new([0;4]);
    let arc_data = Arc::new(atomic_value);
    let arc_msgs = Arc::new(ArrayQueue::<String>::new(100));


    // UART and CLI initialization
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

    let arc_data_1 = arc_data.clone();
    let arc_msgs_1 = arc_msgs.clone();

    let _cli_thread = std::thread::Builder::new()
        .stack_size(CLI_STACK_SIZE)
        .spawn(move || cli::uart_thread(uart, arc_data_1, arc_msgs_1))
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

    let arc_data_2 = arc_data.clone();
    let arc_msgs_2 = arc_msgs.clone();

    let adc_streamer = adc_thread::AdcStream {
        adc :  adc1,
        a1_ch0 :  a1_ch0,
        a1_ch2 :  a1_ch2,
        a1_ch3 :  a1_ch3,
        a1_ch4 :  a1_ch4,
        adc_atomic: arc_data_2,
        cli_msgs : arc_msgs_2,
    };

    let _adc_thread = std::thread::Builder::new()
        .stack_size(ADC_STACK_SIZE)
        .spawn(move || adc_thread::adc_thread(adc_streamer))
        .unwrap();
    
}
