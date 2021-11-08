BOOT_SECTOR = ../boot_sector.bin

BUILDER = builder
BUILDER_SOURCE = $(addsuffix .cpp, $(BUILDER))

CLEANER = ../../../script/clean.sh

COMPILER = g++
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

FSINFO_SECTOR = ../fsinfo_sector.bin

TARGET = reserved_area.bin
SOURCE = $(addsuffix .cpp, $(TARGET))

all: $(TARGET)

clean:
	$(CLEANER) $(BUILDER) $(TARGET)

rebuild: clean
	make

$(BUILDER): $(BUILDER_SOURCE)
	$(COMPILER) $^ $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

$(TARGET): $(BUILDER)
	./$(BUILDER) $(BOOT_SECTOR) $(FSINFO_SECTOR) $@

.PHONY: all clean rebuild

