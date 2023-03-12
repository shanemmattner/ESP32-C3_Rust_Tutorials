use esp_idf_hal::{
    delay::FreeRtos, 
    adc::{self,*}, 
    gpio::*,
    };


pub enum AdcChannel{
    A1CH0,
    A1CH2,
    A1CH3,
    A1CH4,
}
// #[derive(Debug)]
pub struct AdcStream<'a> {
     pub adc: AdcDriver<'a, adc::ADC1>,
     pub a1_ch0: adc::AdcChannelDriver<'a, Gpio0, adc::Atten11dB<adc::ADC1>>,
     pub a1_ch2: adc::AdcChannelDriver<'a, Gpio2, adc::Atten11dB<adc::ADC1>>,
     pub a1_ch3: adc::AdcChannelDriver<'a, Gpio3, adc::Atten11dB<adc::ADC1>>,
     pub a1_ch4: adc::AdcChannelDriver<'a, Gpio4, adc::Atten11dB<adc::ADC1>>,
}

impl AdcStream<'_>{
    pub fn read(&mut self, channel: AdcChannel) -> u16{
        let mut ret:u16 = 0;
        match channel{
            AdcChannel::A1CH0 => {
                match self.adc.read(&mut self.a1_ch0)
                {
                    Ok(x) => {
                        println!("A1CH0: {x}\n");
                        ret = x;
                    }
            
                    Err(e) => println!("err reading A1CH0: {:?}\n",e),
                }
            }
            AdcChannel::A1CH2 => {
                match self.adc.read(&mut self.a1_ch2)
                {
                    Ok(x) => {
                        println!("A1CH2: {x}\n");
                        ret = x;
                    }
            
                    Err(e) => println!("err reading A1CH2: {:?}\n",e),
                }
            }
            AdcChannel::A1CH3 => {
                match self.adc.read(&mut self.a1_ch3)
                {
                    Ok(x) => {
                        println!("A1CH3: {x}\n");
                        ret = x;
                    }
                    Err(e) => println!("err reading A1CH3: {:?}\n",e),
                }
            }
            AdcChannel::A1CH4 => {
                match self.adc.read(&mut self.a1_ch4)
                {
                    Ok(x) => {
                        println!("A1CH4: {x}\n");
                        ret = x;
                    }
                    Err(e) => println!("err reading A1CH4: {:?}\n",e),
                }
            }
        }
        ret

    }
}

pub fn adc_thread(mut adc_stream : AdcStream)
{

    loop{
        adc_stream.read(AdcChannel::A1CH0);
        adc_stream.read(AdcChannel::A1CH2);
        adc_stream.read(AdcChannel::A1CH3);
        adc_stream.read(AdcChannel::A1CH4);
        FreeRtos::delay_ms(1000);
    }
}