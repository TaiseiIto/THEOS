COMPILER = gcc
COMPILER_DONT_LINK_OPTION = -c
COMPILER_DONT_USE_STDLIB_OPTION = -nostdlib
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

LINKER = ld
LINKER_MAP_OPTION = -Map
LINKER_OUTPUT_OPTION = -o
LINKER_SCRIPT_OPTION = -T

TARGETS = boot_sector.bin

all: $(TARGETS)

boot_sector.bin: boot_sector.o
	$(LINKER) $^ $(LINKER_MAP_OPTION) $(@:.bin=.map) $(LINKER_OUTPUT_OPTION) $@ $(LINKER_SCRIPT_OPTION) $(@:.bin=.ld)

boot_sector.o: boot_sector.s volume_seroal_number.s
	$(COMPILER) $< $(COMPILER_DONT_LINK_OPTION) $(COMPILER_DONT_USE_STDLIB_OPTION) $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

clean:
	./clean.sh *.bin *.map *.o volume_serial_number.s

volume_serial_number.s: volume_serial_number.sh
	./volume_serial_number.sh

.PHONY: all clean

.PRECIOUS: %.o %.map volume_serial_number.s

