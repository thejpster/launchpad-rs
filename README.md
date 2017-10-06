# Rust on the Stellaris Launchpad

A [TockOS](https://tockos.org) compatible bootloader written in Rust (https://rust-lang.org) for the Stellaris Launchpad (LM4F120 dev board). May also work on the very closely related Tiva-C TM4C120 Launchpad. Based on the [Stellaris Launchpad](https://github.com/thejpster/stellaris-launchpad) starter project.

## Requirements

* rustc nightly
* xargo
* arm-none-eabi-gcc
* arm-none-eabi-ar
* arm-none-eabi-objcopy

## Geting set up

```bash
cargo install xargo
rustup install nightly
git clone https://github.com/thejpster/stellaris-launchpad-bootloader.git
cd ./stellaris-launchpad-bootloader
rustup override set nightly
rustup component add rust-src
```

## Compile and upload

```bash
xargo build --example bootloader
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/examples/bootloader target/thumbv7em-none-eabihf/debug/examples/bootloader.bin
sudo lm4flash target/thumbv7em-none-eabihf/debug/examples/bootloader.bin
```

## You can also debug

```
~/stellaris-launchpad-bootloader $ sudo openocd -f /usr/share/openocd/scripts/board/ek-lm4f120xl.cfg
~/stellaris-launchpad-bootloader $ arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/examples/bootloader
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
* You can flash binaries using [Tockloader](https://www.pypi.org/project/tockloader).
