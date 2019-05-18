DEVKIT_ARM_ROOT ?= /opt/devkitpro/devkitARM

AS = $(DEVKIT_ARM_ROOT)/bin/arm-none-eabi-as
LD = $(DEVKIT_ARM_ROOT)/bin/arm-none-eabi-ld
OBJCOPY = $(DEVKIT_ARM_ROOT)/bin/arm-none-eabi-objcopy
GBAFIX = $(DEVKIT_ARM_ROOT)/../tools/bin/gbafix

CROSS_DESC = target/thumbv4-none-agb.json

RUST_SRCS := $(shell find src -type f -name '*.rs')
ASM_SRCS := $(shell find src/asm -type f -name '*.s')
ASM_OBJS := $(ASM_SRCS:src/asm/%.s=target/asm/%.o)

ELF := target/thumbv4-none-agb/debug/gba-rom
ROM := target/gba-rom.gba

FONT_SRC := resources/font/font.png
FONT_BMP := target/font.bmp
FONT_BIN := target/font.bin

.PHONY: all clean mgba

all: $(ROM)

$(ROM): $(ELF)
	$(OBJCOPY) -O binary $^ $@
	$(GBAFIX) $@

$(ELF): $(CROSS_DESC) $(RUST_SRCS) $(ASM_OBJS) $(FONT_BIN)
	cargo +nightly xbuild --target $(CROSS_DESC)

$(ASM_OBJS): target/asm/%.o: src/asm/%.s | target/asm
	$(AS) $^ -o $@

$(CROSS_DESC): | target
	sed -e s#LINKER#$(LD)#g thumbv4-none-agb.json.in >$@

target target/asm:
	mkdir -p $@

mgba: $(ROM)
	mgba-qt $(ROM)

$(FONT_BMP): $(FONT_SRC) | target
	convert $^ -resize 25% $@

$(FONT_BIN): $(FONT_SRC)
	j2-gba-tool gfx-convert bg256c1p $^ $@ $@.pal

clean:
	rm -rf target
