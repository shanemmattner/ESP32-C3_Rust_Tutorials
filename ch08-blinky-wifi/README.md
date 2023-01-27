```
export RUST_ESP32_STD_DEMO_WIFI_SSID='yourSSID'
export RUST_ESP32_STD_DEMO_WIFI_PASS='yourPASS'
cargo build
espflash /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/ch04-blinky-wifi
espmonitor /dev/ttyACM0
```