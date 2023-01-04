NAME = boot_sector
MAP = $(NAME).map
OBJECT = $(NAME).o
SCRIPT = $(NAME).ld
SOURCE = $(NAME).s
TARGET = $(NAME).bin

all: $(TARGET)

clean:
	rm -f $(MAP) $(OBJECT) $(TARGET)

rebuild: clean
	make

$(TARGET): $(OBJECT) $(SCRIPT)
	ld $(OBJECT) -Map $(MAP) -o $@ -T $(SCRIPT)

$(OBJECT): $(SOURCE)
	as $^ -c -nostdlib -o $@

