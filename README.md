# Paper train

Displays NMBS train delays on an e-ink display, driven by an esp32, in rust.

## Development

This thing uses <https://github.com/esp-rs/rust-build>

Set it up:

```sh
cargo install espup cargo-espflash ldproxy
espup install
```

## Hardware

- [WaveShare 7.5" e-paper display](https://www.waveshare.com/wiki/7.5inch_e-Paper_HAT_(B)_Manual)
- [WaveShare e-Paper Driver HAT](https://www.waveshare.com/wiki/E-Paper_Driver_HAT#Rev2.2)
- [Esp32-S3-DevKitC-1](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/hw-reference/esp32s3/user-guide-devkitc-1.html)

### Connections

| driver hat    | esp32 devkit |
|---------------|--------------|
| BUSY (purple) | 18           |
| RST (white)   | 16           |
| DC (green)    | 17           |
| CS (orange)   | 10           |
| CLK (yellow)  | 12           |
| DIN (blue)    | 11           |
| GND (brown)   | G            |
| VCC (gray)    | 3.3V         |
