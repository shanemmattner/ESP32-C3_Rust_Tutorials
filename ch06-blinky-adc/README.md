# Blinky ADC

In this example we'll use ADC values to blinky the LED at a certain PWM based on the ADC reading.

## setup debugging
https://docs.espressif.com/projects/esp-idf/en/v5.0/esp32c3/api-guides/jtag-debugging/index.html

## TODO
3. incorporate the led pwm into the current blinky state machine
  a. mutex for adc readings
  b. When it's time to blink look at the ADC reading and calculate a pwm

