BOOT_SECTOR = ../boot_sector.bin

BUILDER = builder
BUILDER_SOURCE = $(addsuffix .cpp, $(BUILDER)) boot_sector.cpp
BUILDER_OBJECTS = $(patsubst %.cpp, %.o, $(BUILDER_SOURCE))

CLEANER = ../../../script/clean.sh

COMPILER = g++
COMPILER_DONT_LINK_OPTION = -c
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

FSINFO_SECTOR = ../fsinfo_sector.bin

LINKER = g++
LINKER_OUTPUT_OPTION = -o

TARGET = reserved_area.bin
SOURCE = $(addsuffix .cpp, $(TARGET))

all: $(TARGET)

clean:
	$(CLEANER) $(BUILDER) $(TARGET)

rebuild: clean
	make

$(BUILDER): $(BUILDER_OBJECTS)
	$(LINKER) $^ $(LINKER_OUTPUT_OPTION) $@

$(BUILDER_OBJECTS): $(@:.o=.cpp)

%.o: %.cpp
	$(COMPILER) $^ $(COMPILER_DONT_LINK_OPTION) $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

$(TARGET): $(BUILDER)
	./$(BUILDER) $(BOOT_SECTOR) $(FSINFO_SECTOR) $@

.PHONY: all clean rebuild

