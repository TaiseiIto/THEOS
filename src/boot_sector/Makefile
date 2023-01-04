NAME = boot_sector
MAP = $(NAME).map
OBJECT = $(NAME).o
SCRIPT = $(NAME).ld
SOURCE = $(NAME).s
TARGET = $(NAME).bin

all: $(TARGET)

$(TARGET): $(OBJECT) $(SCRIPT)
	ld $(OBJECT) -Map $(MAP) -o $@ -T $(SCRIPT)

$(OBJECT): $(SOURCE)
	gcc $^ -c -nostdlib -o $@ -Wall -Wextra

