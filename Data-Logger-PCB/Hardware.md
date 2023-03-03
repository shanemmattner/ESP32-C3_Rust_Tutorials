# Hardware

This document is for the hardware portion of the project.

## Hardware Design Log
1. Import ESP32-C3-Mini-1 symbol and footprint from [Espressif KiCad Library](https://github.com/espressif/kicad-libraries)
2. Place `ESP32-CE-Mini-1` module with decoupling/bulking caps
3. Create `enable` circuit for ESP32C3
4. Create programming circuit for ESP32C3
    - Only need `USB D+/D-` pins `18/19` to program the ESP32-C3 because it has a `integrated USB Serial/JTAG Controller`, see [doc](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-guides/usb-serial-jtag-console.html)
    - [USB C plug vs recepticle](https://www.arrow.com/en/research-and-events/articles/usb-technology-c-plug-and-receptacle-pinouts)
5. Add power circuits
    - Linear LDO will do for the first version
    - Need `0.5A` max
    - Add [PMOS reverse polarity protection circuit](https://components101.com/articles/design-guide-pmos-mosfet-for-reverse-voltage-polarity-protection)
6. ADC circuit
    - Op-amp voltage follower
    - 5 voltage ranges monitored:
        - 2 x `3.3v`
        - 1 x `5v`
        - 1 x `12v`
        - 1 x `24v`
7. uSD card circuit
    - [pull-ups for all unused pins](https://electronics.stackexchange.com/questions/39571/how-to-do-pulling-up-or-down-correctly-when-interfacing-a-microsd-card)


## Convert EasyEDA footprints to KiCAD
JLCPCB is the cheapest board house including assembly.  But you have to use their specific [parts library](https://jlcpcb.com/parts). To easily get footprints for parts you can:
1. Open the footprint in EasyEDA
2. `File` > `Export` > `EasyEDA`
3. Insteall [lc2kicad](https://github.com/RigoLigoRLC/LC2KiCad)
4. Alias the command so it's easier to user
```bash
alias lc2kicad="${Path_to_repo}/submodules/lc2kicad/build/lc2kicad"
```
5. Convert `EasyEDA` footprint to KiCAD
```bash
lc2kicad ~/Downloads/PCB_NEW_PCB_2022-12-04.json
```
6. Open footprint in KiCAD
7. Create new footprint in KiCAD and copy/paste part

https://jlcpcb.com/parts

## TODO
- Page Documentation
- Circuit documenations
- spec specific passives

## Circuits
- ESP32-C3
    - Power bulk cap
    - Power decoupling cap
    - `Enable` circuit
        - `RC` circuit
    - Programming circuit

## Notes
- Use relative path libraries in KiCAD
- Make sure all data sheets are saved
- Add thermistor circuits
- Add I2C power bus reset circuit
- Add decoupling caps near USB trace switching planes
    - [info](https://electronics.stackexchange.com/questions/141264/can-differential-usb-traces-be-routed-relative-to-power-not-ground-planes)