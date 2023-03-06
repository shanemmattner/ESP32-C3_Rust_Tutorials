# ESP32-C3 Rust Tutorials

Various `embedded Rust tutorials` using the [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3). We'll work towards making a `remote data logger` by first implementing peripherals we will need to build the larger project.

Each tutorial below has a `YouTube video` link where I will start with a blank project and implement a peripherals using the latest [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) version. 

## Repo organization

- [P0-Peripheral-Tutorials](https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/P0-Peripheral-Tutorials): Short tutorials on using various peripherals. This is the code for the [YouTube playlist](https://youtube.com/playlist?list=PLkch9g9DEE0Lkm1LqcD7pZNDmXEczOo-a)
- [P0-Project-Tutorials](https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/P0-Project-Tutorials): Longer tutorials implementing some feature that will go into our data logger project
- [WIP](https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/WIP): Half-baked crates that may have some useful code in the future
- [docs](https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/docs): Various datasheets, books, and technical manuals
- [submodules](https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/docs): All of the submodules for this repository.
   - <b>[referecnce projects](https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/submodules/reference_projects)</b>: A huge collection of embedded Rust projects that I copy code from
- [Data-Logger-PCB](https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/Data-Logger-PCB): KiCAD hardware designs for ESP32-C3 data logger PCB



## Project P0: Remote Data Logger MVP
[Minimum viable product](https://en.wikipedia.org/wiki/Minimum_viable_product) project where we'll implement all the peripherals and features we need for the data logger
- 4 ADC's
- 16 Digital I/O
- DHT11
- SD card logging
- UART CLI Shell

### Peripheral Tutorials
- [p0-output](https://youtu.be/vUSHaogHs1s): Make the "Hellow World" of embedded systems: a `Blinky application` to periodically turn an LED on and off
- [p1-input](https://youtu.be/2IY27b9TT2k): Add a button to turn the blinking logic on and off
- [p2-threads](https://youtu.be/ht5t39dEa4E): Move the button logic and LED logic to their own thread and pass messages between threads with [crossbeams channel](https://docs.rs/crossbeam/latest/crossbeam/channel/index.html)
- [p3-adc](https://youtu.be/07SVj6R_AhA): Read an [analog-to-digital converter channel](https://en.wikipedia.org/wiki/Analog-to-digital_converter) and print out the value
- [p4-dht11](tbd): Read a [DHT11](https://www.mouser.com/datasheet/2/758/DHT11-Technical-Data-Sheet-Translated-Version-1143054.pdf) for temperature and humidity data  
- [p5-i2c](https://youtu.be/NvEnHJPpavo): Configure and use a [SX1509 GPIO Expander](https://www.sparkfun.com/products/13601) through [I2C](https://learn.sparkfun.com/tutorials/i2c)
- [p6-spi](https://youtu.be/PUL8ehH6eUg): Write a string to a uSD card over SPI using [embedded-sdmmc](https://github.com/rust-embedded-community/embedded-sdmmc-rs) crate
- [p7-uart](https://youtu.be/-xEivxWe29M): Receiver characters from the UART and send them back when we detect a [carriage return](https://developer.mozilla.org/en-US/docs/Glossary/CRLF)

### Project Tutorials
- [p8-cli-shell](): Create a simple [CLI shell](https://interrupt.memfault.com/blog/firmware-shell) to interact with the device
- [p9-adc-stream](): Configure ADC's to continuously read and make that data available to other parts of the application through a mutex
- [p10-log-values](): Log all the values read from the ADC's
- [p11-digital-stream(): Continuously read the SX1509 inputs and log to SD card

## Project P1: 
For the next part of this project (`P1`) we will add in features listed below:
- Debugging
- Logging
- FSM/HSM
- Wifi
- MQTT
- OTA
- Pub/sub
- Crash dumps & diagnostics

Other features I'd like to add but don't have a clear example for yet:
- DMA
- Timer usage

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

