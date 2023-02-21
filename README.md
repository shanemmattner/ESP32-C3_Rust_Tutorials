# ESP32-C3 Rust Tutorials

This will be a series of embedded Rust tutorials using the [Olimex ESP32-C3 Dev kit](https://www.olimex.com/Products/IoT/ESP32-C3/ESP32-C3-DevKit-Lipo/open-source-hardware). I'm making this repo as a way to record everything I learn so I don't forget it and hopefully others will find it beneficial, too.

## Notes:
- `submodules/reference_projects` contains a bunch of promising ESP32 Rust projects.  Since there's a lack of tutorials out on the web today I scoured Github for any project using `esp-idf-hal`.
- These tutorials are all using `std`.  Eventually I'll probably make some `no-std` examples but I'm still learning Rust so `std` is easier

## Tutorials
The tutorials are meant to be followed in order.  Each chapter consists of a short example that builds off the previous chapter.

- `Chapter 1: Blinky` - The "Hellow World" of embedded.  We'll blink an LED on and off
<img src="./pics/blinky.gif" width="250" height="250"/>

- `Chapter 2: Blinky Thread` - Take the blinky code from the previous chapter and make it run in it's own thread
- `Chapter 3: Blinky Button` - Add a button to turn the blinking logic on and off
<img src="./pics/blinky-btn.gif" width="250" height="250"/>


<b>WIP</b>
- `Chapter 3: Blinky FSM` - Move the blinky logic into a [finite state machine](https://brilliant.org/wiki/finite-state-machines)
- `Chapter 4: Blinky + Button HSM` - Add a button to the previous example which will start and stop the `blinky state machine` in a [hierarchical state machine](https://www.eventhelix.com/design-patterns/hierarchical-state-machine)
- `Chapter 5: Blinky HSM + MPMC` - Create seperate threads for monitoring the button and sending events to the LED FSM. Use a [crossbeams channel](https://docs.rs/crossbeam/latest/crossbeam/channel/index.html) to pass data from the button thread to the `blinky fsm` thread
- `Chapter 6: Blinky HSM + ADC LED PWM` - Take ADC readings and adjust the PWM of the LED we are blinking on/off in previous examples


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
  <summary>TODO</summary>
- Initialize pins with pull-up or pull down 
  - Issue occurs when using '.downgrade_input()'

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
