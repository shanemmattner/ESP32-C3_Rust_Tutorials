# ESP32-C3 Rust Tutorials

This will be a series of embedded Rust tutorials using the [Olimex ESP32-C3 Dev kit](https://www.olimex.com/Products/IoT/ESP32-C3/ESP32-C3-DevKit-Lipo/open-source-hardware). 

The structure of these tutorials is part classic tutorials and part project based. The project is an `ESP32-C3 Remote Data Logger` for which we will need several different peripherals.  I'll go through short tutorials from scratch on implementing the peripherals.  Then in a later chapter we'll combine all the peripherals into a cohesive project.




## Notes:
- `submodules/reference_projects` contains a bunch of promising ESP32 Rust projects.  Since there's a lack of tutorials out on the web today I scoured Github for any project using `esp-idf-hal`.
- These tutorials are all using `std`.  Eventually I'll probably make some `no-std` examples but I'm still learning Rust so `std` is easier

## Tutorials
Each part is self contained
- `0 - Output`: We'll make the "Hellow World" of embedded: a Blinky application to turn an LED on and off periodically
<img src="./pics/blinky.gif" width="250" height="250"/>

- `1 - Input`: Add a button to turn the blinking logic on and off
<img src="./pics/blinky-btn.gif" width="250" height="250"/>

- `2 - Threads`: Move the button logic and LED logic to their own thread and pass messages between threads with [crossbeams channel](https://docs.rs/crossbeam/latest/crossbeam/channel/index.html)
- `3 - ADC`: Read 4 different ADC's and print out their values
- `4 - neopixel`: Use an ADC reading to change the color of a [neopixel](https://www.adafruit.com/category/168) 
- `5 - I2C`: Use I2C to talk to a SX1509 GPIO Expander. Configure some pins as inputs and others as outputs.
- `6 - SPI`: Read ADC values and store them on a micro-SD card

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
- uSD card
- Debug project (https://github.com/knurling-rs/probe-run)
- MQTT transfer
- Pub/sub (https://github.com/jakmeier/nuts)
- Timer to generate blinky
- DMA
- SPI
- OTA
- I2C
- CLI
- UART
- Crash dumps and diagnostics


Other interesting crates/ideas:
- [static assertions](https://github.com/nvzqz/static-assertions-rs)
- [lazy static](https://github.com/rust-lang-nursery/lazy-static.rs)
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

