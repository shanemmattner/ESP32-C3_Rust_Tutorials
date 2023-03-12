use esp_idf_hal::{delay::FreeRtos, adc::{self,*}, gpio::{*,self,AnyIOPin, AnyOutputPin, IOPin, Input, Output, OutputPin, PinDriver, Pull, InputOutput}, peripherals::{Peripherals, self}, prelude::*, uart};

// #[derive(Debug)]
pub struct adc_stream<'a> {
     pub adc: AdcDriver<'a, adc::ADC1>,
     pub a1_ch0: adc::AdcChannelDriver<'a, Gpio0, adc::Atten11dB<adc::ADC1>>,
     pub a1_ch2: adc::AdcChannelDriver<'a, Gpio2, adc::Atten11dB<adc::ADC1>>,
     pub a1_ch3: adc::AdcChannelDriver<'a, Gpio3, adc::Atten11dB<adc::ADC1>>,
     pub a1_ch4: adc::AdcChannelDriver<'a, Gpio4, adc::Atten11dB<adc::ADC1>>,
}

