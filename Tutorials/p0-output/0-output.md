# 0 - Output

## Goal
Make simple blinky program

## Topics covered
- [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal)
- [Peripheral Singleton](https://docs.rust-embedded.org/book/peripherals/singletons.html)
- Configure GPIO as [output](https://esp-rs.github.io/esp-idf-hal/esp_idf_hal/gpio/struct.PinDriver.html#method.output)
- [FreeRTOS delay](https://esp-rs.github.io/esp-idf-hal/esp_idf_hal/delay/struct.FreeRtos.html)
- [esp-println](https://github.com/esp-rs/esp-println)


## ESP-IDF-HAL
Espressif's Rust implementation for the `ESP-idf` that uses [Rust's STD support](https://esp-rs.github.io/book/overview/using-the-standard-library.html)

## Singleton
<b>Singleton</b>: software design pattern that restricts the instantiation of a class to one object.

Why have a Singleton for peripherals?
There are two important factors in play here:

"
- Because we are using a singleton, there is only one way or place to obtain a SerialPort structure
- To call the read_speed() method, we must have ownership or a reference to a SerialPort structure

These two factors put together means that it is only possible to access the hardware if we have appropriately satisfied the borrow checker, meaning that at no point do we have multiple mutable references to the same hardware!
"

## Delays
Note that we will need to use different delays depending on how long we want to delay for:

- `Ets`:	Espressif built-in delay provider Use only for very small delays (us or a few ms at most), or else the FreeRTOS IDLE tasks’ might starve and the IDLE tasks’ watchdog will trigger
- `FreeRtos`:	FreeRTOS-based delay provider Use for delays larger than 10ms (delays smaller than 10ms used in a loop would starve the FreeRTOS IDLE tasks’ as they are low prio tasks and hence the the IDLE tasks’ watchdog will trigger)

