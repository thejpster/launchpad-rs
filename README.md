# Rust on the Stellaris Launchpad

A bare metal example program written in Rust (https://rust-lang.org) for the Stellaris Launchpad (LM4F120 dev board). May also work on the very closely related Tiva-C TM4C123 Launchpad.

## Requirements

* rustc stable
* arm-none-eabi-gcc
* arm-none-eabi-ar
* arm-none-eabi-objcopy

## Geting set up

```bash
git clone https://github.com/thejpster/launchpad-rs.git
cd ./launchpad-rs
rustup component add rust-src
rustup target add thumbv7em-none-eabihf
```

or simply

```bash
git clone https://github.com/thejpster/launchpad-rs.git
cd ./launchpad-rs
make prerequisites
```

## Compile and upload

```bash
cargo build --example launchpad_blink --release
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/examples/launchpad_blink target/thumbv7em-none-eabihf/release/examples/launchpad_blink.bin
sudo lm4flash target/thumbv7em-none-eabihf/release/examples/launchpad_blink.bin
```

## You can also debug

```
~/launchpad-rs $ cargo build --example launchpad_blink
~/launchpad-rs $ openocd -f /usr/share/openocd/scripts/board/ek-lm4f120xl.cfg
<open a new terminal, leaving OpenOCD running...>
~/launchpad-rs $ arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/examples/launchpad_blink
(gdb) target remote localhost:3333
(gdb) load
Loading section .text, size 0x1e98 lma 0x0
Loading section .ARM.exidx, size 0x8 lma 0x1e98
Loading section .data, size 0xc lma 0x1ea0
Start address 0x0, load size 7852
Transfer rate: 7 KB/sec, 2617 bytes/write.
(gdb) monitor reset halt
(gdb) break main
(gdb) continue
```

## What works:

* UART works, using the on-board UART-to-USB bridge (115200 bps, 8N1)
* PLL runs at 66.7MHz
* SysTick works at 4MHz, providing a timer a currently use for the busy-waits
* GPIO works - you can control the on-board RGB LED
* Timer works - you can drive GPIOs (including the LED) with PWM
* Panic handler works - it quickly flashes the red LED if it panics or hits a hardfault

## License

Licensed under the MIT license ([LICENSE](../LICENSE) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
