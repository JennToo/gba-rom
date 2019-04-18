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

.PHONY: all clean mgba

all: $(ROM)

$(ROM): $(ELF)
	$(OBJCOPY) -O binary $^ $@
	$(GBAFIX) $@

$(ELF): $(CROSS_DESC) $(RUST_SRCS) $(ASM_OBJS)
	cargo +nightly xbuild --target $(CROSS_DESC)

$(ASM_OBJS): target/asm/%.o: src/asm/%.s | target/asm
	$(AS) $^ -o $@

$(CROSS_DESC): | target
	sed -e s#LINKER#$(LD)#g thumbv4-none-agb.json.in >$@

target target/asm:
	mkdir -p $@

mgba:
	mgba-qt $(ROM)

clean:
	rm -rf target
