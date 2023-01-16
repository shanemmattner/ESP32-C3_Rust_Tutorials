# esp32c3_rust_tutorials

This will be a series of embedded Rust tutorials.  I'm making this repo as a way to record everything I learn so I don't forget it and hopefully it'll help other people.  Embedded Rust is mature enough for a general embedded user, but there's not as many examples out there to follow.


## Steps to set up a new ESP32 Rust project
1. [Install](https://github.com/esp-rs/rust-build) Rust and Xtensa build tools
    - Make sure to `sudo chmod +x export-esp.sh`
2. Start a project using the [esp-idf-template](https://github.com/esp-rs/esp-idf-template) from the private repo home `dir`. I chose all the default options
```
cargo generate --vcs none --git https://github.com/esp-rs/esp-idf-template cargo
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

## Chapter 1 - Blinky
Classic blinky program where we'll dig into the basics of initializing a pin and looping while blining on/off

## Chapter 2 - Blinky thread
We'll take our blinky example and move it into a thread.  

## Chapter 3 - Blinky state machine
Next we'll put the blinky logic into a state machine using the [statig crate](https://github.com/mdeloof/statig).

## Chapter 4 - Blinky + Button Hierarchical State Machine
Add a layer above the blinky that will monitor for a GPIO interrupt and turns the blinky on/off

## Chapter 5 - Timer 
Use a timer to generate blinky

## Chapter 6 - ADC Reading
Read a single ADC which turns the blinky on/off

## Chapter 7 - ADC reader state machine
Make a state machine that will read all of the enabled ADC's at some frequency, and make that data queriable from other parts of the program

## Chapter 8 - DMA transfer
Show how to use DMA transfer

## Chapter 9 - Wi-Fi connection

## Chapter 10 - MQTT transfer

## Chapter 11 - SPI 
Read/write to SD card

## Chapter 12 - I2C
Interract with SX1509 GPIO expander

## Chapter 13 - LED PWM controller

## Chapter 14 - UART CLI

## Chapter 15 - 
