CLEANER = ../../script/clean.sh

COMPILER = gcc
COMPILER_DONT_LINK_OPTION = -c
COMPILER_DONT_USE_STDLIB_OPTION = -nostdlib
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

LINKER = ld
LINKER_MAP_OPTION = -Map
LINKER_OUTPUT_OPTION = -o
LINKER_SCRIPT_OPTION = -T

MAKE = make
MAKE_OUT = makeout.txt

TARGET = fsinfo_sector.bin

all: $(TARGET)

clean:
	$(CLEANER) *.bin *.map *.o $(MAKE_OUT)

fsinfo_sector.bin: fsinfo_sector.o
	$(LINKER) $^ $(LINKER_MAP_OPTION) $(@:.bin=.map) $(LINKER_OUTPUT_OPTION) $@ $(LINKER_SCRIPT_OPTION) $(@:.bin=.ld)

fsinfo_sector.o: fsinfo_sector.s
	$(COMPILER) $< $(COMPILER_DONT_LINK_OPTION) $(COMPILER_DONT_USE_STDLIB_OPTION) $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

rebuild: clean
	$(MAKE) 2>&1 | tee $(MAKE_OUT)

.PHONY: all clean rebuild

.PRECIOUS: %.o %.map

