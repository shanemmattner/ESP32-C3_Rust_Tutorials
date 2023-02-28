# ESP32-C3 Rust Tutorials

This will be a series of `embedded Rust tutorials` using the [Olimex ESP32-C3 Dev kit](https://www.olimex.com/Products/IoT/ESP32-C3/ESP32-C3-DevKit-Lipo/open-source-hardware). 

We will start with a blanky project for each tutorial and implement a peripherals using the latest [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) version. After implementing various peripherals we will build a larger project: `ESP32-C3 Remote Data Logger`.

## Tutorials
Check out the `YouTue video` link for a guide on building each example
- [p0-output](https://youtu.be/vUSHaogHs1s): Make the "Hellow World" of embedded systems: a `Blinky application` to periodically turn an LED on and off
- [p1-input](https://youtu.be/2IY27b9TT2k): Add a button to turn the blinking logic on and off
- [p2-threads](https://youtu.be/ht5t39dEa4E): Move the button logic and LED logic to their own thread and pass messages between threads with [crossbeams channel](https://docs.rs/crossbeam/latest/crossbeam/channel/index.html)
- [p3-adc](https://youtu.be/07SVj6R_AhA): Read an [analog-to-digital converter channel](https://en.wikipedia.org/wiki/Analog-to-digital_converter) and print out the value
- [p4-neopixel](https://youtu.be/VIVML4cioIo): Use the ADC reading to change the color of a [neopixel](https://www.adafruit.com/category/168) 
- [p5-i2c](https://youtu.be/NvEnHJPpavo): Configure and use a [SX1509 GPIO Expander](https://www.sparkfun.com/products/13601) through [I2C](https://learn.sparkfun.com/tutorials/i2c)
- `p6-spi`: Read ADC values and store them on a micro-SD card

<details>
  <summary>Development environment setup</summary>
  
1. [Install](https://github.com/esp-rs/rust-build) Rust and Xtensa build tools
    - Make sure to `sudo chmod +x export-esp.sh`
2. Start a project using the [esp-idf-template](https://github.com/esp-rs/esp-idf-template) from the private repo home `dir`. I chose all the default options
```
# STD Project
cargo generate https://github.com/esp-rs/esp-idf-template cargo
# NO-STD (Bare-metal) Project
cargo generate https://github.com/esp-rs/esp-template
```
3. Build the `Hello World` program by running `cargo build` in the new project dir. This will take a while to build the first time:
```
cd esp32-rust
cargo build
...
Finished dev [optimized + debuginfo] target(s) in 6m 40s
```
4. Flash the ESP32 with the build artifact:
```
espflash /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/project
```
5. Connect to ESP32 and monitor
```
espmonitor /dev/ttyACM0
```
 </details>

<details>
  <summary>Roadmap</summary>
  
- Logging (https://github.com/knurling-rs/defmt)
- Debug project (https://github.com/knurling-rs/probe-run)
- MQTT transfer
- Pub/sub (https://github.com/jakmeier/nuts)
- Timer to generate blinky
- DMA
- OTA
- CLI
- UART
- Crash dumps and diagnostics

</details>


<details>
  <summary>Misc</summary>
  
[Singletons in Embedded Rust](https://docs.rust-embedded.org/book/peripherals/singletons.html)

Pull in code for submodules with:
```
git submodule update --init --recursive
```
</details>


<details>
  <summary>Links</summary>

- [150+ ESP32 project](https://microcontrollerslab.com/esp32-tutorials-projects/)
- [Wokwi ESP32 Rust](https://wokwi.com/rust)
- [ESP32 Tutorials](https://embeddedexplorer.com/esp32/)
- [160+ ESP32 Projects, Tutorials, and Guides](https://randomnerdtutorials.com/projects-esp32/)

