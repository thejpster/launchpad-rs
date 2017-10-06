#!/bin/bash
set -e

#
# launchpad build script
#
# Copyright (c) 2016 Jonathan 'theJPster' Pallant <github@thejpster.org.uk>
#

DEBUG_PATH=./target/thumbv7em-none-eabihf/debug/bootloader
RELEASE_PATH=${DEBUG_PATH/debug/release}

echo "Building ${EXAMPLE}..."
xargo build $@
xargo build $@ --release
arm-none-eabi-size -B -x ${DEBUG_PATH}
arm-none-eabi-size -B -x ${RELEASE_PATH}

echo "Converting elf -> bin..."
arm-none-eabi-objcopy -O binary ${DEBUG_PATH} ${DEBUG_PATH}.bin
arm-none-eabi-objcopy -O binary ${RELEASE_PATH} ${RELEASE_PATH}.bin

echo "Done!"
