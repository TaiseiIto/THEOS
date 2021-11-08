CLEANER = ../../../script/clean.sh

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

VOLUME_SERIAL_NUMBER = volume_serial_number.s
VOLUME_SERIAL_NUMBER_GENERATOR = $(patsubst %.s, %.sh, $(VOLUME_SERIAL_NUMBER))

all: $(TARGETS)

boot_sector.bin: boot_sector.o
	$(LINKER) $^ $(LINKER_MAP_OPTION) $(@:.bin=.map) $(LINKER_OUTPUT_OPTION) $@ $(LINKER_SCRIPT_OPTION) $(@:.bin=.ld)

boot_sector.o: boot_sector.s $(VOLUME_SERIAL_NUMBER)
	$(COMPILER) $< $(COMPILER_DONT_LINK_OPTION) $(COMPILER_DONT_USE_STDLIB_OPTION) $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

clean:
	$(CLEANER) *.bin *.map *.o volume_serial_number.s

rebuild: clean
	make

$(VOLUME_SERIAL_NUMBER): $(VOLUME_SERIAL_NUMBER_GENERATOR)
	./$<

.PHONY: all clean rebuild

.PRECIOUS: %.o %.map volume_serial_number.s

