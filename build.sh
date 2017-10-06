#!/bin/bash
set -e

#
# launchpad build script
#
# Copyright (c) 2016 Jonathan 'theJPster' Pallant <github@thejpster.org.uk>
#

for EXAMPLE_RS in ./examples/*.rs; do

    EXAMPLE=`basename ${EXAMPLE_RS/.rs/}`
    DEBUG_PATH=./target/thumbv7em-none-eabihf/debug/examples/${EXAMPLE}
    RELEASE_PATH=${DEBUG_PATH/debug/release}


    echo "Building ${EXAMPLE}..."
    xargo build $@ --example ${EXAMPLE}
    xargo build $@ --release --example ${EXAMPLE}
    arm-none-eabi-size -B -x ${DEBUG_PATH}
    arm-none-eabi-size -B -x ${RELEASE_PATH}


    echo "Converting elf -> bin..."
    arm-none-eabi-objcopy -O binary ${DEBUG_PATH} ${DEBUG_PATH}.bin
    arm-none-eabi-objcopy -O binary ${RELEASE_PATH} ${RELEASE_PATH}.bin

done

echo "Examples available..."
ls -lh ./target/thumbv7em-none-eabihf/*/examples/*

echo "Done!"
