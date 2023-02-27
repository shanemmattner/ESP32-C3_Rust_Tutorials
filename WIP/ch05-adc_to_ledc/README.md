In this chapter we will be using [Crossbeam AtomicCell](https://docs.rs/crossbeam/latest/crossbeam/atomic/struct.AtomicCell.html) to share data between threads.

We will create a thread to read an [analog to digital](https://www.electronics-tutorials.ws/combination/analogue-to-digital-converter.html) reading, then the LED thread will take that reading and
use it to adjust the brightness of the LED by controlling the PWM.

In order to control the PWM of the LED we will need to use the ESP32-C3 [led controller](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/peripherals/ledc.html).

