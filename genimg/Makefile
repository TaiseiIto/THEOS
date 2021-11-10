BOOT_SECTOR = ../src/reserved_area/boot_sector.bin

GENIMG = genimg
GENIMG_SOURCE = $(wildcard *.cpp)
GENIMG_OBJECTS = $(patsubst %.cpp, %.o, $(GENIMG_SOURCE))

CLEANER = ../script/clean.sh

COMPILER = g++
COMPILER_DONT_LINK_OPTION = -c
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

FSINFO_SECTOR = ../src/reserved_area/fsinfo_sector.bin

LINKER = g++
LINKER_OUTPUT_OPTION = -o

TARGET = reserved_area.bin

all: $(TARGET)

clean:
	$(CLEANER) $(GENIMG) $(TARGET) $(GENIMG_OBJECTS)

rebuild: clean
	make

$(GENIMG): $(GENIMG_OBJECTS)
	$(LINKER) $^ $(LINKER_OUTPUT_OPTION) $@

$(GENIMG_OBJECTS): $(@:.o=.cpp)

%.o: %.cpp
	$(COMPILER) $^ $(COMPILER_DONT_LINK_OPTION) $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

$(TARGET): $(GENIMG)
	./$(GENIMG) $(BOOT_SECTOR) $(FSINFO_SECTOR) $@

.PHONY: all clean rebuild

