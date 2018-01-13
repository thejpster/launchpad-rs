.PHONY = all rebuild build clean

DEBUG_ELF = ./target/thumbv7em-none-eabihf/debug/bootloader
RELEASE_ELF = ./target/thumbv7em-none-eabihf/release/bootloader
DEBUG_BIN = $(DEBUG_ELF).bin
RELEASE_BIN = $(RELEASE_ELF).bin

all: build

rebuild: clean build

build: $(DEBUG_BIN) $(RELEASE_BIN)

clean:
	cargo clean

$(DEBUG_BIN) $(RELEASE_BIN): %.bin: %
	arm-none-eabi-size -x $<
	arm-none-eabi-objcopy -O binary $< $@

$(DEBUG_ELF): FORCE
	xargo build

$(RELEASE_ELF): FORCE
	xargo build --release

FORCE:
