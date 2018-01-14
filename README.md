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
git clone https://github.com/thejpster/stellaris-launchpad.git
cd ./stellaris-launchpad
git checkout bootloader
rustup override set nightly
rustup component add rust-src
```

## Compile and upload

```bash
make # calls xargo build, then arm-none-eabi-objcopy
sudo lm4flash target/thumbv7em-none-eabihf/debug/bootloader.bin
```

## You can also debug

```
~/stellaris-launchpad-bootloader $ sudo openocd -f /usr/share/openocd/scripts/board/ek-lm4f120xl.cfg
~/stellaris-launchpad-bootloader $ arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/bootloader
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

## You can play with Tockloader

```
$ python3 -mvenv ./env
$ ./env/bin/pip install tockloader
$ ./env/bin/tockloader dump-flash-page 0
No device name specified. Using default "tock"
No serial port with device name "tock" found
Found 1 serial port(s).
Using "/dev/ttyACM0 - In-Circuit Debug Interface"

Getting page of flash...
Page number: 0 (0x000000)
00000000  00 80 00 20 97 14 00 00  71 15 00 00 59 15 00 00  |... ....q...Y...|
00000010  73 15 00 00 7d 15 00 00  87 15 00 00 00 00 00 00  |s...}...........|
...
000001e0  9b 15 00 00 9b 15 00 00  9b 15 00 00 00 00 00 00  |................|
000001f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
Finished in 1.262 seconds

$ ./env/bin/tockloader list-attributes
No device name specified. Using default "tock"
No serial port with device name "tock" found
Found 1 serial port(s).
Using "/dev/ttyACM0 - In-Circuit Debug Interface"

Listing attributes...
00:    board = stellaris launchpad
01:     arch = cortex-m4
02: jldevice = LM4F120H5QR
03:          = 
04:          = 
05:          = 
06:          = 
07:          = 
08:          = 
09:          = 
10:          = 
11:          = 
12:          = 
13:          = 
14:          = 
15:          = 
Finished in 1.325 seconds
```

## Flash memory map

The LM4F120 has 256 KiB of Flash and 32 KiB of SRAM. The flash
starts at address 0x0000_0000 and the SRAM starts at address
0x2000_0000.


```
+----------+------+-------------+---------------------------+
|          |      |             |                           |
|Interrupts| Info |Bootloader   | Application               |
|          |      |             |                           |
+----------+------+-----------------------------------------+
0x0        0x400  0x980         0x10000
```

The interrupt table takes up only the first 156 32-bit words or so. The Info
block starts at address 1024 (0x400) and contains the various TockOS
attributes like version numbers and board info. The bootloader follows the
info block and ends at 64 KiB (0x10000), leaving the remaining 192 KiB for
applications.

## What works:

* UART works, using the on-board UART-to-USB bridge (115200 bps, 8N1)
* PLL runs at 66.7MHz
* SysTick works at 4MHz, providing a timer a currently use for the busy-waits
* GPIO works - you can control the on-board RGB LED
* Timer works - you can drive GPIOs (including the LED) with PWM
* Panic handler works - it quickly flashes the red LED if it panics or hits a hardfault
* You can flash binaries using [Tockloader](https://www.pypi.org/project/tockloader).
