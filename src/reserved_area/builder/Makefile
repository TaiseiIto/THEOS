BUILDER = builder
BUILDER_SOURCE = $(addsuffix .cpp, $(BUILDER))

COMPILER = g++
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

TARGET = reserved_area.bin
SOURCE = $(addsuffix .cpp, $(TARGET))

all: $(TARGET)

clean:
	../../../clean.sh $(BUILDER) $(TARGET)

rebuild: clean
	make

$(BUILDER): $(BUILDER_SOURCE)
	$(COMPILER) $^ $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

$(TARGET): $(BUILDER)
	./$(BUILDER) ../boot_sector.bin ../fsinfo_sector.bin $@

.PHONY: all clean rebuild

