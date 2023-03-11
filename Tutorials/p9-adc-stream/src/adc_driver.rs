use esp_idf_hal::{delay::FreeRtos, adc::{self,*}, gpio::{*,self,AnyIOPin, AnyOutputPin, IOPin, Input, Output, OutputPin, PinDriver, Pull, InputOutput}, peripherals::{Peripherals, self}, prelude::*, uart};
use esp_println::println;
use statig::prelude::*;

// #[derive(Debug)]
pub struct adc_stream_t<'a, T:ADCPin> {
     pub adc: AdcDriver<'a, adc::ADC1>,
     pub pin: adc::AdcChannelDriver<'a, T, adc::Atten11dB<adc::ADC1>>,
}



pub fn adc_init<T:ADCPin>(    mut adc: AdcDriver<adc::ADC1>,
    mut pin: adc::AdcChannelDriver<T, adc::Atten11dB<adc::ADC1>>,
) where
    Atten11dB<ADC1>: Attenuation<<T as ADCPin>::Adc>,
{
    match adc.read(&mut pin) {
        Ok(x) => {
            println!("reading: {x}\n");
        }

        Err(e) => println!("err reading ADC: {e}\n"),
    }
}