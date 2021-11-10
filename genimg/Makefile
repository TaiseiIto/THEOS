BOOT_SECTOR = ../src/reserved_area/boot_sector.bin

CLEANER = ../script/clean.sh

COMPILER = g++
COMPILER_DONT_LINK_OPTION = -c
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

FSINFO_SECTOR = ../src/reserved_area/fsinfo_sector.bin

GENIMG = genimg
GENIMG_SOURCE = $(wildcard *.cpp)
GENIMG_OBJECTS = $(patsubst %.cpp, %.o, $(GENIMG_SOURCE))

LINKER = g++
LINKER_OUTPUT_OPTION = -o

MAKE = make
MAKE_OUT = makeout.txt

TARGET = theos.img

all: $(TARGET)

clean:
	$(CLEANER) $(GENIMG) $(TARGET) $(GENIMG_OBJECTS)

rebuild: clean
	$(MAKE) 2>&1 | tee $(MAKE_OUT)

$(GENIMG): $(GENIMG_OBJECTS)
	$(LINKER) $^ $(LINKER_OUTPUT_OPTION) $@

$(GENIMG_OBJECTS): $(@:.o=.cpp)

%.o: %.cpp
	$(COMPILER) $^ $(COMPILER_DONT_LINK_OPTION) $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

$(TARGET): $(GENIMG)
	./$(GENIMG) $(BOOT_SECTOR) $(FSINFO_SECTOR) $@

.PHONY: all clean rebuild

