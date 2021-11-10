BOOT_SECTOR = boot_sector/boot_sector.bin

CLEANER = ../script/clean.sh

COMPILER = g++
COMPILER_DONT_LINK_OPTION = -c
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

FSINFO_SECTOR = fsinfo_sector/fsinfo_sector.bin

GENIMG = ./genimg
GENIMG_SOURCE = $(wildcard *.cpp)
GENIMG_OBJECTS = $(patsubst %.cpp, %.o, $(GENIMG_SOURCE))

LINKER = g++
LINKER_OUTPUT_OPTION = -o

MAKE = make
MAKE_OUT = makeout.txt

TARGET = theos.img

all: $(TARGET)

clean:
	$(CLEANER) $(GENIMG) $(TARGET) $(GENIMG_OBJECTS) $(MAKE_OUT)
	$(MAKE) $@ -C $(dir $(BOOT_SECTOR))
	$(MAKE) $@ -C $(dir $(FSINFO_SECTOR))

rebuild: clean
	$(MAKE) 2>&1 | tee $(MAKE_OUT)

$(GENIMG): $(GENIMG_OBJECTS)
	$(LINKER) $^ $(LINKER_OUTPUT_OPTION) $@

$(GENIMG_OBJECTS): $(@:.o=.cpp)

%.o: %.cpp
	$(COMPILER) $^ $(COMPILER_DONT_LINK_OPTION) $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

$(BOOT_SECTOR):
	$(MAKE) -C $(dir $@)

$(FSINFO_SECTOR):
	$(MAKE) -C $(dir $@)

$(TARGET): $(GENIMG) $(BOOT_SECTOR) $(FSINFO_SECTOR)
	$(GENIMG) $(BOOT_SECTOR) $(FSINFO_SECTOR) $@

.PHONY: all clean rebuild

