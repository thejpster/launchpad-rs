.PHONY = all rebuild build clean examples

EXAMPLES_RS = $(wildcard examples/*.rs)
EXAMPLES = $(EXAMPLES_RS:.rs=)
TARGET_ROOT = target/thumbv7em-none-eabihf
DEBUG_ROOT = $(TARGET_ROOT)/debug
RELEASE_ROOT = $(TARGET_ROOT)/release
DEBUG_ELF = $(EXAMPLES:%=$(DEBUG_ROOT)/%)
RELEASE_ELF = $(EXAMPLES:%=$(RELEASE_ROOT)/%)
DEBUG_BIN = $(DEBUG_ELF:%=%.bin)
RELEASE_BIN = $(RELEASE_ELF:%=%.bin)

all: build

rebuild: clean build

build: $(DEBUG_BIN) $(RELEASE_BIN)

clean:
	cargo clean

$(DEBUG_BIN) $(RELEASE_BIN): %.bin: %
	arm-none-eabi-size -x $<
	arm-none-eabi-objcopy -O binary $< $@

$(TARGET_ROOT)/debug/%: FORCE
	xargo build --example $(basename $(notdir $@))

$(TARGET_ROOT)/release/%: FORCE
	xargo build --release --example $(basename $(notdir $@))

FORCE:
