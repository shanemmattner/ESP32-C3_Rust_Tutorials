use anyhow::{ensure, Context, Result};
use esp_idf_hal::{
    delay::Ets,
    gpio::{InputOutput, InputPin, Level, OutputPin, PinDriver},
};
use std::fmt::Display;

pub const TIMEOUT_US: u16 = 1_000;

pub struct DHT11<'d, P: InputPin + OutputPin>(PinDriver<'d, P, InputOutput>);

impl<'d, P> DHT11<'d, P>
where
    P: InputPin + OutputPin,
{
    pub fn new(gpio: P) -> Result<Self> {
        Ok(DHT11(PinDriver::input_output(gpio)?))
    }

    pub fn measure(&mut self) -> Result<Measurement> {
        self.handshake().context("handshake error")?;

        let mut data = [0u8; 5];

        for i in 0..40 {
            data[i / 8] <<= 1;
            if self.read_bit()? {
                data[i / 8] |= 1;
            }
        }

        // checksum
        ensure!(
            data[0] + data[1] + data[2] + data[3] == data[4],
            "checksum error"
        );

        let humidity = f32::from(data[0]) + f32::from(data[1]) / 10.;
        let temperature = f32::from(data[2]) + f32::from(data[3]) / 10.;

        Ok(Measurement {
            humidity,
            temperature,
        })
    }

    fn handshake(&mut self) -> Result<()> {
        self.0.set_low()?;
        Ets::delay_ms(18);

        self.0.set_high()?;
        Ets::delay_us(40);

        self.read_bit()?;

        Ok(())
    }

    fn read_bit(&self) -> Result<bool> {
        let low = self.read_level_time(Level::Low)?;
        let high = self.read_level_time(Level::High)?;
        Ok(high > low)
    }

    fn read_level_time(&self, level: Level) -> Result<u16> {
        let mut count = 0;
        while self.0.get_level() == level {
            Ets::delay_us(1);
            count += 1;

            ensure!(count < TIMEOUT_US, "timeout");
        }
        Ok(count)
    }
}

#[derive(Debug)]
pub struct Measurement {
    pub humidity: f32,
    pub temperature: f32,
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "temperature: {}℃, humidity: {}%RH",
            self.temperature, self.humidity
        )
    }
}

