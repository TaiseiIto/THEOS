COMPILER = g++
COMPILER_OUTPUT_OPTION = -o
COMPILER_WARNING_OPTION = -Wall -Wextra

TARGET = builder
SOURCE = $(addsuffix .cpp, $(TARGET))

all: $(TARGET)

clean:
	../../../clean.sh $(TARGET)

rebuild: clean
	make

$(TARGET): $(SOURCE)
	$(COMPILER) $^ $(COMPILER_OUTPUT_OPTION) $@ $(COMPILER_WARNING_OPTION)

.PHONY: all clean rebuild

